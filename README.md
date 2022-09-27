Comparision for different frontend frameworks (for Rust-lang) using
different features.

The implementations can use different framework features. But things
that are not framework-related should be the same. Such as, all
implementations should use the same type for item's id, or use the
same third party crates.

All builds will use default config. But it is better if we can
make it possible to run the build with different build-config to
provide more info for better comparisions.

Currently there are only implementations of Spair, Yew for TodoMVC app.
