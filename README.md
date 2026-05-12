# regex-pool

`regex-pool` is a minimal crate containing the reusable pool implementation
from [`rust-lang/regex`](https://github.com/rust-lang/regex).

This repository is a fork/copy of that pool code with a few minor local changes.
It is intended for consumers that only need:

```rust
use regex_pool::util::pool::{Pool, PoolGuard};
```
