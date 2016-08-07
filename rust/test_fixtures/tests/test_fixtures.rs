extern crate test_fixtures;

#[test]
fn should_return_hotspot_name() {
    assert_eq!("hotspot name", test_fixtures::parse("asdasd"));
}
