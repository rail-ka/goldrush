# Libraries

https://github.com/hyperium/hyper
https://github.com/seanmonstar/reqwest
https://github.com/tokio-rs/bytes?


**queue**
https://stackoverflow.com/questions/40848918/are-there-queue-and-stack-collections
https://github.com/crossbeam-rs/crossbeam - crossbeam-queue
[stjepang/concurrent-queue](https://github.com/stjepang/concurrent-queue)
[garro95/priority-queue](https://github.com/garro95/priority-queue)
github.com/kinghajj/deque

## Параллелизм/Конкуренция
github.com/carllerche/mio
[rayon-rs/rayon](https://github.com/rayon-rs/rayon) - параллельная работа над структурами данных
github.com/crossbeam-rs/crossbeam
github.com/alexcrichton/futures-rs
github.com/tokio-rs/tokio - 7,8k, 305 contrib
[async-rs/async-std](https://github.com/async-rs/async-std) - Tokio alternative, 2,1k, 91 contrib
https://github.com/stjepang/smol - Tokio alternative
[Amanieu/parking_lot](https://github.com/Amanieu/parking_lot) - Mutex
[jonhoo/rust-evmap: A lock-free, eventually consistent, concurrent multi-value map.](https://github.com/jonhoo/rust-evmap)
[hawkw/sharded-slab: a lock-free concurrent slab (experimental)](https://github.com/hawkw/sharded-slab)
[GitHub - xacrimon/dashmap: Blazing fast concurrent HashMap for Rust.](https://github.com/xacrimon/dashmap)
https://github.com/vertexclique/lever  Transaction Systems and Data Grid

[maciejhirsz/beef: Faster, compact implementation of std: :Cow](https://github.com/maciejhirsz/beef)
[GitHub - dzamlo/rust-bitfield](https://github.com/dzamlo/rust-bitfield)
[GitHub - BurntSushi/byteorder: Rust library for reading/writing numbers in big-endian and little-endian.](https://github.com/BurntSushi/byteorder)
[bitflags/bitflags ](https://github.com/bitflags/bitflags)
https://github.com/salsa-rs/salsa - computations...


## collections
https://github.com/bluss/indexmap - hashmap, сохраняет порядок вставки
https://github.com/jaemk/cached - мемоизирование функций
[jeromefroe/lru-rs: LRU cache](https://github.com/jeromefroe/lru-rs)
https://github.com/paritytech/trie - Base-16 Modified Merkle Tree
[GitHub - carllerche/slab: Slab allocator for Rust](https://github.com/carllerche/slab) - предварительно выделенное хранилище типа Vec<T>, но как использовать повторно?
[servo/rust-fnv](https://github.com/servo/rust-fnv) - FNV hash для ключей малого размера
[rustc-hash — Rust implementation // Lib.rs](https://lib.rs/crates/rustc-hash) - Like FNV, but hashing 8 bytes at a time on 64-bit platforms, where the FNV algorithm works on one byte at a time
https://crates.io/crates/typed-arena - аллокация памяти, без удаления отдельных элементов
[sebastiencs/shared-arena](https://github.com/sebastiencs/shared-arena) - A thread-safe & efficient memory pool
https://crates.io/crates/cranelift-bforest - B-tree for small 32bit keys and values

https://github.com/carllerche/iovec
