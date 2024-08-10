use solver::Alternates;

#[test]
fn alternates() {
    let mut alternates: Alternates = Alternates::new("start", "end");

    assert!(alternates.evaluate(true, false));
    assert!(alternates.evaluate(false, true));
    // end cannot tick without a start
    assert!(!alternates.evaluate(false, true));
}
