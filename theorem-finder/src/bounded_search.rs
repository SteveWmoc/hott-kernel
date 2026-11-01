//! Bounded multi-step rewriting with kernel-checked proof reconstruction.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::{
    BridgeError, DIRECT_AXIOMS, DirectSearchResult, EquationCandidate, SemiringAxiom, Term,
    check_candidate_with_proof,
    kernel_bridge::{
        render_candidate_term, render_open_axiom_application, wrap_candidate_lambdas,
    },
    search::{Pattern, match_pattern, search_direct_axiom},
};

/// Resource bounds for the first multi-step proof search.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RewriteSearchLimits {
    pub max_steps: usize,
    pub max_term_size: usize,
    pub max_term_depth: usize,
    pub max_nodes: usize,
}

impl Default for RewriteSearchLimits {
    fn default() -> Self {
        Self {
            max_steps: 3,
            max_term_size: 7,
            max_term_depth: 3,
            max_nodes: 1_000,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ContextFrame {
    AddLeft { right: Term },
    AddRight { left: Term },
    MulLeft { right: Term },
    MulRight { left: Term },
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RewriteStep {
    source: Term,
    target: Term,
    axiom_source: Term,
    axiom_target: Term,
    axiom: SemiringAxiom,
    arguments: Vec<Term>,
    forward: bool,
    context: Vec<ContextFrame>,
}

fn instantiate_pattern(pattern: &Pattern, substitution: &[Option<Term>]) -> Option<Term> {
    match pattern {
        Pattern::Meta(index) => substitution.get(*index)?.clone(),
        Pattern::Zero => Some(Term::Zero),
        Pattern::One => Some(Term::One),
        Pattern::Add(left, right) => Some(Term::Add(
            Box::new(instantiate_pattern(left, substitution)?),
            Box::new(instantiate_pattern(right, substitution)?),
        )),
        Pattern::Mul(left, right) => Some(Term::Mul(
            Box::new(instantiate_pattern(left, substitution)?),
            Box::new(instantiate_pattern(right, substitution)?),
        )),
    }
}

fn root_rewrites(term: &Term) -> Vec<RewriteStep> {
    let mut rewrites = Vec::new();

    for axiom in DIRECT_AXIOMS {
        let (lhs, rhs, arity) = axiom.specification();

        for forward in [true, false] {
            let (source_pattern, target_pattern) = if forward {
                (&lhs, &rhs)
            } else {
                (&rhs, &lhs)
            };
            let mut substitution = vec![None; arity];

            if !match_pattern(source_pattern, term, &mut substitution) {
                continue;
            }

            let Some(target) = instantiate_pattern(target_pattern, &substitution) else {
                continue;
            };
            let Some(arguments) = substitution.into_iter().collect::<Option<Vec<_>>>() else {
                continue;
            };

            if target == *term {
                continue;
            }

            rewrites.push(RewriteStep {
                source: term.clone(),
                target: target.clone(),
                axiom_source: term.clone(),
                axiom_target: target,
                axiom,
                arguments,
                forward,
                context: Vec::new(),
            });
        }
    }

    rewrites
}

fn within_limits(term: &Term, limits: RewriteSearchLimits) -> bool {
    term.size() <= limits.max_term_size && term.depth() <= limits.max_term_depth
}

fn rewrites_at_any_subterm(term: &Term, limits: RewriteSearchLimits) -> Vec<RewriteStep> {
    let mut by_target = BTreeMap::new();

    for step in root_rewrites(term) {
        if within_limits(&step.target, limits) {
            by_target.entry(step.target.clone()).or_insert(step);
        }
    }

    match term {
        Term::Add(left, right) => {
            for mut step in rewrites_at_any_subterm(left, limits) {
                let source = Term::Add(Box::new(step.source.clone()), right.clone());
                let target = Term::Add(Box::new(step.target.clone()), right.clone());
                if within_limits(&target, limits) {
                    step.source = source;
                    step.target = target.clone();
                    step.context.push(ContextFrame::AddLeft {
                        right: (**right).clone(),
                    });
                    by_target.entry(target).or_insert(step);
                }
            }

            for mut step in rewrites_at_any_subterm(right, limits) {
                let source = Term::Add(left.clone(), Box::new(step.source.clone()));
                let target = Term::Add(left.clone(), Box::new(step.target.clone()));
                if within_limits(&target, limits) {
                    step.source = source;
                    step.target = target.clone();
                    step.context.push(ContextFrame::AddRight {
                        left: (**left).clone(),
                    });
                    by_target.entry(target).or_insert(step);
                }
            }
        }
        Term::Mul(left, right) => {
            for mut step in rewrites_at_any_subterm(left, limits) {
                let source = Term::Mul(Box::new(step.source.clone()), right.clone());
                let target = Term::Mul(Box::new(step.target.clone()), right.clone());
                if within_limits(&target, limits) {
                    step.source = source;
                    step.target = target.clone();
                    step.context.push(ContextFrame::MulLeft {
                        right: (**right).clone(),
                    });
                    by_target.entry(target).or_insert(step);
                }
            }

            for mut step in rewrites_at_any_subterm(right, limits) {
                let source = Term::Mul(left.clone(), Box::new(step.source.clone()));
                let target = Term::Mul(left.clone(), Box::new(step.target.clone()));
                if within_limits(&target, limits) {
                    step.source = source;
                    step.target = target.clone();
                    step.context.push(ContextFrame::MulRight {
                        left: (**left).clone(),
                    });
                    by_target.entry(target).or_insert(step);
                }
            }
        }
        Term::Var(_) | Term::Zero | Term::One => {}
    }

    by_target.into_values().collect()
}

fn render_symmetry(
    candidate: &EquationCandidate,
    source: &Term,
    target: &Term,
    proof: String,
) -> String {
    format!(
        "(app (app (app (ref eq_symm) {}) {}) {proof})",
        render_candidate_term(source, candidate),
        render_candidate_term(target, candidate)
    )
}

fn render_transitivity(
    candidate: &EquationCandidate,
    source: &Term,
    middle: &Term,
    target: &Term,
    first: String,
    second: String,
) -> String {
    format!(
        "(app (app (app (app (app (ref eq_trans) {}) {}) {}) {first}) {second})",
        render_candidate_term(source, candidate),
        render_candidate_term(middle, candidate),
        render_candidate_term(target, candidate)
    )
}

fn render_congruence(
    candidate: &EquationCandidate,
    frame: &ContextFrame,
    source: &Term,
    target: &Term,
    proof: String,
) -> (Term, Term, String) {
    let (outer_source, outer_target, function) = match frame {
        ContextFrame::AddLeft { right } => (
            Term::Add(Box::new(source.clone()), Box::new(right.clone())),
            Term::Add(Box::new(target.clone()), Box::new(right.clone())),
            format!(
                "(lam hole (app (app (ref add) (ref hole)) {}))",
                render_candidate_term(right, candidate)
            ),
        ),
        ContextFrame::AddRight { left } => (
            Term::Add(Box::new(left.clone()), Box::new(source.clone())),
            Term::Add(Box::new(left.clone()), Box::new(target.clone())),
            format!(
                "(lam hole (app (app (ref add) {}) (ref hole)))",
                render_candidate_term(left, candidate)
            ),
        ),
        ContextFrame::MulLeft { right } => (
            Term::Mul(Box::new(source.clone()), Box::new(right.clone())),
            Term::Mul(Box::new(target.clone()), Box::new(right.clone())),
            format!(
                "(lam hole (app (app (ref mul) (ref hole)) {}))",
                render_candidate_term(right, candidate)
            ),
        ),
        ContextFrame::MulRight { left } => (
            Term::Mul(Box::new(left.clone()), Box::new(source.clone())),
            Term::Mul(Box::new(left.clone()), Box::new(target.clone())),
            format!(
                "(lam hole (app (app (ref mul) {}) (ref hole)))",
                render_candidate_term(left, candidate)
            ),
        ),
    };

    let lifted = format!(
        "(app (app (app (app (ref eq_ap) {function}) {}) {}) {proof})",
        render_candidate_term(source, candidate),
        render_candidate_term(target, candidate)
    );

    (outer_source, outer_target, lifted)
}

fn render_step(candidate: &EquationCandidate, step: &RewriteStep) -> String {
    let raw = render_open_axiom_application(candidate, step.axiom.name(), &step.arguments);
    let mut source = step.axiom_source.clone();
    let mut target = step.axiom_target.clone();
    let mut proof = if step.forward {
        raw
    } else {
        render_symmetry(candidate, &target, &source, raw)
    };

    for frame in &step.context {
        let lifted = render_congruence(candidate, frame, &source, &target, proof);
        source = lifted.0;
        target = lifted.1;
        proof = lifted.2;
    }

    debug_assert_eq!(source, step.source);
    debug_assert_eq!(target, step.target);
    proof
}

fn reconstruct_path(
    start: &Term,
    goal: &Term,
    parents: &BTreeMap<Term, (Term, RewriteStep)>,
) -> Vec<RewriteStep> {
    let mut current = goal.clone();
    let mut reversed = Vec::new();

    while current != *start {
        let (previous, step) = parents
            .get(&current)
            .expect("reached search node must have a parent");
        reversed.push(step.clone());
        current = previous.clone();
    }

    reversed.reverse();
    reversed
}

fn render_path(candidate: &EquationCandidate, steps: &[RewriteStep]) -> String {
    let first = steps.first().expect("successful rewrite path is nonempty");
    let mut proof = render_step(candidate, first);
    let source = first.source.clone();
    let mut middle = first.target.clone();

    for step in &steps[1..] {
        debug_assert_eq!(middle, step.source);
        let next_proof = render_step(candidate, step);
        proof = render_transitivity(
            candidate,
            &source,
            &middle,
            &step.target,
            proof,
            next_proof,
        );
        middle = step.target.clone();
    }

    wrap_candidate_lambdas(candidate, proof)
}

/// A kernel-checked proof found by bounded multi-step rewriting.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoundedRewriteProof {
    steps: usize,
    nodes_expanded: usize,
    dependencies: Vec<SemiringAxiom>,
    proof_term: String,
    checked_core_size: usize,
}

impl BoundedRewriteProof {
    #[must_use]
    pub const fn steps(&self) -> usize {
        self.steps
    }

    #[must_use]
    pub const fn nodes_expanded(&self) -> usize {
        self.nodes_expanded
    }

    #[must_use]
    pub fn dependencies(&self) -> &[SemiringAxiom] {
        &self.dependencies
    }

    #[must_use]
    pub fn proof_term(&self) -> &str {
        &self.proof_term
    }

    #[must_use]
    pub const fn checked_core_size(&self) -> usize {
        self.checked_core_size
    }
}

/// Result of the bounded rewrite prover.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BoundedRewriteResult {
    Proved(BoundedRewriteProof),
    NotProved { nodes_expanded: usize },
}

/// Searches for a short equational derivation and kernel-checks every success.
///
/// Direct one-axiom proofs are retained as the baseline fast path. Remaining
/// candidates are explored breadth-first using forward/reverse axiom rewrites
/// at any subterm. Reverse rewrites use equality symmetry, nested rewrites use
/// function congruence, and multi-step paths use equality transitivity.
pub fn search_bounded_rewrite(
    declaration_name: &str,
    candidate: &EquationCandidate,
    limits: RewriteSearchLimits,
) -> Result<BoundedRewriteResult, BridgeError> {
    if let DirectSearchResult::Proved(proof) = search_direct_axiom(declaration_name, candidate)? {
        return Ok(BoundedRewriteResult::Proved(BoundedRewriteProof {
            steps: 1,
            nodes_expanded: 0,
            dependencies: proof.dependencies().to_vec(),
            proof_term: proof.proof_term().to_owned(),
            checked_core_size: proof.checked_core_size(),
        }));
    }

    let start = candidate.lhs().clone();
    let goal = candidate.rhs().clone();
    let mut queue = VecDeque::from([(start.clone(), 0usize)]);
    let mut seen = BTreeSet::from([start.clone()]);
    let mut parents = BTreeMap::new();
    let mut nodes_expanded = 0usize;

    while let Some((current, depth)) = queue.pop_front() {
        if depth >= limits.max_steps || nodes_expanded >= limits.max_nodes {
            continue;
        }

        nodes_expanded += 1;

        for step in rewrites_at_any_subterm(&current, limits) {
            let next = step.target.clone();
            if !seen.insert(next.clone()) {
                continue;
            }

            parents.insert(next.clone(), (current.clone(), step));

            if next == goal {
                let steps = reconstruct_path(&start, &goal, &parents);
                let proof_term = render_path(candidate, &steps);
                let core = check_candidate_with_proof(declaration_name, candidate, &proof_term)?;
                let dependencies: BTreeSet<_> =
                    steps.iter().map(|step| step.axiom).collect();

                return Ok(BoundedRewriteResult::Proved(BoundedRewriteProof {
                    steps: steps.len(),
                    nodes_expanded,
                    dependencies: dependencies.into_iter().collect(),
                    proof_term,
                    checked_core_size: core.len(),
                }));
            }

            queue.push_back((next, depth + 1));
        }
    }

    Ok(BoundedRewriteResult::NotProved { nodes_expanded })
}

#[cfg(test)]
mod tests {
    use super::{
        BoundedRewriteResult, RewriteSearchLimits, search_bounded_rewrite,
    };
    use crate::{EquationCandidate, Term, semiring_identity_candidates, terms_up_to_depth};

    #[test]
    fn proves_left_additive_identity_using_symmetry_and_commutativity() {
        let candidate = EquationCandidate::new(
            Term::Var(0),
            Term::Add(Box::new(Term::Zero), Box::new(Term::Var(0))),
        )
        .unwrap();

        let result =
            search_bounded_rewrite("left_add_zero", &candidate, RewriteSearchLimits::default())
                .unwrap();

        let BoundedRewriteResult::Proved(proof) = result else {
            panic!("bounded rewriting should prove left additive identity");
        };

        assert!(proof.steps() <= 3);
        assert!(proof.checked_core_size() > 0);
    }

    #[test]
    fn depth_one_coverage_rises_from_fifteen_to_thirty_three() {
        let terms = terms_up_to_depth(3, 1);
        let candidates = semiring_identity_candidates(&terms);

        assert_eq!(candidates.len(), 42);

        let proved = candidates
            .iter()
            .enumerate()
            .filter(|(index, candidate)| {
                matches!(
                    search_bounded_rewrite(
                        &format!("rewrite_{index}"),
                        candidate,
                        RewriteSearchLimits::default(),
                    )
                    .unwrap(),
                    BoundedRewriteResult::Proved(_)
                )
            })
            .count();

        assert_eq!(proved, 33);
    }
}
