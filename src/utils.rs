pub fn in_range(value: u32, lower: u32, upper: u32) -> bool {
    value > lower && value < upper
}