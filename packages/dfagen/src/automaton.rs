use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;

use indexmap::IndexMap;
use indexmap::IndexSet;
use serde::Deserialize;
use serde::Serialize;

use bee_macros::delegate_all;

use crate::unicode::UnicodeSet;
use crate::unicode::UnicodeSetsBuilder;

// macros

macro_rules! state {
    ($id:expr) => {
        State {
            id: $id.into(),
            ..Default::default()
        }
    };
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Nfa(Automaton);

delegate_all! {Nfa => Automaton}

impl Nfa {
    pub fn new() -> Self {
        Default::default()
    }

    #[tracing::instrument(level = "debug", skip_all)]
    pub fn build_dfa(&self) -> Dfa {
        let unicode_sets = self.build_unicode_sets();

        let mut dfa = Dfa::new();

        let dfa_start_id = dfa.create_state();
        dfa.add_label(dfa_start_id, "@start".to_string());

        let nfa_start_id = StateId(0);
        let nfa_start_group = self.compute_closure(&nfa_start_id.into());

        let mut dfa_state_map: IndexMap<StateGroup, StateId> = Default::default();
        dfa_state_map.insert(nfa_start_group.clone(), dfa_start_id);

        let mut remaining = VecDeque::new();
        remaining.push_back(nfa_start_group);
        while let Some(nfa_group) = remaining.pop_front() {
            let dfa_id = dfa_state_map.get(&nfa_group).unwrap().clone();
            tracing::debug!(state = %dfa.state(dfa_id), %nfa_group);
            for unicode_set in unicode_sets.iter() {
                let nfa_move_group = self.compute_move(&nfa_group, unicode_set);
                let nfa_next_group = self.compute_closure(&nfa_move_group);
                if nfa_next_group.is_empty() {
                    tracing::debug!(
                        state = %dfa.state(dfa_id),
                        %unicode_set,
                        next = ?(),
                    );
                    continue;
                }
                let dfa_next_id = dfa_state_map
                    .entry(nfa_next_group.clone())
                    .or_insert_with(|| {
                        let dfa_next_id = dfa.create_state();
                        if let Some(token) = self.determine_token(&nfa_next_group) {
                            dfa.accept(dfa_next_id, token);
                        }
                        // It's ensured that every state in the group before computing the
                        // closure was generated by a lookahead item in the lexical grammar.
                        if self.determine_lookahead(&nfa_next_group) {
                            dfa.lookahead(dfa_next_id);
                        }
                        for &nfa_id in nfa_next_group.iter() {
                            for label in self.state(nfa_id).labels.iter() {
                                dfa.add_label(dfa_next_id, label.clone());
                            }
                        }
                        remaining.push_back(nfa_next_group.clone());
                        dfa_next_id
                    })
                    .clone();
                tracing::debug!(
                    state = %dfa.state(dfa_id),
                    %unicode_set,
                    next = %dfa.state(dfa_next_id),
                );
                dfa.transition(dfa_id, dfa_next_id, unicode_set.clone());
            }
        }

        dfa
    }

    fn compute_move(&self, group: &StateGroup, unicode_set: &UnicodeSet) -> StateGroup {
        let mut new_group = StateGroup::new();

        for &id in group.iter() {
            for trans in self.state(id).transitions.iter() {
                if trans.unicode_set.contains(unicode_set) {
                    new_group.push(trans.next_id);
                }
            }
        }

        self.validate_group(&new_group);

        new_group
    }

    fn compute_closure(&self, group: &StateGroup) -> StateGroup {
        // Use BTreeSet internally for efficiency.
        let mut closure = BTreeSet::new();
        closure.extend(group.iter());

        let mut remaining: Vec<StateId> = group.iter().cloned().collect();
        while let Some(id) = remaining.pop() {
            let ids: Vec<StateId> = self
                .state(id)
                .transitions
                .iter()
                .filter(|trans| trans.unicode_set.is_empty())
                .filter(|trans| !closure.contains(&trans.next_id))
                .map(|trans| trans.next_id)
                .collect();
            for &id in ids.iter() {
                closure.insert(id);
                remaining.push(id);
            }
        }

        closure.into()
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Dfa(Automaton);

delegate_all! {Dfa => Automaton}

impl Dfa {
    fn new() -> Self {
        Default::default()
    }

    pub fn minify(&self, tokens: &[String]) -> Dfa {
        let unicode_sets = self.build_unicode_sets();

        let mut groups = vec![];
        // Separate lookahead states from others.
        self.collect_group(None, false, &mut groups);
        self.collect_group(None, true, &mut groups);
        // Create a separate group for each token.
        for token in tokens.iter() {
            self.collect_group(Some(token.as_str()), false, &mut groups);
            self.collect_group(Some(token.as_str()), true, &mut groups);
        }

        assert!(!groups.is_empty());

        let mut iteration = 0;
        loop {
            tracing::debug!(iteration, groups.len = groups.len());
            let mut new_groups = vec![];
            for group in groups.iter() {
                self.validate_group(&group);
                // Collect states having the same transition table in `groups`.
                let mut trans_map: IndexMap<Vec<Option<usize>>, StateGroup> = Default::default();
                for &id in group.iter() {
                    let transitions = self.build_transition_table(id, &unicode_sets, &groups);
                    trans_map
                        .entry(transitions)
                        .and_modify(|group| group.push(id))
                        .or_insert_with(|| StateGroup::from(id));
                }
                new_groups.extend(trans_map.into_values());
            }
            let done = groups.len() == new_groups.len();
            groups = new_groups;
            if done {
                break;
            }
            iteration += 1;
        }

        assert!(groups.iter().all(|group| !group.is_empty()));

        let mut dfa = Dfa::new();
        // Reconstruct states.
        for (i, group) in groups.iter().enumerate() {
            let new_id = dfa.create_state();
            if let Some(token) = self.determine_token(&group) {
                dfa.accept(new_id, token);
            }
            if self.determine_lookahead(&group) {
                dfa.lookahead(new_id);
            }
            for &id in group.iter() {
                for label in self.state(id).labels.iter() {
                    dfa.add_label(new_id, label.clone());
                }
            }
            assert_eq!(i, new_id.0);
        }
        // Reconstruct the transitions of each state.
        for (i, group) in groups.iter().enumerate() {
            // Every state in `group` has the same transitions.  So, we can use
            // `group.first()` for rebuilding the transitions of the new state.
            let &id = group.first().unwrap();
            for trans in self.state(id).transitions.iter() {
                let next = groups
                    .iter()
                    .position(|group| group.contains(&trans.next_id))
                    .unwrap();
                dfa.transition(i.into(), next.into(), trans.unicode_set.clone());
            }
        }

        dfa.minify_all_transitions();
        dfa
    }

    fn collect_group(&self, token: Option<&str>, lookahead: bool, groups: &mut Vec<StateGroup>) {
        let state_ids: Vec<StateId> = self
            .states
            .iter()
            .filter(|state| {
                state.accept.as_ref().map(|tok| tok.as_str()) == token
                    && state.lookahead == lookahead
            })
            .map(|state| state.id)
            .collect();
        if !state_ids.is_empty() {
            groups.push(StateGroup(state_ids));
        }
    }

    fn build_transition_table(
        &self,
        id: StateId,
        unicode_sets: &[UnicodeSet],
        groups: &[StateGroup],
    ) -> Vec<Option<usize>> {
        let mut transitions = vec![];
        for unicode_set in unicode_sets.iter() {
            let trans = self
                .state(id)
                .transitions
                .iter()
                .find(|trans| trans.unicode_set.contains(unicode_set));
            if let Some(trans) = trans {
                let next = groups
                    .iter()
                    .position(|group| group.contains(&trans.next_id));
                assert!(next.is_some());
                transitions.push(next);
            } else {
                transitions.push(None);
            }
        }
        transitions
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Automaton {
    states: Vec<State>,
}

impl Automaton {
    pub fn size(&self) -> usize {
        self.states.len()
    }

    pub(crate) fn create_state(&mut self) -> StateId {
        let id = self.states.len();
        self.states.push(state!(id));
        id.into()
    }

    fn state(&self, id: StateId) -> &State {
        self.states.get(id.0).unwrap()
    }

    fn state_mut(&mut self, id: StateId) -> &mut State {
        self.states.get_mut(id.0).unwrap()
    }

    pub(crate) fn add_label(&mut self, id: StateId, label: String) {
        self.state_mut(id).labels.insert(label);
    }

    pub(crate) fn accept(&mut self, id: StateId, token: String) {
        self.state_mut(id).accept = Some(token);
    }

    pub(crate) fn lookahead(&mut self, id: StateId) {
        self.state_mut(id).lookahead = true;
    }

    pub(crate) fn transition(&mut self, id: StateId, next_id: StateId, unicode_set: UnicodeSet) {
        let label = format!("{unicode_set} => {}", self.state(next_id));
        self.state_mut(id).transitions.push(Transition { next_id, unicode_set, label });
    }

    pub(crate) fn epsilon_transition(&mut self, id: StateId, next_id: StateId) {
        self.transition(id, next_id, UnicodeSet::empty());
    }

    pub fn build_unicode_sets(&self) -> Vec<UnicodeSet> {
        let mut builder = UnicodeSetsBuilder::new();
        for state in self.states.iter() {
            for trans in state.transitions.iter() {
                builder.add(&trans.unicode_set);
            }
        }
        builder.build()
    }

    fn minify_all_transitions(&mut self) {
        for i in 0..self.states.len() {
            self.minify_transitions(i.into());
        }
    }

    #[tracing::instrument(level = "debug", skip_all, fields(state = %self.state(state_id)))]
    fn minify_transitions(&mut self, state_id: StateId) {
        // Merge Unicode sets which make a transition to the same state.
        let unicode_sets: BTreeMap<StateId, UnicodeSet> = self
            .state(state_id)
            .transitions
            .iter()
            .fold(Default::default(), |mut unicode_sets, trans| {
                unicode_sets
                    .entry(trans.next_id)
                    .and_modify(|unicode_set| {
                        *unicode_set = unicode_set.merge(&trans.unicode_set);
                    })
                    .or_insert_with(|| trans.unicode_set.clone());
                unicode_sets
            });

        // Build a new transition table.
        let transitions: Vec<Transition> = unicode_sets
            .into_iter()
            .map(|(next_id, unicode_set)| {
                let label = format!("{unicode_set} => {}", self.state(next_id));
                Transition { next_id, unicode_set, label }
            })
            .inspect(|trans| {
                let next_state = self.states.get(trans.next_id.0).unwrap();
                tracing::debug!(
                    transition.next_state = %next_state,
                    transition.unicode_set = %trans.unicode_set
                );
            })
            .collect();

        // And then update the transition table.
        self.state_mut(state_id).transitions = transitions;
    }

    fn determine_token(&self, group: &StateGroup) -> Option<String> {
        // We assume that tokens specified in the command line have been sorted in
        // order of priority.  A higher priority token has a smaller identifier and
        // the `ids` have been sorted in ascending order.  So, we return the
        // identifier of the first accept state.
        group.iter().find_map(|&id| self.state(id).accept.clone())
    }

    fn determine_lookahead(&self, group: &StateGroup) -> bool {
        group.iter().any(|&id| self.state(id).lookahead)
    }

    fn validate_group(&self, group: &StateGroup) {
        if group.iter().any(|&id| self.state(id).lookahead) {
            // If a group has a state generated by a lookahead item, every state in
            // the group must be a state generated by a lookahead item.
            if !group.iter().all(|&id| self.state(id).lookahead) {
                tracing::error!("Ambiguous lexical grammer");
                for &id in group.iter() {
                    let state = self.state(id);
                    if !state.lookahead {
                        tracing::error!(%state);
                    }
                }
                panic!();
            }
        }
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
struct StateGroup(Vec<StateId>);

delegate_all! {StateGroup => Vec<StateId>}

impl StateGroup {
    fn new() -> Self {
        Default::default()
    }
}

impl From<StateId> for StateGroup {
    fn from(value: StateId) -> Self {
        StateGroup(vec![value])
    }
}

impl From<BTreeSet<StateId>> for StateGroup {
    fn from(value: BTreeSet<StateId>) -> Self {
        StateGroup(value.into_iter().collect())
    }
}

impl std::fmt::Display for StateGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        if let Some((last, leadings)) = self.split_last() {
            for id in leadings.iter() {
                write!(f, "{},", id)?;
            }
            write!(f, "{}", last)?;
        }
        write!(f, "]")
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct State {
    id: StateId,
    accept: Option<String>,
    lookahead: bool,
    transitions: Vec<Transition>,
    labels: IndexSet<String>,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "State({})", self.id.0)?;
        if let Some(ref token) = self.accept {
            write!(f, ":{token}")?;
        }
        if self.lookahead {
            write!(f, "?")?;
        }
        Ok(())
    }
}

#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize,
)]
pub struct StateId(usize);

impl From<usize> for StateId {
    fn from(value: usize) -> Self {
        StateId(value)
    }
}

impl std::fmt::Display for StateId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Transition {
    next_id: StateId,
    unicode_set: UnicodeSet,
    label: String,
}

// <coverage:exclude>
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_format() {
        assert_eq!(format!("{}", state!(0)), "State#0");
        let mut state = state!(0);
        state.accept = Some("token".to_string());
        assert_eq!(format!("{}", state), "State#0(token)");
        state.lookahead = true;
        assert_eq!(format!("{}", state), "State#0(token)?");
    }
}
// </coverage:exclude>
