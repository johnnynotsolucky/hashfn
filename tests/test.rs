use hashfn::hashfn;

/// Expands to
///
/// ```
/// pub(crate) const DO_A_THING_HASH: &str =
///     "a429692d48481f8baed209e387949132f77fe7c3465e121ae31d7518db2f";
/// pub(crate) fn do_a_thing() {}
/// ```
#[hashfn]
pub(crate) fn do_a_thing() {

}

/// TODO
#[test]
fn test_hashfn() {
    assert_eq!(
        "5f7919f82b459da2a812694d479eae667797b2f36b4c8fe5e51d1aeb1fad78f",
        DO_A_THING_HASH
    );
    do_a_thing();
}
