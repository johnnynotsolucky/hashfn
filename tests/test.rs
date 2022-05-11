use hashfn::hashfn;

#[hashfn]
pub(crate) fn noop() {}

/// TODO
#[test]
fn test_hashfn() {
    assert_eq!(
        "6da81c4933d90d64ed5f156349b3856a7dbdda28d2afff38fcbc5c9e09e7a82",
        NOOP_HASH
    );
    noop();
}
