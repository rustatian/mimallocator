# mimallocator
This is the educational project. I am trying to rewrite mimalloc allocator [link](https://microsoft.github.io/mimalloc) into the Rust. Let's start the journey :)

Some Rust problems which I got during the implementation:
1. https://github.com/rust-lang/rust/issues/49804 [GCC](https://gcc.gnu.org/onlinedocs/gcc/Unnamed-Fields.html). Rust doesn't support unnamed(anonimous) struct/union fields at the time.
