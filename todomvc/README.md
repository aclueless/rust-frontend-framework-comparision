Implementations of TodoMVC for using different frameworks and framework's
different features.

`todomvc_shared` crate implements the core functionality of the app.
An implementation is encourage (not mandatory) to use this crate.

If different implementations for the same framework can share common code,
then extract them to a `framework_name_shared`, similar to `spair_shared`.

Currently, only implementations for Yew and Spair use `todomvc_shared`.
Other implementations do not make use of `todomvc_shared`, yet, because
I do not have enough time and not enough knowledge about those frameworks.

I choose to include these frameworks:
    * Yew, because I believe it is the most popular.
    * Dominator and Sycamore, because these have comparable performance
    compare to Spair
    * Spair, because it is my own framework :D

If you want to include implementations for other frameworks or to improve
current implementations, PRs are welcome.

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
| dominator       | 213.9               | 178.6           | 174.9           |
| spair keyed     | 176.6               | 148.9           | 138.0           |
| spair non keyed | 146.6               | 132.8           | 125.5           |
| sycamore        | 268.2               | 252.1           | 259.9 (!z)      |
| yew non keyed   | 249.8               | 203.8           | 189.7           |

* _!z_: opt-level = "z" produce a bigger size.

Note: The results above is manually collected (mistakes have high chance to
occur). TODO: Automatically build and collect the results by some tools.
