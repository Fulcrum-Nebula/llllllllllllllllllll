use typex_core::{dispatch, Extension, Outcome, Disposition};
struct Counter;
impl Extension for Counter {
    type State = u32;
    type Event = u32;
    type View = String;
    type Effect = bool;
    fn step(&self, state: &u32, event: u32) -> Outcome<u32, bool> {
        Outcome { state: state + event, disposition: Disposition::Consume, effects: vec![true] }
    }
    fn view(&self, state: &u32) -> String { state.to_string() }
    fn probe(&self, _: &u32, _: &u32) -> Disposition { Disposition::Consume }
}
#[test]
fn non_math_extension_dispatches_typed_values() {
    let result = dispatch(&Counter, &2, 3);
    assert_eq!(result.state, 5);
    assert_eq!(result.effects, vec![true]);
    assert_eq!(Counter.view(&result.state), "5");
    assert_eq!(Counter.probe(&2, &3), Disposition::Consume);
}
