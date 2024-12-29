pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    if factors.is_empty() {
        return 0;
    }

    let multiples: std::collections::HashSet<u32> = factors
        .iter()
        .filter(|&&factor| factor > 0)
        .flat_map(|&factor| (factor..limit).step_by(factor as usize))
        .collect();

    multiples.iter().sum()
}
