# rs-sumtype-boilplate

Library `rs-sumtype-boilplate` provides some useful macros to manipulate sum-type(enum) inside your project.

## Supported Macros

### Macro `create_sumtype`

Macro `create_sumtype` generates an enum type according to the types you passed.
For example, the following macro usage will produce an enum `MySum`.

```rust
struct A;
struct B;
create_sumtype!(MySum, A, B);
```

The `MySum` is equivalent to the definition below:

```rust
enum MySum {
    MySumA(A),
    MySumB(B),
}
```
