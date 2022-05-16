use hashfn::hashfn;

#[test]
fn test_noop_hash_eq() {
    #[hashfn]
    fn noop() {}

    assert_eq!(
        "434aed607068173f18e2dce5cac8d0607e4027c4d3349d71278335d8734d7510",
        NOOP_HASH
    );
    noop();
}

#[test]
fn test_noop_different_visibility() {
    #[hashfn]
    pub(crate) fn noop() {}

    assert_eq!(
        "06da81c4933d90d64ed5f156349b3856a7dbdda28d2afff38fcbc5c9e09e7a82",
        NOOP_HASH
    );
    noop();
}

#[test]
fn test_noop_with_name() {
    #[hashfn(NOOP)]
    pub(crate) fn noop() {}

    assert_eq!(
        "06da81c4933d90d64ed5f156349b3856a7dbdda28d2afff38fcbc5c9e09e7a82",
        NOOP
    );
    noop();
}

#[test]
fn test_adder_hash_eq() {
    #[hashfn]
    fn adder(x: i32, y: i32) -> i32 {
        x + y
    }

    assert_eq!(
        "f9734f8f9401fd980e11a095223e6b566f81b539207c2453f6f6a28d6e8bbd33",
        ADDER_HASH
    );
    assert_eq!(adder(1, 2), 3);
}

#[allow(clippy::let_and_return)]
#[test]
fn test_adder_same_signature_different_body() {
    #[hashfn]
    fn adder(x: i32, y: i32) -> i32 {
        let sum = x + y;
        sum
    }

    assert_eq!(
        "d54fd0062b388ea9af1ae4871a164473f914f72277e5c572f79e0c599cdf3b7c",
        ADDER_HASH
    );
    assert_eq!(adder(1, 2), 3);
}
