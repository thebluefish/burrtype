# Sandbox

---

This example aims to demonstrate every feature available.

- [Types](src/lib.rs) for export are conveniently organized in one place, but can span multiple crates.
- [An Exporter](src/main.rs) writes these types [in the Client.](test-client/src/api)
- [The Server](test-server/src/main.rs) hosts an API covering each type.
- [The Client](test-client/src/index.ts) communicates with this API using the generated types.