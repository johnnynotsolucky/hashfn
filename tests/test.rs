use hashfn::hashfn;

/// Expands to
///
/// ```
/// pub(crate) const DO_A_THING_HASH: &str =
///     "927d548953088d9e1e2e69ca1b5c10ada7a1a08aebade958b7fbf16ddc284a";
/// pub(crate) fn do_a_thing() {}
/// ```
#[hashfn]
pub(crate) fn do_a_thing() {

}

/// TODO
#[test]
fn test_hashfn() {
    assert_eq!(
        "927d548953088d9e1e2e69ca1b5c10ada7a1a08aebade958b7fbf16ddc284a",
        DO_A_THING_HASH
    );
    do_a_thing();
}
