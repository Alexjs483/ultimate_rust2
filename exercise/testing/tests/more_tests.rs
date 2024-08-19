use testing::{sploosh, splish};

#[test]
fn external_splish_sploosh_integration_test() {
    assert_eq!(sploosh(splish(-1, 0), splish(1, 1), splish(3, 2)), 4);
}