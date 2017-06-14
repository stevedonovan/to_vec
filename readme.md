## Capturing some Common `collect` patterns

A common pattern in Rust is collecting an iterator:

```rust
let values = &["one","two","three"];
let v: Vec<_> = values.iter().cloned().collect();

let s: String = values.iter().cloned().collect();

println!("vector is {:?}, string is '{}'", v, s);
// vector is ["one", "two", "three"], string is 'onetwothree'
```

`collect` is very versatile - but it needs a type hint. This can be
an imcomplete type like `Vec<_>` (ask the compiler to fill the blanks)
or using the turbofish operator like so `collect::Vec<_>()`.

Although you get used to this, I still find the notation a little clumsy.
The `to_vec` crate is designed to help in the common cases:

```rust
extern crate to_vec;
use to_vec::ToVec;

let v = values.iter().cloned().to_vec();
assert_eq!(v,&["one", "two", "three"]);
```

It is a simple trait (which must be therefore brought into scope)
which is implemented for iterators and leans on `FromIterator`
just like `collect` does.

One marvelous little specialization in the standard library will
collect an iterator of `Result<T,E>` and return a `Result<Vec<T>,E>`,
where the first error encountered will be returned. It's awkward
to type, even if you _do_ know about this hidden gem. Hence `to_vec_result`:

```rust
use to_vec::ToVecResult;

let numbers = "23E 5F5 FF00".split_whitespace()
    .map(|s| u32::from_str_radix(s,16)).to_vec_result().unwrap();

assert_eq!(numbers,&[0x23E, 0x5F5, 0xFF00]);
```

Although less commonly used, `collect` will also take an iterator of values
and create a `HashSet` from them.  Now often you only have an iterator
of references and need to pass through `cloned` like with the `to_vec`
example. `to_set` is given an iterator of references and implicitly
invokes `cloned` on that iterator. This gives us a tidy notation for
sets:

```rust
use to_vec::ToSet;

let colours = ["green","orange","blue"].iter().to_set();
let fruit = ["apple","banana","orange"].iter().to_set();
let common = colours.intersection(&fruit).to_set();
assert_eq!(common, ["orange"].iter().to_set());
```

Likewise, when collecting a map, you need a tuple, not a reference to a tuple.

```rust
const VALUES: &[(&str,i32)] = &[("hello",10),("dolly",20)];

let map = VALUES.iter().to_map();

assert_eq!(map.get("hello"),Some(&10));
assert_eq!(map.get("dolly"),Some(&20));
```

So, `to_set` and `to_map` implicitly clone. If you do have values and cloning
could be expensive, you always have the old method.
