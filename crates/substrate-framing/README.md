# substrate-framing

The framing the Language Server and Debug Adapter protocols share: a
`Content-Length` header, a blank line, then that many bytes of JSON.

**Layer and context:** shared kernel · framing

- For: `write_frame` and `read_frame` over any `Write` and `BufRead`: a
  child's pipes, a socket.
- Not for: the protocols' messages and sessions (`substrate-lsp-app`,
  `substrate-dap-app`).

## Usage

```rust
use std::io::Cursor;

use serde_json::json;
use substrate_framing::{read_frame, write_frame};

let mut bytes = Vec::new();
write_frame(&mut bytes, &json!({ "method": "initialize" }))?;
let message = read_frame(&mut Cursor::new(bytes))?;
assert_eq!(message, Some(json!({ "method": "initialize" })));
# Ok::<(), std::io::Error>(())
```

## Depends on / used by

Depends on nothing in the workspace. Used by
[`substrate-lsp-app`](../substrate-lsp-app/README.md) and
[`substrate-dap-app`](../substrate-dap-app/README.md).

## License

AGPL-3.0-only; see [LICENSE](../../LICENSE).
