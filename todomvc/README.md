Implementations of TodoMVC for using different frameworks and framework's
different features.

`todomvc_shared` crate implements the core functionality of the app.
An implementation is encourage (not mandatory) to use this crate.

If different implementations for the same framework can share common code,
then extract them to a `framework_name_shared`, similar to `spair_shared`.

Build

rustc 1.64.0 (a55dd71d5 2022-09-19)
trunk 0.16.0

trunk build --release

Results

| Implementations | Size (kb) |
|-----------------|-----------|
| spair keyed     | 202.1     |
| spair non keyed | 172.5     |
| yew non keyed   | 298.1     |
