# Libraries

**queue**
https://stackoverflow.com/questions/40848918/are-there-queue-and-stack-collections
https://github.com/crossbeam-rs/crossbeam - crossbeam-queue
[stjepang/concurrent-queue](https://github.com/stjepang/concurrent-queue)
[garro95/priority-queue](https://github.com/garro95/priority-queue)
github.com/kinghajj/deque

## Параллелизм/Конкуренция

[rayon-rs/rayon](https://github.com/rayon-rs/rayon) - параллельная работа над структурами данных
github.com/crossbeam-rs/crossbeam
[parking_lot](https://github.com/Amanieu/parking_lot) - Mutex
[rust-evmap: A lock-free, eventually consistent, concurrent multi-value map.](https://github.com/jonhoo/rust-evmap)
[sharded-slab: a lock-free concurrent slab](https://github.com/hawkw/sharded-slab)
[dashmap: Blazing fast concurrent HashMap for Rust.](https://github.com/xacrimon/dashmap)
https://github.com/vertexclique/lever  Transaction Systems and Data Grid

[bit-vec](https://github.com/contain-rs/bit-vec) - 89 stars, 21 contrib
[smallbitvec](https://github.com/servo/smallbitvec) - 27 stars, 10 contrib
[bitvec](https://github.com/bitvecto-rs/bitvec) - 405 stars, 16 contrib

[beef: Faster, compact implementation of std: :Cow](https://github.com/maciejhirsz/beef)
https://github.com/salsa-rs/salsa - computations...

## collections
https://github.com/bluss/indexmap - hashmap, сохраняет порядок вставки
https://github.com/jaemk/cached - мемоизирование функций
[jeromefroe/lru-rs: LRU cache](https://github.com/jeromefroe/lru-rs)
https://github.com/paritytech/trie - Base-16 Modified Merkle Tree
[slab: Slab allocator for Rust](https://github.com/carllerche/slab) - предварительно выделенное хранилище типа Vec<T>, но как использовать повторно?
[rustc-hash](https://lib.rs/crates/rustc-hash) - Like FNV, but hashing 8 bytes at a time on 64-bit platforms, where the FNV algorithm works on one byte at a time
https://crates.io/crates/typed-arena - аллокация памяти, без удаления отдельных элементов
[shared-arena](https://github.com/sebastiencs/shared-arena) - A thread-safe & efficient memory pool
https://crates.io/crates/cranelift-bforest - B-tree for small 32bit keys and values
