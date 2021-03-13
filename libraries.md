# Libraries

## Параллелизм/Конкуренция

[rayon-rs/rayon](https://github.com/rayon-rs/rayon) - параллельная работа над структурами данных
[parking_lot](https://github.com/Amanieu/parking_lot) - more efficient Mutex, Condvar, RwLock than in std
[rust-evmap: A lock-free, eventually consistent, concurrent multi-value map.](https://github.com/jonhoo/rust-evmap)
[sharded-slab: a lock-free concurrent slab](https://github.com/hawkw/sharded-slab)
[dashmap: Blazing fast concurrent HashMap for Rust.](https://github.com/xacrimon/dashmap)
[Transaction Systems and Data Grid](https://github.com/vertexclique/lever) - MVCC lock-free wait-free for in multi threads (OS threads) operations (25+ million operations under 2 seconds)
[beef: Faster, compact implementation of std::Cow](https://github.com/maciejhirsz/beef) - clone-on-write smart pointer.

## collections, allocations
[computations...](https://github.com/salsa-rs/salsa) - functions memoization
[indexmap](https://github.com/bluss/indexmap) - hashmap, сохраняет порядок вставки
[jeromefroe/lru-rs: LRU cache](https://github.com/jeromefroe/lru-rs) - алгоритм, при котором вытесняются значения, которые дольше всего не запрашивались
[Base-16 Modified Merkle Tree](https://github.com/paritytech/trie) - бинарное KV-хранилище, имеющее возможность подтвердить наличие тех или иных данных
[slab: Slab allocator](https://github.com/carllerche/slab) - предварительно выделенное хранилище типа Vec<T>, при вставке отдает ключ, по которому потом нужно вытаскивать значение
[typed-arena - аллокация памяти, без удаления отдельных элементов](https://crates.io/crates/typed-arena)
[shared-arena](https://github.com/sebastiencs/shared-arena) - A thread-safe & efficient memory pool
[generational-arena](https://github.com/fitzgen/generational-arena)
[B-tree for small 32bit keys and values](https://crates.io/crates/cranelift-bforest)

### priority queue
Нам нужна однопоточная очередь или многопоточная?
[garro95/priority-queue](https://github.com/garro95/priority-queue) - priority queue with change priority
[smol-rs/concurrent-queue](https://github.com/smol-rs/concurrent-queue) - concurrent multi-producer multi-consumer queue with get closed for push features
[crossbeam-queue](https://github.com/crossbeam-rs/crossbeam) - ?
std::collections::binary_heap
[min_max_heap](https://github.com/tov/min-max-heap-rs) - min and max value O(1), extremum - O(log n)
[binary-heap-plus](https://github.com/sekineh/binary-heap-plus-rs) - Enhancement over std::collections::BinaryHeap

### BitVec
[bit-vec](https://github.com/contain-rs/bit-vec) - 89 stars, 21 contrib
[smallbitvec](https://github.com/servo/smallbitvec) - 27 stars, 10 contrib
[bitvec](https://github.com/bitvecto-rs/bitvec) - 405 stars, 16 contrib
