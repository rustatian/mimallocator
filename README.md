# mimallocator
This is the educational project. I am trying to rewrite mimalloc allocator [link](https://microsoft.github.io/mimalloc) into the Rust. Let's start the journey :)

Some Rust problems which I got during the implementation:
1. https://github.com/rust-lang/rust/issues/49804 + [GCC](https://gcc.gnu.org/onlinedocs/gcc/Unnamed-Fields.html). Rust doesn't support unnamed(anonymous) struct/union fields at the time.
For example on union in C:
```C
typedef union mi_page_flags_s {
  uint8_t full_aligned;
  struct {
    uint8_t in_full : 1;
    uint8_t has_aligned : 1;
  } x;
} mi_page_flags_t;
```
Rewrites into 2 types in Rust. It's not really a problem, just note:
```rust
#[repr(C)]
#[derive(Copy, Clone)]
union MiPageFlagsT {
    full_aligned: u8,
    x: X,
}

#[derive(Copy, Clone)]
struct X {
    in_full: bool,
    has_aligned: bool,
}
```
