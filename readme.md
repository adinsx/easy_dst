# easy_dst

I want to solve a couple of problems with this library:
1. Make custom dynamically sized types (hereafter DSTs) *much* more ergonomic to define and instantiate.
2. Make such DSTs available on the stack (or heap, but doing *that* isn't currently *too* difficult without help of a library).
3. Make such DSTs available in const context.
4. Custom DSTs must be of arbitrary size, and shouldn't require the user to know their size when they create them.
5. Minimize the number of required type annotations.

I've seen some other crates that try to solve these problems, but I haven't seen any that solve all at once. This crate currently implements a proof-of-concept for a custom DST with a str member. Currently, the macro `named!` only supports the creation of const DSTs. I chose the combination of str member + const context as these are two of the harder problems to solve, and solving them first would inform the rest of the design.

As an example, the following macro invocation:
`named!(const FIVE = <[u32; _]>([1,2,3,4,5], "five"));`

Expands to the following rust:
```rust
const FIVE: Named<[u32; [1, 2, 3, 4, 5].len()]> = {
            const TMP: &([u32; [1, 2, 3, 4, 5].len()], [u8; "five".len()]) =
                &([1, 2, 3, 4, 5], crate::as_bytes_sized("five"));
            crate::Named::new(TMP)
        };
```
Which compiles to the following asm (on x64 linux):
```asm
.Lanon.8afc1e9bec810034dafd45c6854f1dd9.0:
	.ascii	"\001\000\000\000\002\000\000\000\003\000\000\000\004\000\000\000\005\000\000\000five"
	.size	.Lanon.8afc1e9bec810034dafd45c6854f1dd9.0, 24
```
