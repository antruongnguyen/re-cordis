use yaml_rust2::{
    parser::{Event, MarkedEventReceiver, Parser},
    scanner::Marker,
};
#[derive(Default)]
struct Events(Vec<(Event, Marker)>);
impl MarkedEventReceiver for Events {
    fn on_event(&mut self, e: Event, m: Marker) {
        self.0.push((e, m));
    }
}
fn events(text: &str) -> Events {
    let mut e = Events::default();
    Parser::new_from_str(text).load(&mut e, true).unwrap();
    e
}
#[test]
fn scalar_events_retain_text_style_and_marks() {
    let e = events("yes: true\nnum: 012\nquoted: 'false'\n");
    assert!(
        e.0.iter()
            .any(|(v, m)| matches!(v,Event::Scalar(s,_,_,_) if s=="012") && m.line() == 2)
    );
}
#[test]
fn duplicate_keys_are_not_rejected_by_event_parser() {
    let e = events("a: 1\na: 2\n");
    assert_eq!(
        e.0.iter()
            .filter(|(v, _)| matches!(v,Event::Scalar(s,_,_,_) if s=="a"))
            .count(),
        2
    );
}
#[test]
fn alias_and_anchor_are_visible() {
    let e = events("a: &x [1]\nb: *x\n");
    assert!(e.0.iter().any(|(v, _)| matches!(v, Event::Alias(_))));
    assert!(
        e.0.iter()
            .any(|(v, _)| matches!(v,Event::SequenceStart(a,_) if *a!=0))
    );
}
#[test]
fn tags_are_visible() {
    let e = events("a: !js 1+1\n");
    assert!(
        e.0.iter()
            .any(|(v, _)| matches!(v, Event::Scalar(_, _, _, Some(_))))
    );
}
#[test]
fn comments_and_style_are_not_lossless() {
    let plain = events("a: 1\n");
    let commented = events("# comment\na: 1 # trailing\n");
    let p: Vec<_> = plain.0.iter().map(|(e, _)| e).collect();
    let c: Vec<_> = commented.0.iter().map(|(e, _)| e).collect();
    assert_eq!(p, c);
}
