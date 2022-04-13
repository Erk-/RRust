# RRust, reversible DSL for Rust

RRust is a reversible subset of Rust inside of Rust, this means
you can use it to write a algorithm and then it can run forwards
as usually, but it can also run in the reverse direction undoing
everything it just did.

# Example

```rust
use rrust::{rfn, rif};

rfn!(Fib, (x1: &mut i32, x2: &mut i32, n: &mut i32), {
    rif!(
        *n == 0,
        {
            *x1 += 1;
            *x2 += 1;
        },
        {
            *n -= 1;
            Fib::forward(x1, x2, n);
            *x1 += *x2;
            std::mem::swap(x1, x2);
        },
        *x1 == *x2
    );
});

let mut x1 = 0;
let mut x2 = 0;
let mut n = 10;

Fib::forward(&mut x1, &mut x2, &mut n);

assert_eq!(x1, 89);
assert_eq!(x2, 144);
assert_eq!(n, 0);

Fib::backwards(&mut x1, &mut x2, &mut n);

assert_eq!(x1, 0);
assert_eq!(x2, 0);
assert_eq!(n, 10);
```

# Limitations

To keep the code reversible it is necessary to put some limitations on what is possible to do.

## Mutating operations

The only operations in this DSL that can cause a mutation are
`+=`, `-=` and `^=` all other mutating operations are disallowed
as they cannot be reversed.

Though it is possible to use other operations together with
mutating operations for example in `a += e`. Here `a` must be a
identifier or a dereference of a identifier, but e can be any
expression that does not cause a mutation.

| Operator | Reverse |
|----------|---------|
|  `+=`    |  `-=`   |
|  `-=`    |  `+=`   |
|  `^=`    |  `^=`   |

## Aliasing

Mutable aliasing is not allowed and will cause a runtime error if
attempted. This is because a operation with aliasing can cause
loss of information and thus making it irreversible. For example `a
-= a` will always cause `a` to be nullified and thus causing a
loss of information.

## Function and method calls

At the given time no non-reversible Rust functions or methods are
allowed to be called inside of reversible code, this is a
something that can be changed since non-mutating functions and
methods could be allowed here.

# Bibliography
The language as it is now is mostly based upon the
[Janus](https://en.wikipedia.org/wiki/Janus_(time-reversible_computing_programming_language))
formalized in the following paper:

Tetsuo Yokoyama and Robert Glück. 2007. A reversible programming
language and its invertible self-interpreter.
[DOI](https://doi.org/10.1145/1244381.1244404)
