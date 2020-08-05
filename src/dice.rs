use rand::Rng;

/// Roll a dice of n sides a certain number of times and return the accumulated result.
pub fn roll(n: u32, times: u32) -> u32 {
    if n == 0 || times == 0 {
        return 0;
    }
    let mut total = 0;
    for _ in 0..times {
        total += rand::thread_rng().gen_range(1, n + 1);
    }
    total
}
