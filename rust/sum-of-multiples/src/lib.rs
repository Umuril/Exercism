pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|&num| {
            factors
                .iter()
                .filter(|&&x| x > 0)
                .any(|&factor| num % factor == 0)
        })
        .sum()
}
