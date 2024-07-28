use crate::gber;
use crate::common_types::{InputInt, BaseInt};
use crate::common_utilities;
use std::collections::VecDeque;
use pyo3::prelude::*;


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


#[derive(Default, Debug)]
pub struct MatchupIterator {
    pub network_size: InputInt,
    pub matchup_size: BaseInt,
    pub offset: InputInt,
    level: u8,
    // Current item, exclusive end item
    section_states: Vec<(InputInt, InputInt)>,
    section_group_head: usize,
}

impl MatchupIterator {

    pub fn new(
        network_size: InputInt,
        matchup_size: BaseInt,
        offset: InputInt,
    ) -> Result<Self, String> {
        Ok(Self {
            network_size,
            matchup_size,
            offset,
            ..Default::default()
        })
    }
    fn init_level(&mut self) -> Option<()> {
        self.level += 1;
        let sections_number = self.calculate_sections_number();
        if sections_number > self.network_size {
            return None;
        }
        self.section_states.clear();
        let section_size: InputInt = self.network_size / sections_number;
        let mut head: InputInt = 0;
        let rangetails = (section_size..self.network_size + 1).filter(
            |&ix| ix % section_size == 0
        );
        self.section_states = rangetails.map(
            |tail| {
                let section = (head, tail);
                head += section_size;
                section
            }
        ).collect();
        self.section_group_head = 0;
        Some(())
    }

    fn calculate_sections_number(&self) -> InputInt {
        (self.matchup_size as InputInt).pow(self.level as InputInt)
    }
    fn increment_internal_section_pointer(&mut self, index: usize) {
        self.section_states[index].0 += 1;
    }
    fn increment_group_head_if_exhausted(&mut self) {
        let (ctrlhead, ctrltail) = self.section_states[self.section_group_head];
        if ctrltail == ctrlhead {
            self.section_group_head += self.matchup_size as usize;
        };
    }
}

impl Iterator for MatchupIterator {
    type Item = Vec<InputInt>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.section_group_head >= self.section_states.len() {
            self.init_level()?
        };
        let group_size = self.matchup_size as usize;
        let sections = self.section_states.clone();

        let selected_sections = sections.iter().enumerate()
            .skip(self.section_group_head)
            .take(group_size)
        ;
        let matchup: Vec<InputInt> = selected_sections
            .map(|(ix, section)| {
                let old_head = section.0.clone();
                self.increment_internal_section_pointer(ix);
                old_head + self.offset
            })
            .collect();
        self.increment_group_head_if_exhausted();

        Some(matchup)
    }

}

#[derive(Default, Debug)]
pub struct StreamNetworkMatchups {
    pub network_gber: gber::GBERepresentation,
    term_iterators: VecDeque<MatchupIterator>,
}

impl StreamNetworkMatchups {
    pub fn new(network_gber: gber::GBERepresentation) -> Self {
        let mut rolling_offset: InputInt = 0;
        let term_iterators = network_gber.calculate_components().iter()
            .map(
                |sn_size| {
                    let iter = MatchupIterator::new(sn_size.clone(), network_gber.base, rolling_offset).unwrap();
                    rolling_offset += sn_size;
                    iter
                }
        ).collect();
        Self {
            network_gber,
            term_iterators,
        }
    }
}

impl Iterator for StreamNetworkMatchups {
    type Item = Vec<InputInt>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut titlen = self.term_iterators.len();
        while titlen > 0 {
            let current_stream = self.term_iterators.iter_mut().next()?;
            if let Some(current_match) = current_stream.next() {
                return Some(current_match)
            } else {
                let _ = &mut self.term_iterators.pop_front();
                titlen = self.term_iterators.len();
                continue
            };
        }
        None
    }
}
