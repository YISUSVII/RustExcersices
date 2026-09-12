use testing_lab::parse_pair;

#[test]
fn rejects_bad_input() {
    assert!(parse_pair("nope").is_err());
    assert!(parse_pair("2x").is_err());
}
