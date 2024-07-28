mod gber;
mod network;
mod common_types;
mod common_utilities;

fn main() {
    let original_number = common_types::InputInt::MAX;
    let base = common_types::BaseInt::MAX - 1;
    let r = dbg!(gber::GBERepresentation::new(original_number, base));
    assert_eq!(r.to_decimal(), original_number);
    let mut running_matchups_sum = 0u64;
    let mut running_comparisons_sum = 0u64;
    for subnetwork_size in dbg!(r.calculate_components()) {
        let matchups = dbg!(network::calculate_matchups_number_for(subnetwork_size, r.base));
        let comparisons = dbg!(network::calculate_comparisons_number_for(matchups, r.base));
        running_matchups_sum += matchups;
        running_comparisons_sum += comparisons;
        if dbg!(r.remainder > 0) {
            running_matchups_sum += 1;
            running_comparisons_sum += dbg!(network::calculate_comparisons_number_for(1, r.base));
        }
    }
    dbg!(running_matchups_sum, running_comparisons_sum);
}
