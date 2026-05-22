### 🚦 svetofor
... is simple semaphore implementation in Rust using `parking_lot`

### 📦 Installation
Add this to your Cargo.toml:
```
[dependencies]
svetofor = "0.1.0"
```

### 🐝 Getting started
```rust
use svetofor::Semaphore;

fn main() {
  // creating new semaphore
  let sem = Semaphore::new(1);

  // acquiring guard
  let guard = sem.acquire();

  // while guard alive, counter must be 0
  assert_eq!(sem.counter(), 0);

  // dropping guard
  drop(guard);

  // after guard drop, counter must be 1
  assert_eq!(sem.counter(), 1);
}
```
