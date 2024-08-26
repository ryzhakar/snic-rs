use crate::common_types::BaseInt;

/// Generate pairwise combinations of indices for given size
fn generate_expansion_mould_for(size: BaseInt) -> Vec<(BaseInt, BaseInt)> {
    let mut comparison_indices: Vec<(BaseInt, BaseInt)> = vec!();
    for left_pointer in 0..(size-1) {
        for right_pointer in (left_pointer+1)..size {
            comparison_indices.push((left_pointer, right_pointer));
        };
    };
    comparison_indices
}


#[cfg(test)]
mod test_comparison_expansion {
    use super::generate_expansion_mould_for;
    use crate::common_types::{BaseInt, InputInt};
    const MATCHUP_SIZE: BaseInt = u8::MAX as BaseInt;

    #[test]
    fn correct_number_of_pairs() {
        let expected_pairs_number: InputInt;
        expected_pairs_number = (MATCHUP_SIZE as InputInt * (MATCHUP_SIZE as InputInt - 1)) / 2;
        assert_eq!(
            expected_pairs_number as usize,
            generate_expansion_mould_for(MATCHUP_SIZE).len()
        )
    }

    #[test]
    fn expand_3() {
        assert_eq!(
            generate_expansion_mould_for(3),
            vec![(0, 1), (0, 2), (1, 2)],
        );
    }

    #[test]
    fn expand_4() {
        assert_eq!(
            generate_expansion_mould_for(4),
            vec![
                (0, 1), (0, 2), (0, 3),
                (1, 2), (1, 3),
                (2, 3)
            ],
        );
    }
}
