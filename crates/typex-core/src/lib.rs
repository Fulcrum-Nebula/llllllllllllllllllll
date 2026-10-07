//! Pure route A core; no platform or transport dependencies.
#![forbid(unsafe_code)]
pub mod typex;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Disposition { Pass, Consume, NotKey }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Outcome<S, E> {
    pub state: S,
    pub disposition: Disposition,
    pub effects: Vec<E>,
}

/// Generic dispatch has no knowledge of any extension's grammar or state.
pub trait Extension {
    type State;
    type Event;
    type View;
    type Effect;
    fn step(&self, state: &Self::State, event: Self::Event) -> Outcome<Self::State, Self::Effect>;
    fn view(&self, state: &Self::State) -> Self::View;
    fn probe(&self, state: &Self::State, event: &Self::Event) -> Disposition;
}

pub fn dispatch<X: Extension>(extension: &X, state: &X::State, event: X::Event) -> Outcome<X::State, X::Effect> {
    extension.step(state, event)
}
