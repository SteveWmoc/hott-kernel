use super::{CheckError, ReferenceKind};
use crate::syntax::{Arena, Module, Term, TermId};

/// Validate only local scope and sequential global-reference availability.
///
/// Success does not assert that any declaration is well typed. Type synthesis,
/// checking, conversion, and normalization belong to later checker layers.
pub fn check_references(module: &Module) -> Result<(), CheckError> {
    for (declaration_index, declaration) in module.declarations().iter().enumerate() {
        check_term(module.arena(), declaration_index, declaration.ty())?;
        if let Some(body) = declaration.body() {
            check_term(module.arena(), declaration_index, body)?;
        }
    }
    Ok(())
}

#[derive(Clone, Copy)]
struct Pending {
    term_id: TermId,
    local_depth: usize,
}

fn check_term(arena: &Arena, declaration_index: usize, root: TermId) -> Result<(), CheckError> {
    let mut pending = Vec::new();
    push_pending(&mut pending, declaration_index, root, 0)?;

    while let Some(Pending {
        term_id,
        local_depth,
    }) = pending.pop()
    {
        let term = arena.get(term_id).expect("module term-id invariant");
        match term {
            Term::Var(index) => {
                if index.to_usize().is_none_or(|index| index >= local_depth) {
                    return Err(CheckError::invalid_reference(
                        declaration_index,
                        term_id,
                        ReferenceKind::Local,
                    ));
                }
            }
            Term::Global(index) => {
                if index
                    .to_usize()
                    .is_none_or(|index| index >= declaration_index)
                {
                    return Err(CheckError::invalid_reference(
                        declaration_index,
                        term_id,
                        ReferenceKind::Global,
                    ));
                }
            }
            Term::Pi(domain, codomain) | Term::Sigma(domain, codomain) => {
                let binder_depth = local_depth
                    .checked_add(1)
                    .ok_or(CheckError::resource_exhausted(declaration_index))?;
                reserve_pending(&mut pending, declaration_index, 2)?;
                pending.push(Pending {
                    term_id: *codomain,
                    local_depth: binder_depth,
                });
                pending.push(Pending {
                    term_id: *domain,
                    local_depth,
                });
            }
            Term::Lam(body) => {
                let binder_depth = local_depth
                    .checked_add(1)
                    .ok_or(CheckError::resource_exhausted(declaration_index))?;
                push_pending(&mut pending, declaration_index, *body, binder_depth)?;
            }
            Term::App(function, argument)
            | Term::Pair(function, argument)
            | Term::EmptyElim(function, argument)
            | Term::Ann(function, argument) => {
                push_terms(
                    &mut pending,
                    declaration_index,
                    local_depth,
                    &[*function, *argument],
                )?;
            }
            Term::Fst(value) | Term::Snd(value) | Term::Refl(value) | Term::Succ(value) => {
                push_pending(&mut pending, declaration_index, *value, local_depth)?;
            }
            Term::Id(ty, left, right) | Term::UnitElim(ty, left, right) => {
                push_terms(
                    &mut pending,
                    declaration_index,
                    local_depth,
                    &[*ty, *left, *right],
                )?;
            }
            Term::J(a, b, c, d, e, f) => {
                push_terms(
                    &mut pending,
                    declaration_index,
                    local_depth,
                    &[*a, *b, *c, *d, *e, *f],
                )?;
            }
            Term::NatElim(a, b, c, d) => {
                push_terms(
                    &mut pending,
                    declaration_index,
                    local_depth,
                    &[*a, *b, *c, *d],
                )?;
            }
            Term::Universe(_) | Term::Empty | Term::Unit | Term::Star | Term::Nat | Term::Zero => {}
        }
    }

    Ok(())
}

fn push_terms(
    pending: &mut Vec<Pending>,
    declaration_index: usize,
    local_depth: usize,
    terms: &[TermId],
) -> Result<(), CheckError> {
    reserve_pending(pending, declaration_index, terms.len())?;
    for term_id in terms.iter().rev() {
        pending.push(Pending {
            term_id: *term_id,
            local_depth,
        });
    }
    Ok(())
}

fn push_pending(
    pending: &mut Vec<Pending>,
    declaration_index: usize,
    term_id: TermId,
    local_depth: usize,
) -> Result<(), CheckError> {
    reserve_pending(pending, declaration_index, 1)?;
    pending.push(Pending {
        term_id,
        local_depth,
    });
    Ok(())
}

fn reserve_pending(
    pending: &mut Vec<Pending>,
    declaration_index: usize,
    additional: usize,
) -> Result<(), CheckError> {
    pending
        .try_reserve(additional)
        .map_err(|_| CheckError::resource_exhausted(declaration_index))
}
