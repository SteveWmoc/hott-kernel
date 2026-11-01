use std::collections::BTreeMap;

use hott_theorem_finder::{
    BoundedRewriteResult, RewriteSearchLimits, SemiringAxiom, search_bounded_rewrite,
    semiring_identity_candidates, terms_up_to_depth,
};

fn main() {
    let terms = terms_up_to_depth(3, 1);
    let candidates = semiring_identity_candidates(&terms);
    let limits = RewriteSearchLimits::default();

    let mut proved = 0usize;
    let mut direct = 0usize;
    let mut max_steps = 0usize;
    let mut total_nodes = 0usize;
    let mut dependencies: BTreeMap<SemiringAxiom, usize> = BTreeMap::new();

    for (index, candidate) in candidates.iter().enumerate() {
        match search_bounded_rewrite(&format!("rewrite_{index}"), candidate, limits)
            .expect("every reported success must be kernel checked")
        {
            BoundedRewriteResult::Proved(proof) => {
                proved += 1;
                direct += usize::from(proof.steps() == 1);
                max_steps = max_steps.max(proof.steps());
                total_nodes += proof.nodes_expanded();
                for axiom in proof.dependencies() {
                    *dependencies.entry(*axiom).or_default() += 1;
                }
            }
            BoundedRewriteResult::NotProved { nodes_expanded } => {
                total_nodes += nodes_expanded;
            }
        }
    }

    println!("depth-1 semiring identity classes: {}", candidates.len());
    println!("proved within bounds: {proved}");
    println!("one-step baseline proofs: {direct}");
    println!("additional multi-step proofs: {}", proved - direct);
    println!("not proved within bounds: {}", candidates.len() - proved);
    println!("longest successful path: {max_steps} steps");
    println!("total rewrite nodes expanded: {total_nodes}");
    println!("dependency counts:");

    for (axiom, count) in dependencies {
        println!("  {}: {count}", axiom.name());
    }
}
