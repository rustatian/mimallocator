// A heap owns a set of pages.
// struct mi_heap_s {
//     mi_tld_t*             tld;
//     mi_page_t*            pages_free_direct[MI_PAGES_DIRECT];  // optimize: array where every entry points a page with possibly free blocks in the corresponding queue for that size.
//     mi_page_queue_t       pages[MI_BIN_FULL + 1];              // queue of pages for each size class (or "bin")
//     volatile _Atomic(mi_block_t*) thread_delayed_free;
//     uintptr_t             thread_id;                           // thread this heap belongs too
//     uintptr_t             cookie;                              // random cookie to verify pointers (see `_mi_ptr_cookie`)
//     uintptr_t             keys[2];                             // two random keys used to encode the `thread_delayed_free` list
//     mi_random_ctx_t       random;                              // random number context used for secure allocation
//     size_t                page_count;                          // total number of pages in the `pages` queues.
//     size_t                page_retired_min;                    // smallest retired index (retired pages are fully free, but still in the page queues)
//     size_t                page_retired_max;                    // largest retired index into the `pages` array.
//     mi_heap_t*            next;                                // list of heaps per thread
//     bool                  no_reclaim;                          // `true` if this heap should not reclaim abandoned pages
// };

struct MiHeapS {

}

type MiHeapT = MiHeapS;

// Thread local data
// struct mi_tld_s {
//     unsigned long long  heartbeat;     // monotonic heartbeat count
//     bool                recurse;       // true if deferred was called; used to prevent infinite recursion.
//     mi_heap_t*          heap_backing;  // backing heap of this thread (cannot be deleted)
//     mi_heap_t*          heaps;         // list of heaps in this thread (so we can abandon all when the thread terminates)
//     MiSegmentsTldT   segments;      // segment tld
//     mi_os_tld_t         os;            // os tld
//     mi_stats_t          stats;         // statistics
// };

// Segments thread local data
// typedef struct mi_segments_tld_s {
//     mi_segment_queue_t  small_free;   // queue of segments with free small pages
//     mi_segment_queue_t  medium_free;  // queue of segments with free medium pages
//     mi_page_queue_t     pages_reset;  // queue of freed pages that can be reset
//     size_t              count;        // current number of segments;
//     size_t              peak_count;   // peak number of segments
//     size_t              current_size; // current size of all segments
//     size_t              peak_size;    // peak size of all segments
//     size_t              cache_count;  // number of segments in the cache
//     size_t              cache_size;   // total size of all segments in the cache
//     mi_segment_t*       cache;        // (small) cache of segments
//     mi_stats_t*         stats;        // points to tld stats
//     mi_os_tld_t*        os;           // points to os stats
// } MiSegmentsTldT;

struct MiSegmentsTldT {

}

struct MiSegmentS {
    memid: usize,
    mem_is_fixed: bool,
    mem_is_committed: bool,

    // segment fields
    next: &'static MiSegmentS,
    prev: &'static MiSegmentS,
    abandoned_next: &'static MiSegmentS,

}

struct MiTldS {
    heartbeat: u64,
    recurse: bool,
    heap_backing: &'static MiHeapT,
    heaps: &'static MiHeapT,

}