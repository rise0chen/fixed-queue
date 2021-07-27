# Fixed-Queue

A heapless version of the Rust `Vec`, `VecDeque`.

no_std, no_alloc, use [T; N].

support `Vec`/`VecDeque`/`spsc`/`History`.

## Deprecated

Replaced by [heapless](https://docs.rs/heapless/0.7.3/heapless/).

## Usage

### Vec

```rust
use fixed_queue::Vec;

let mut vec: Vec<u8, 3> = Vec::new();
assert_eq!(vec.capacity(), 3);
assert!(vec.is_empty());
vec.push(1);
println!("{}", vec[0]);
println!("{:?}", vec.pop());
```

### VecDeque

```rust
use fixed_queue::VecDeque;

let mut vec: VecDeque<u8, 4> = VecDeque::new();
assert_eq!(vec.capacity(), 3);
assert!(vec.is_empty());
vec.push_back(1);
println!("{}", vec[0]);
vec.push_front(2);
println!("{}", vec[1]);
println!("{:?}", vec.pop_back());
println!("{:?}", vec.pop_front());
```

### SPSC

```rust
use fixed_queue::Spsc;
static SPSC: Spsc<u8, 4> = Spsc::new();

let sender = SPSC.take_sender().unwrap();
let recver = SPSC.take_recver().unwrap();

assert_eq!(SPSC.capacity(), 3);
assert!(sender.send(1).is_ok());
assert!(sender.send(2).is_ok());
assert!(sender.send(3).is_ok());
assert!(sender.send(4).is_err());
```

### History

```rust
use fixed_queue::History;
static HISTORY: History<u8, 3> = History::new();

assert!(HISTORY.insert(1));
assert!(HISTORY.contains(&1));
```
