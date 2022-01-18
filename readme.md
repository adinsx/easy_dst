# easy_dst

I want to solve a couple problems with this library:
1. Make custom dynamically sized types (hereafter DSTs) *much* more ergonomic to define and instantiate.
2. Make such DSTs available on the stack (or heap, but doing *that* isn't currently *too* difficult without help of a library).
3. Make such DSTs available in const context.
4. Custom DSTs must be of arbitrary size, and shouldn't require the user to know their size when they create them.

I've seen some other crates that try to solve these problems, but I haven't seen any that solve all at once. This crate currently implements a proof-of-concept for a custom DST with a str member. Currently, the ergonomic macro `named!` only supports the creation of const DSTs. I chose the combination of str member + const context as these are two of the harder problems to solve, and solving them first would inform the rest of the design.