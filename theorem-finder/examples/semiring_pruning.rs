use hott_theorem_finder::{
    CandidateCounts, count_normalized_equations_mod_variable_renaming, count_semiring_normal_forms,
    count_terms_up_to_depth,
};

fn main() {
    const VARIABLES: u32 = 3;

    println!(
        "depth | raw terms | structural equations | normal forms | normalized modulo renaming"
    );

    for depth in 0..=2 {
        let raw_terms = count_terms_up_to_depth(VARIABLES, depth)
            .expect("displayed raw term counts should fit in u128");
        let structural = CandidateCounts::from_term_count(raw_terms)
            .expect("displayed candidate counts should fit in u128")
            .canonical_non_reflexive();
        let normal_forms = count_semiring_normal_forms(VARIABLES, depth);
        let normalized = count_normalized_equations_mod_variable_renaming(VARIABLES, depth);

        println!(
            "{depth:>5} | {raw_terms:>9} | {structural:>20} | {normal_forms:>12} | {normalized:>26}"
        );
    }
}
