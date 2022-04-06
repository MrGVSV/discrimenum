# discrimenum

Derive `Hash` and `PartialEq` on enums where only the discriminant matters.

### Why?

There are times where you have an enum where its variants might contain some data. However, you don't necessarily care
what that data is when comparing or hashing, just that the discriminant matches. This can be done with a super simple
impl—it's really only five lines of code. This crate simply does it for you as a convenience.

### Usage

By default, discrimenum's derives take the name of the respective trait. Which means you only need to add the following
line for things to work:

```rust
use discrimenum::{Hash, PartialEq};
```

> Keep in mind that this will shadow the existing derives automatically imported in the prelude, preventing non-discrimenum usage for the current scope.
>
> If this is undesired, either...
> * Qualify it: `#[derive(discrimenum::Hash)]`
> * Rename it: `use discrimenum::Hash as DHash;`
> * Scope it:
> 
>   ```rust
>   // ...
>   {
>     use discrimenum::Hash;
>     #[derive(Hash)]
>     enum {
>       // ...
>     }
>   }
>   // ...
>   ```

Then apply it to your enum:

```rust
#[derive(Hash, PartialEq)]
enum MyEnum {
    A(usize),
    B(usize)
}

assert_eq!(MyEnum::A(123), MyEnum::A(321));
assert_ne!(MyEnum::A(123), MyEnum::B(123));
```

#### Generics

This also applies to generic enums. In fact, since we only care about the discriminants, none of the generics need to be
bound by `Hash` or `PartialEq`:

```rust
#[derive(Hash, PartialEq)]
enum MyGenericEnum<T> {
    A(T),
    B(T)
}

// Notice we can put `T` instead of `T: Hash + PartialEq` 
```

### Difference from similar crates?

None. Just felt like adding another 😉
