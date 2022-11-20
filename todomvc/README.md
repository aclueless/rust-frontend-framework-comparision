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

All builds have these (you can see this in `Cargo.toml` at root)

    [profile.release]
    lto = true
    codegen-units = 1

[Here is the results for TodoMVC implementations](./results.md)

