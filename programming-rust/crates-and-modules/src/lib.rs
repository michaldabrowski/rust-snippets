use std::ops::Range;

/// Returns true if two ranges overlap
///
///     assert_eq!(ranges::overlap(0..5, 3..7), true);
///     assert_eq!(ranges::overlap(0..2, 2..5), false);
///
/// If either range is empty, they do not overlap
///
///     assert_eq!(ranges::overlap(0..0, 1..5), false);
///
pub fn overlap(r1: Range<usize>, r2: Range<usize>) -> bool {
    r1.start < r1.end && r2.start < r2.end && r1.start < r2.end && r2.start < r1.end
}
