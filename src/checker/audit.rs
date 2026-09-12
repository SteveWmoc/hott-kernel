use super::declarations::check_module;
use super::{CheckError, ReferenceKind};
use crate::syntax::{Arena, Declaration, Module, Term, TermId};

pub const FEATURE_VOCABULARY: &str = "mltt-core-features/0.1";

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum KernelFeature {
    Empty,
    Identity,
    NaturalNumbers,
    Pi,
    Sigma,
    Unit,
    Universe,
}

impl KernelFeature {
    const ALL: [Self; 7] = [
        Self::Empty,
        Self::Identity,
        Self::NaturalNumbers,
        Self::Pi,
        Self::Sigma,
        Self::Unit,
        Self::Universe,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::Identity => "identity",
            Self::NaturalNumbers => "natural-numbers",
            Self::Pi => "pi",
            Self::Sigma => "sigma",
            Self::Unit => "unit",
            Self::Universe => "universe",
        }
    }

    const fn slot(self) -> usize {
        match self {
            Self::Empty => 0,
            Self::Identity => 1,
            Self::NaturalNumbers => 2,
            Self::Pi => 3,
            Self::Sigma => 4,
            Self::Unit => 5,
            Self::Universe => 6,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AuditDeclarationKind {
    Postulate,
    Transparent,
    Opaque,
}

impl AuditDeclarationKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Postulate => "postulate",
            Self::Transparent => "transparent",
            Self::Opaque => "opaque",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuditDependencies {
    kernel_features: Vec<KernelFeature>,
    extensions: Vec<String>,
    postulates: Vec<usize>,
    declarations: Vec<usize>,
}

impl AuditDependencies {
    pub fn kernel_features(&self) -> &[KernelFeature] {
        &self.kernel_features
    }

    pub fn extensions(&self) -> &[String] {
        &self.extensions
    }

    pub fn postulates(&self) -> &[usize] {
        &self.postulates
    }

    pub fn declarations(&self) -> &[usize] {
        &self.declarations
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclarationAudit {
    index: usize,
    display_name: String,
    kind: AuditDeclarationKind,
    direct: AuditDependencies,
    transitive: AuditDependencies,
}

impl DeclarationAudit {
    pub const fn index(&self) -> usize {
        self.index
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub const fn kind(&self) -> AuditDeclarationKind {
        self.kind
    }

    pub const fn direct(&self) -> &AuditDependencies {
        &self.direct
    }

    pub const fn transitive(&self) -> &AuditDependencies {
        &self.transitive
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FoundationAudit {
    declarations: Vec<DeclarationAudit>,
}

impl FoundationAudit {
    pub const fn feature_vocabulary(&self) -> &'static str {
        FEATURE_VOCABULARY
    }

    pub fn declarations(&self) -> &[DeclarationAudit] {
        &self.declarations
    }
}

/// Check a complete Core v0.1 module and extract its deterministic foundation audit.
///
/// No audit result is produced for a logically invalid module. Extraction scans
/// accepted source syntax only; it never normalizes or unfolds declarations.
pub fn check_and_extract_audit(module: &mut Module) -> Result<FoundationAudit, CheckError> {
    check_module(module)?;
    extract_checked_audit(module)
}

fn extract_checked_audit(module: &Module) -> Result<FoundationAudit, CheckError> {
    let declarations = module.declarations();
    let mut records = Vec::new();
    records
        .try_reserve_exact(declarations.len())
        .map_err(|_| CheckError::resource_exhausted(0))?;

    for (index, declaration) in declarations.iter().enumerate() {
        let direct = scan_direct(module.arena(), declarations, index, declaration)?;
        let transitive = close_transitively(index, &direct, &records)?;
        let display_name = copy_name(index, declaration.name())?;
        let kind = match declaration {
            Declaration::Postulate { .. } => AuditDeclarationKind::Postulate,
            Declaration::Transparent { .. } => AuditDeclarationKind::Transparent,
            Declaration::Opaque { .. } => AuditDeclarationKind::Opaque,
        };
        records.push(DeclarationAudit {
            index,
            display_name,
            kind,
            direct,
            transitive,
        });
    }

    Ok(FoundationAudit {
        declarations: records,
    })
}

fn scan_direct(
    arena: &Arena,
    declarations: &[Declaration],
    declaration_index: usize,
    declaration: &Declaration,
) -> Result<AuditDependencies, CheckError> {
    let mut seen = Vec::new();
    seen.try_reserve_exact(arena.len())
        .map_err(|_| CheckError::resource_exhausted(declaration_index))?;
    seen.resize(arena.len(), false);

    let mut pending = Vec::new();
    let root_count = usize::from(declaration.body().is_some()) + 1;
    pending
        .try_reserve(root_count)
        .map_err(|_| CheckError::resource_exhausted(declaration_index))?;
    if let Some(body) = declaration.body() {
        pending.push(body);
    }
    pending.push(declaration.ty());

    let mut feature_bits = [false; 7];
    let mut declaration_dependencies = Vec::new();

    while let Some(term_id) = pending.pop() {
        if seen[term_id.index()] {
            continue;
        }
        seen[term_id.index()] = true;

        let term = arena.get(term_id).expect("module term-id invariant");
        match term {
            Term::Var(_) => {}
            Term::Global(index) => {
                let dependency = index
                    .to_usize()
                    .filter(|dependency| *dependency < declaration_index)
                    .ok_or_else(|| {
                        CheckError::invalid_reference(
                            declaration_index,
                            term_id,
                            ReferenceKind::Global,
                        )
                    })?;
                push_index(&mut declaration_dependencies, declaration_index, dependency)?;
            }
            Term::Universe(_) => mark(&mut feature_bits, KernelFeature::Universe),
            Term::Pi(a, b) | Term::App(a, b) => {
                mark(&mut feature_bits, KernelFeature::Pi);
                push_terms(&mut pending, declaration_index, &[*a, *b])?;
            }
            Term::Lam(body) => {
                mark(&mut feature_bits, KernelFeature::Pi);
                push_terms(&mut pending, declaration_index, &[*body])?;
            }
            Term::Sigma(a, b) | Term::Pair(a, b) => {
                mark(&mut feature_bits, KernelFeature::Sigma);
                push_terms(&mut pending, declaration_index, &[*a, *b])?;
            }
            Term::Fst(value) | Term::Snd(value) => {
                mark(&mut feature_bits, KernelFeature::Sigma);
                push_terms(&mut pending, declaration_index, &[*value])?;
            }
            Term::Id(a, b, c) => {
                mark(&mut feature_bits, KernelFeature::Identity);
                push_terms(&mut pending, declaration_index, &[*a, *b, *c])?;
            }
            Term::Refl(value) => {
                mark(&mut feature_bits, KernelFeature::Identity);
                push_terms(&mut pending, declaration_index, &[*value])?;
            }
            Term::J(a, b, c, d, e, f) => {
                mark(&mut feature_bits, KernelFeature::Identity);
                push_terms(&mut pending, declaration_index, &[*a, *b, *c, *d, *e, *f])?;
            }
            Term::Empty => mark(&mut feature_bits, KernelFeature::Empty),
            Term::EmptyElim(a, b) => {
                mark(&mut feature_bits, KernelFeature::Empty);
                push_terms(&mut pending, declaration_index, &[*a, *b])?;
            }
            Term::Unit | Term::Star => mark(&mut feature_bits, KernelFeature::Unit),
            Term::UnitElim(a, b, c) => {
                mark(&mut feature_bits, KernelFeature::Unit);
                push_terms(&mut pending, declaration_index, &[*a, *b, *c])?;
            }
            Term::Nat | Term::Zero => mark(&mut feature_bits, KernelFeature::NaturalNumbers),
            Term::Succ(value) => {
                mark(&mut feature_bits, KernelFeature::NaturalNumbers);
                push_terms(&mut pending, declaration_index, &[*value])?;
            }
            Term::NatElim(a, b, c, d) => {
                mark(&mut feature_bits, KernelFeature::NaturalNumbers);
                push_terms(&mut pending, declaration_index, &[*a, *b, *c, *d])?;
            }
            Term::Ann(value, ty) => {
                push_terms(&mut pending, declaration_index, &[*value, *ty])?;
            }
        }
    }

    declaration_dependencies.sort_unstable();
    declaration_dependencies.dedup();

    let mut postulates = Vec::new();
    postulates
        .try_reserve(declaration_dependencies.len())
        .map_err(|_| CheckError::resource_exhausted(declaration_index))?;
    for dependency in &declaration_dependencies {
        if matches!(declarations[*dependency], Declaration::Postulate { .. }) {
            postulates.push(*dependency);
        }
    }

    Ok(AuditDependencies {
        kernel_features: features_from_bits(declaration_index, &feature_bits)?,
        extensions: Vec::new(),
        postulates,
        declarations: declaration_dependencies,
    })
}

fn close_transitively(
    declaration_index: usize,
    direct: &AuditDependencies,
    earlier: &[DeclarationAudit],
) -> Result<AuditDependencies, CheckError> {
    let mut feature_bits = [false; 7];
    for feature in &direct.kernel_features {
        mark(&mut feature_bits, *feature);
    }

    // Declaration indices are dense and every dependency is earlier, so a compact
    // bitset avoids materializing duplicate inherited indices. Visit direct
    // dependencies newest-first: if a dependency is already marked, a newer
    // dependency's transitive closure already contains it and its entire closure.
    let mut declaration_bits = Vec::new();
    declaration_bits
        .try_reserve_exact(declaration_index)
        .map_err(|_| CheckError::resource_exhausted(declaration_index))?;
    declaration_bits.resize(declaration_index, false);

    for dependency in direct.declarations.iter().rev() {
        if declaration_bits[*dependency] {
            continue;
        }
        declaration_bits[*dependency] = true;

        let inherited = &earlier[*dependency].transitive;
        for feature in &inherited.kernel_features {
            mark(&mut feature_bits, *feature);
        }
        for inherited_dependency in &inherited.declarations {
            declaration_bits[*inherited_dependency] = true;
        }
    }

    let declarations = indices_from_bits(declaration_index, &declaration_bits)?;
    let postulate_count = declarations
        .iter()
        .filter(|dependency| matches!(earlier[**dependency].kind, AuditDeclarationKind::Postulate))
        .count();
    let mut postulates = Vec::new();
    postulates
        .try_reserve_exact(postulate_count)
        .map_err(|_| CheckError::resource_exhausted(declaration_index))?;
    for dependency in &declarations {
        if matches!(earlier[*dependency].kind, AuditDeclarationKind::Postulate) {
            postulates.push(*dependency);
        }
    }

    Ok(AuditDependencies {
        kernel_features: features_from_bits(declaration_index, &feature_bits)?,
        extensions: Vec::new(),
        postulates,
        declarations,
    })
}

fn features_from_bits(
    declaration_index: usize,
    bits: &[bool; 7],
) -> Result<Vec<KernelFeature>, CheckError> {
    let count = bits.iter().filter(|used| **used).count();
    let mut features = Vec::new();
    features
        .try_reserve_exact(count)
        .map_err(|_| CheckError::resource_exhausted(declaration_index))?;
    for feature in KernelFeature::ALL {
        if bits[feature.slot()] {
            features.push(feature);
        }
    }
    Ok(features)
}

fn mark(bits: &mut [bool; 7], feature: KernelFeature) {
    bits[feature.slot()] = true;
}

fn push_terms(
    pending: &mut Vec<TermId>,
    declaration_index: usize,
    terms: &[TermId],
) -> Result<(), CheckError> {
    pending
        .try_reserve(terms.len())
        .map_err(|_| CheckError::resource_exhausted(declaration_index))?;
    for term in terms.iter().rev() {
        pending.push(*term);
    }
    Ok(())
}

fn push_index(
    values: &mut Vec<usize>,
    declaration_index: usize,
    value: usize,
) -> Result<(), CheckError> {
    values
        .try_reserve(1)
        .map_err(|_| CheckError::resource_exhausted(declaration_index))?;
    values.push(value);
    Ok(())
}

fn indices_from_bits(declaration_index: usize, bits: &[bool]) -> Result<Vec<usize>, CheckError> {
    let count = bits.iter().filter(|used| **used).count();
    let mut indices = Vec::new();
    indices
        .try_reserve_exact(count)
        .map_err(|_| CheckError::resource_exhausted(declaration_index))?;
    for (index, used) in bits.iter().enumerate() {
        if *used {
            indices.push(index);
        }
    }
    Ok(indices)
}

fn copy_name(declaration_index: usize, source: &str) -> Result<String, CheckError> {
    let mut copy = String::new();
    copy.try_reserve_exact(source.len())
        .map_err(|_| CheckError::resource_exhausted(declaration_index))?;
    copy.push_str(source);
    Ok(copy)
}
