mod gber;
mod network;
mod common_types;
mod common_utilities;
use pyo3::prelude::*;


#[pyfunction]
#[pyo3(signature = (network_size, match_size))]
fn stream_matches_from<'py>(
    _py: Python<'py>,
    network_size: common_types::InputInt,
    match_size: common_types::BaseInt,
) -> Vec<Vec<common_types::InputInt>> {
    let iterator = network::StreamNetworkMatchups::new(
        gber::GBERepresentation::new(
            network_size,
            match_size,
        )
    );
    let mut vector = vec![];
    for matchup in iterator {
        vector.push(matchup);
    };
    vector
}

#[pymodule]
fn snic(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(stream_matches_from, m)?)?;
    Ok(())
}
