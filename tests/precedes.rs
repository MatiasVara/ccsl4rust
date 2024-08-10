use solver::Precedes;

#[test]
fn precedes() {
    let mut precedes: Precedes = Precedes::new("Driver_tick", "FeatureOK_tick");

    // FeatureOK_tick cannot tick before Driver_tick
    assert!(!precedes.evaluate(false, true));
    assert!(precedes.evaluate(true, false));
    assert!(precedes.evaluate(false, true));
    assert!(precedes.evaluate(true, false));
}
