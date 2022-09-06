# Bassert: Better Assertions

The `bassert` library contains two macros to make life easier:
- `bassert!`: a 'better assert' which asserts that a boolean expression is `true` at runtime, and prints the values of the operands.
- `debug_bassert!`: which works similarly, but is a no-op outside of debug builds.

The basic usage of the macros is similar to [`std::assert!`].
`bassert!` is meant as an improved replacement for [`std::assert!`], [`std::assert_eq!`], [`std::assert_ne!`],
and (basic usage of) the experimental [`std::assert_matches::assert_matches!`]

Instead of remembering multiple different assertion functions,
all common assertions can be written as `bassert!(binary_expression)`,
with `binary_expression` being some simple or complicated expression
that uses an operator in the following list.

# Supported operators
- `==` (equals)
- `!=` (not equals)
- `>` (greater than)
- `>=` (greater than or equals)
- `<` (less than)
- `<=` (less than or equals)
- `=` (match)

In all of these cases, if the assertion fails, the panic message will contain:
 - the passed expression
 - the actual value of the left-hand-side and right-hand-side operands to the operator.
 - If a custom format string (and optional extra arguments) were passed, these are printed as well.

 ## Requirements

 - The left-hand-side and right-hand-side operands both need to implement the [`std::fmt::Debug`] trait.
 - The particular traits required to evaluate the expression under consideration needs to be implemented. E.g. [`PartialEq`] or [`PartialOrd`].
 - If complex expressions are used as one (or both) of the operands, extra parentheses are required. This is a good idea for legibility,
   but also a requirement because of how the macro is written. If you forget, the compiler will remind you with a compiler error.

# Examples
This will happily pass:
```rust
let x = 10;
let y = 20;
bassert!(x < y) // All was happy in the world
```

The following will panic.
```rust
let x = 10;
let y = 20;
bassert!(y < x);
```
It will panic with the message:
```text
assertion failed: `y < x`
y: `20`,
x: `10`
```

## Custom messages
You can optionally pass a custom panic message with or without arguments for formatting. (Using the [`std::fmt`] syntax)
The expressions used as format arguments will only be evaluated if the assertion fails.

The custom panic message will not replace the normal panic message, but will be printed
at the end of the normal message:

```
let x = 10;
bassert!(x > (x + 2), "to surprise of no-one, x is not larger than x plus two. {}", "some_extra_argument")
```
This will panic with the message:
```text
assertion failed: `x > (x + 2)`
x: `10`,
(x + 2): `12`: to surprise of no-one, x is not larger than x plus two. some extra argument
```
