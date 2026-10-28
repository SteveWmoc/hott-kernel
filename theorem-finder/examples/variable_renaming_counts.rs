use hott_theorem_finder::{
    count_candidate_stages_up_to_depth, count_equations_mod_variable_renaming,
};

fn main() {
    const VARIABLES: u32 = 3;

    println!("depth | structural canonical | modulo variable renaming");

    for depth in 0..=1 {
        let structural = count_candidate_stages_up_to_depth(VARIABLES, depth)
            .expect("displayed structural counts should fit in u128")
            .canonical_non_reflexive();
        let renamed = count_equations_mod_variable_renaming(VARIABLES, depth);

        println!("{depth:>5} | {structural:>20} | {renamed:>24}");
    }
}
