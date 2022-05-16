# hashfn

A procedural macro to generate a hash representation of a function as a string slice.

The hash is generated as a const with the same visibility as the function the macro is applied
to.

## Example:

```
use hashfn::hashfn;

#[hashfn(DO_SOMETHING)]
pub(crate) fn do_something() {}

// Will expand to
// pub(crate) const DO_SOMETHING: &str = "<hash>";
// pub(crate) fn do_something() {}
```

`hashfn` will generate the name of the constant if it is omitted:

```
use hashfn::hashfn;

#[hashfn]
pub(crate) fn do_something() {}

// Will expand to
// pub(crate) const DO_SOMETHING_HASH: &str = "<hash>";
// pub(crate) fn do_something() {}
```
