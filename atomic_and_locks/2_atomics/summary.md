- Atomics operations are the main building block for anything involvong multiple threads. Mutex and Condvar implemented using it. Atomics operation are available as methods on the standard atomics types std::sync::atomic like AtomicI32 or AtomicUsize.
- Memory ordering - is a mode of operation of atomics. It defines how the atomic operation interacts with other threads. There are 6 memory ordering modes:
  - Relaxed - no ordering guarantees
  <!-- - Release - all previous writes are visible to other threads
  - Acquire - all subsequent reads are visible to other threads
  - AcqRel - both Release and Acquire
  - SeqCst - all operations are visible to other threads in the order they were performed
  - Consume - only subsequent reads that depend on the current value are visible to other threads -->
- Atomic operations: load and store (store takes shared reference of self, but it actually modify)
- thread scope joins all threads on drop
- Using lazy initialization of static Atomic variable with more then 1 thread could cause a race. Could be solved by std::sync::Once or OnceLock or using compare_exchange_weak().
- fetch_* - atomic operations that return old value, but not always the same as load, so it is unreliable. fetch_add and fetch_sub won't panic on overflow, instead flips to beginning or the end.
- working with Atomics could safe some bits of time on synchronization, but it is not always the case. Use Mutex if need synchronization of the data in price of some time.
- Not all atomic types avaliable on all platforms.
