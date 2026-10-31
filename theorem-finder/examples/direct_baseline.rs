use std::collections::BTreeMap;

use hott_theorem_finder::{
    DirectSearchResult, SemiringAxiom, search_direct_axiom, semiring_identity_candidates,
    terms_up_to_depth,
};

fn main() {
    let terms = terms_up_to_depth(3, 1);
    let candidates = semiring_identity_candidates(&terms);

    let mut proved = 0usize;
    let mut total_attempts = 0usize;
    let mut dependencies: BTreeMap<SemiringAxiom, usize> = BTreeMap::new();

    for (index, candidate) in candidates.iter().enumerate() {
        match search_direct_axiom(&format!("baseline_{index}"), candidate)
            .expect("baseline search should produce only kernel-checked successes")
        {
            DirectSearchResult::Proved(proof) => {
                proved += 1;
                total_attempts += proof.attempts();
                *dependencies.entry(proof.axiom()).or_default() += 1;
            }
            DirectSearchResult::NotProved { attempts } => {
                total_attempts += attempts;
            }
        }
    }

    println!("depth-1 semiring identity classes: {}", candidates.len());
    println!("direct one-axiom proofs: {proved}");
    println!("not proved by baseline: {}", candidates.len() - proved);
    println!("total axiom-schema attempts: {total_attempts}");
    println!("dependency counts:");

    for (axiom, count) in dependencies {
        println!("  {}: {count}", axiom.name());
    }
}
