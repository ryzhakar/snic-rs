//! Python bindings for the snic library.
use snic_core::{common_types, gber, network};
use pyo3::prelude::*;

/// Generate all matchups of a network of a given size.
/// TODO: stream instead of collecting all in memory.
#[pyfunction]
#[pyo3(signature = (network_size, match_size))]
fn stream_matches_from<'py>(
    _py: Python<'py>,
    network_size: common_types::InputInt,
    match_size: common_types::BaseInt,
) -> Vec<Vec<common_types::InputInt>> {
    let decomposition =
        gber::Decomposition::new(network_size, match_size).expect("Invalid decomposition values");
    let matchups_manager = network::matchup::LocalMatchupsManager::new(decomposition);
    matchups_manager.intersubnetwork_matchups
        .into_iter().chain(
            matchups_manager.subnetwork_iterators
                .into_iter().flatten()
        ).collect()
}

#[pymodule]
fn snic(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(stream_matches_from, m)?)?;
    Ok(())
}

