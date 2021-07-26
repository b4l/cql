# cql [![build badge](https://github.com/b4l/cql/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/b4l/cql/actions/workflows/rust.yml)

## Common Query Language (CQL)

The `cql` crate provides parsing of textual CQL queries as specified by the OGC and translating such queries into propper sql statements.

## Example

```rust
use cql;

let query = "landsat:scene_id = 'LC82030282019133LGN00'";
let sql = cql::to_sql(query).unwrap();

assert_eq!(sql, "\"landsat:scene_id\" = 'LC82030282019133LGN00'");
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.