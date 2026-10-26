use hott_theorem_finder::{count_terms_exact_depth, count_terms_up_to_depth};

fn main() {
    const VARIABLES: u32 = 3;
    const MAX_DEPTH: usize = 4;

    println!("three-variable raw term space");
    println!("depth | exact | cumulative");

    for depth in 0..=MAX_DEPTH {
        let exact = count_terms_exact_depth(VARIABLES, depth)
            .expect("count should fit in u128 at displayed depths");
        let cumulative = count_terms_up_to_depth(VARIABLES, depth)
            .expect("count should fit in u128 at displayed depths");

        println!("{depth:>5} | {exact:>17} | {cumulative:>17}");
    }
}
