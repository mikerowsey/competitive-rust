pub fn max_matches_with_tolerance(
    left_sorted: &[i64],
    right_sorted: &[i64],
    tolerance: i64,
) -> u64 {
    let mut left_index = 0usize;
    let mut right_index = 0usize;
    let mut matches = 0u64;

    while left_index < left_sorted.len() && right_index < right_sorted.len() {
        let left_value = left_sorted[left_index];
        let right_value = right_sorted[right_index];

        if (left_value - right_value).abs() <= tolerance {
            left_index += 1;
            right_index += 1;
            matches += 1;
        } else if left_value - right_value > tolerance {
            right_index += 1;
        } else {
            left_index += 1;
        }
    }

    matches
}

pub fn min_pairs_with_limit(sorted_values: &[u64], limit: u64) -> u64 {
    let mut left = 0usize;
    let mut right = sorted_values.len();
    let mut pairs = 0u64;

    while left < right {
        right -= 1;
        if left < right && sorted_values[left] + sorted_values[right] <= limit {
            left += 1;
        }
        pairs += 1;
    }

    pairs
}

pub fn find_two_sum_indices(sorted_values: &[(usize, u32)], target: u32) -> Option<(usize, usize)> {
    if sorted_values.len() < 2 {
        return None;
    }

    let mut left = 0usize;
    let mut right = sorted_values.len() - 1;

    while left < right {
        let sum = sorted_values[left].1 + sorted_values[right].1;
        if sum > target {
            right -= 1;
        } else if sum < target {
            left += 1;
        } else {
            return Some((sorted_values[left].0, sorted_values[right].0));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::{find_two_sum_indices, max_matches_with_tolerance, min_pairs_with_limit};

    #[test]
    fn matching_with_tolerance() {
        let left = [45, 60, 60, 80];
        let right = [30, 60, 75];
        assert_eq!(max_matches_with_tolerance(&left, &right, 5), 2);
    }

    #[test]
    fn pairing_with_limit() {
        let weights = [2, 3, 7, 9];
        assert_eq!(min_pairs_with_limit(&weights, 10), 3);
    }

    #[test]
    fn sorted_two_sum() {
        let values = [(4, 1), (1, 2), (3, 5), (2, 7)];
        assert_eq!(find_two_sum_indices(&values, 8), Some((4, 2)));
    }
}
