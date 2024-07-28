mod gber;
mod network;
mod common_types;
mod common_utilities;

fn main() {
    // let original_number = common_types::InputInt::MAX - 1;
    let original_number = 2u32.pow(10) - 1;
    let max_base = 19;
    let ntwk = network::StreamNetworkMatchups::new(
        gber::GBERepresentation::new(original_number, max_base)
    );
    for matchup in ntwk {
        dbg!(matchup);
    };
}
