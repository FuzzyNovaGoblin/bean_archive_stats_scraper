use shared_singleton::*;
use std::collections::HashMap;

pub struct State {}

impl Default for State {
    fn default() -> Self {
        Self {}
    }
}

impl_singleton_arc_mutex_tokio!(State, State::default());
