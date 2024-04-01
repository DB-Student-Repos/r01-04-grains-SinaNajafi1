pub fn square(s: u32) -> u64 {
    if s == 0 || s > 64 {
        panic!("Square must be between 1 and 64")
    }
    let mut x = 1;
        for number_of_grains in 1..=s - 1 {
            x = x * 2;
        }
    x
}
pub fn total() -> u64 {
    (1 .. 65).map(square).sum()
}
