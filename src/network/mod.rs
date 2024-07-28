use crate::common_types::{InputInt, BaseInt};
use crate::common_utilities;


pub fn calculate_matchups_number_for(
    subnetwork_size: InputInt,
    base: BaseInt,
) -> u64 {
    let initiator_items: u64 = (subnetwork_size / base as InputInt).into();
    let matchups_per_item = common_utilities::integer_log(subnetwork_size, base);
    initiator_items * matchups_per_item as u64
}


pub fn calculate_comparisons_number_for(
    matchups_number: u64,
    base: BaseInt,
) -> u64 {
    let base: InputInt = base.into();
    let per_matchup: InputInt = base * (base-1) / 2;
    matchups_number * per_matchup as u64
}
