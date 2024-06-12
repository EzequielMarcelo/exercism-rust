pub fn square(square_number: u32) -> u64 {
    if square_number < 1 || square_number > 64 {
        panic!("Square must be between 1 and 64")
    }

    2_u64.pow(square_number - 1)
}

pub fn total() -> u64 {
    (1..=64).map(|x| 2_u64.pow(x - 1)).sum()
}
