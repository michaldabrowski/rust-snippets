fn main() {
    let is_even = |num| num % 2 == 0;
    assert_eq!(is_even(14), true);
    assert_eq!(is_even(21), false);
}
