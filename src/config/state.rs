use shared_singleton::*;

pub struct State {}

impl Default for State {
    fn default() -> Self {
        Self {}
    }
}

impl_singleton_arc_mutex_tokio!(State, State::default());
