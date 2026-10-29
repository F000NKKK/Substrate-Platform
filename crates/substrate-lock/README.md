# substrate-lock

Locks that outlive a panic: `Locked` and `ReadWrite` give a `Mutex`'s or
`RwLock`'s guard whether or not a thread panicked while holding it.

**Layer and context:** shared kernel · lock

- For: state shared between threads that must go on after a panicking
  callback (a log, a registry of sessions): the data is as it last was.
- Not for: data a panic can leave half-written; use `lock()` and decide.

## Usage

```rust
use std::sync::Mutex;

use substrate_lock::Locked;

let counter = Mutex::new(1);
*counter.locked() += 1;
assert_eq!(*counter.locked(), 2);
assert_eq!(*counter.try_locked().unwrap(), 2);
```

## Depends on / used by

Depends on nothing in the workspace. Used by the contexts' app and adapter
crates, by [`tauri-plugin-substrate`](../../tauri-plugin/README.md) and by
products.

## License

AGPL-3.0-only or a separate commercial agreement; see [LICENSE](LICENSE)
and [COMMERCIAL-LICENSE.md](COMMERCIAL-LICENSE.md).
