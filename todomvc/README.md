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

All builds have these (you can see this in `Cargo.toml` at root)

    [profile.release]
    lto = true
    codegen-units = 1

Size of the `.wasm` file, in kB:

| Implementations | opt-level = default | opt-level = "s" | opt-level = "z" |
|-----------------|---------------------|-----------------|-----------------|
| spair keyed     | 176.6               | 148.9           | 138.0           |
| spair non keyed | 146.6               | 132.8           | 125.5           |
| yew non keyed   | 249.8               | 203.8           | 189.7           |
