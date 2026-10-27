use hott_theorem_finder::count_candidate_stages_up_to_depth;

fn main() {
    const VARIABLES: u32 = 3;
    const MAX_DEPTH: usize = 3;

    println!("depth | ordered | nonreflexive | canonical");

    for depth in 0..=MAX_DEPTH {
        let counts = count_candidate_stages_up_to_depth(VARIABLES, depth)
            .expect("displayed candidate counts should fit in u128");

        println!(
            "{depth:>5} | {:>16} | {:>16} | {:>16}",
            counts.ordered_with_reflexive(),
            counts.ordered_non_reflexive(),
            counts.canonical_non_reflexive()
        );
    }
}
