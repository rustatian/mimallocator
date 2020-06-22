#![allow(dead_code)]

struct MiHeapS {}

type MiHeapT = MiHeapS;


struct MiSegmentsTldT {}

enum MiPageKindT {
    MiPageSmall,
    // small blocks go into 64kb pages inside a segment
    MiPageMedium,
    // medium blocks go into 512kb pages inside a segment
    MiPageLarge,
    // larger blocks go into a single page spanning a whole segment
    MiPageHuge,      // huge blocks (>512kb) are put into a single page in a segment of the exact size (but still 2mb aligned)
}

#[repr(C)]
struct MiPageT {
    segment_idx: u8,
    segment_in_use: bool,
    is_reset: bool,
    is_committed: bool,
    is_zero_init: bool,

    //
    capacity: u16,
    reserved: u16,

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
    thread_id: usize,
    // volatile _Atomic(uintptr_t)
    page_kind: MiPageKindT,

}

struct MiTldS {
    heartbeat: u64,
    recurse: bool,
    heap_backing: &'static MiHeapT,
    heaps: &'static MiHeapT,

}