#![allow(dead_code)]

use std::sync::Arc;
use std::sync::atomic::AtomicUsize;

const MI_SMALL_WSIZE_MAX: usize = 128;
#[cfg(target_arch = "x86_64")]
const MI_SMALL_SIZE_MAX: usize = MI_SMALL_WSIZE_MAX * 8;

#[cfg(target_arch = "x86")]
const MI_SMALL_SIZE_MAX: usize = MI_SMALL_WSIZE_MAX * 4;

#[cfg(target_feature = "MI_PADDING")]
pub const MI_PADDING_SIZE: usize = std::mem::size_of::<MiPaddingS>();

#[cfg(target_feature = "MI_PADDING")]
struct MiPaddingS {
    canary: usize,
    delta: usize,
}

pub const MI_PADDING_SIZE: usize = 0;

// pub const MI_PADDING_SIZE: usize = 0;
// const MI_PADDING_WSIZE:usize =

type MiHeapT = MiHeapS;
// Thread free list.
// We use the bottom 2 bits of the pointer for mi_delayed_t flags
type MiThreadFreeT = usize;

// The free lists use encoded next fields
// (Only actually encodes when MI_ENCODED_FREELIST is defined.)
type MiEncodedT = usize;

struct MiSegmentsTldT {}

enum MiPageKindT {
    // small blocks go into 64kb pages inside a segment
    Small,
    // medium blocks go into 512kb pages inside a segment
    Medium,
    // larger blocks go into a single page spanning a whole segment
    Large,
    // huge blocks (>512kb) are put into a single page in a segment of the exact size (but still 2mb aligned)
    Huge,
}

struct MiBlockT {
    next: MiEncodedT,
}

// A page contains blocks of one specific size (`block_size`).
// Each page has three list of free blocks:
// `free` for blocks that can be allocated,
// `local_free` for freed blocks that are not yet available to `mi_malloc`
// `thread_free` for freed blocks by other threads
// The `local_free` and `thread_free` lists are migrated to the `free` list
// when it is exhausted. The separate `local_free` list is necessary to
// implement a monotonic heartbeat. The `thread_free` list is needed for
// avoiding atomic operations in the common case.
//
//
// `used - |thread_free|` == actual blocks that are in use (alive)
// `used - |thread_free| + |free| + |local_free| == capacity`
//
// We don't count `freed` (as |free|) but use `used` to reduce
// the number of memory accesses in the `mi_page_all_free` function(s).
//
// Notes:
// - Access is optimized for `mi_free` and `mi_page_alloc` (in `alloc.c`)
// - Using `uint16_t` does not seem to slow things down
// - The size is 8 words on 64-bit which helps the page index calculations
//   (and 10 words on 32-bit, and encoded free lists add 2 words. Sizes 10
//    and 12 are still good for address calculation)
// - To limit the structure size, the `xblock_size` is 32-bits only; for
//   blocks > MI_HUGE_BLOCK_SIZE the size is determined from the segment page size
// - `thread_free` uses the bottom bits as a delayed-free flags to optimize
//   concurrent frees where only the first concurrent free adds to the owning
//   heap `thread_delayed_free` list (see `alloc.c:mi_free_block_mt`).
//   The invariant is that no-delayed-free is only set if there is
//   at least one block that will be added, or as already been added, to
//   the owning heap `thread_delayed_free` list. This guarantees that pages
//   will be freed correctly even if only other threads free blocks.
#[repr(C)]
struct MiPageT {
    // index in the segment `pages` array, `page == &segment->pages[page->segment_idx]`
    segment_idx: u8,
    // `true` if the segment allocated this page
    segment_in_use: bool,
    // `true` if the page memory was reset
    is_reset: bool,
    // `true` if the page virtual memory is committed
    is_committed: bool,
    // `true` if the page was zero initialized
    is_zero_init: bool,

    // layout like this to optimize access in `mi_malloc` and `mi_free`

    // number of blocks committed, must be the first field, see `segment.c:page_clear`
    capacity: u16,
    // number of blocks reserved in memory
    reserved: u16,
    // `in_full` and `has_aligned` flags (8 bits)
    flags: MiPageFlagsT,
    // `true` if the blocks in the free list are zero initialized
    is_zero: bool,
    // expiration count for retired blocks
    retire_expire: u8,

    // list of available free blocks (`malloc` allocates from this list)
    free: &'static MiBlockT,
    // two random keys to encode the free lists (see `_mi_block_next`)
    #[cfg(feature = "MI_ENCODE_FREELIST")]
    keys: [usize; 2],
    used: usize,
    xblock_size: usize,

    local_free: &'static MiBlockT,
    xthread_free: Arc<MiThreadFreeT>,
    xheap: AtomicUsize,

    next: &'static MiPageT,
    prev: &'static MiPageT,

}



#[repr(C)]
#[derive(Copy, Clone)]
// The `in_full` and `has_aligned` page flags are put in a union to efficiently
// test if both are false (`full_aligned == 0`) in the `mi_free` routine.
union MiPageFlagsT {
    full_aligned: u8,
    x: X,
}

#[derive(Copy, Clone)]
struct X {
    in_full: bool,
    has_aligned: bool,
}

struct MiSegmentS {
    memid: usize,
    // id for the os-level memory manager
    mem_is_fixed: bool,
    // `true` if we cannot decommit/reset/protect in this memory (i.e. when allocated using large OS pages)
    mem_is_committed: bool, // `true` if the whole segment is eagerly committed

    // segment fields
    next: &'static MiSegmentS,
    // must be the first segment field -- see `segment.c:segment_alloc` TODO comment
    prev: &'static MiSegmentS,
    abandoned_next: &'static MiSegmentS,
    abandoned: usize,
    abandoned_visited: usize,

    used: usize,
    // count of pages in use (`used <= capacity`)
    capacity: usize,
    // count of available pages (`#free + used`)
    segment_size: usize,
    // for huge pages this may be different from `MI_SEGMENT_SIZE`
    segment_info_size: usize,
    // space we are using from the first page for segment meta-data and possible guard pages.
    cookie: usize, // uintptr_t // verify addresses in secure mode: `_mi_ptr_cookie(segment) == segment->cookie`

    page_shift: usize,
    thread_id: AtomicUsize,
    // volatile _Atomic(uintptr_t)
    page_kind: MiPageKindT,
}

struct MiHeapS {
    tld: &'static MiTldS,
    // pages_free_direct[],
}

struct MiTldS {
    heartbeat: u64,
    recurse: bool,
    heap_backing: &'static MiHeapT,
    heaps: &'static MiHeapT,

}

#[cfg(test)]
mod tests {
    use crate::types::MI_PADDING_SIZE;

    #[test]
    fn test_heap_alloc() {
        println!("the size is: {}", MI_PADDING_SIZE);
    }
}