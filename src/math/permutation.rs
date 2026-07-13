pub fn next_permutation<T: Ord>(values: &mut [T]) -> bool {
    let n = values.len();
    if n < 2 {
        return false;
    }

    for i in (0..n - 1).rev() {
        if values[i] < values[i + 1] {
            for j in (i + 1..n).rev() {
                if values[i] < values[j] {
                    values.swap(i, j);
                    break;
                }
            }

            values[i + 1..].reverse();
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::next_permutation;

    #[test]
    fn simple_sequence() {
        let mut values = [1, 2, 3];
        assert!(next_permutation(&mut values));
        assert_eq!(values, [1, 3, 2]);
    }

    #[test]
    fn descending_sequence() {
        let mut values = [3, 2, 1];
        assert!(!next_permutation(&mut values));
        assert_eq!(values, [3, 2, 1]);
    }
}
