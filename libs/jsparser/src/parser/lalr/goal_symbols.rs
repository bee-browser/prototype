// DO NOT EDIT THIS FILE BY HAND.
//
// This file was automagically generated with:
// template: libs/jsparser/src/parser/lalr/goal_symbols.rs.hbs

use super::State;

pub enum GoalSymbol {
    Script,
    Module,
    ArrowFormalParameters,
}

impl GoalSymbol {
    #[inline(always)]
    pub(crate) fn start_state_id(&self) -> State {
        match self {
            Self::Script => State(0),
            Self::Module => State(1),
            Self::ArrowFormalParameters => State(2),
        }
    }
}
