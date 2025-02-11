//! I'm tired of guessing. I use Rust for its safety, why don't the build scripts give me that?
//!
//! For the best experience, alias this crate to `cargo` when you use it to more closesly match the actual expressions.
//! ```
//! use build_rs_macros as cargo;
//!
//! fn main() {
//!   println!(cargo::rerun_if_changed("Cargo.toml"));
//! }
//! ```
//!
//! [`rerun_if_changed`]
//! [`cargo::rerun-if-changed=PATH`](https://docs.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed)
//! ---
//! Tells Cargo when to re-run the script.
//!
//! [`rerun_if_env_changed`]
//! [`cargo::rerun-if-env-changed=VAR`](https://docs.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-env-changed)
//! ---
//! Tells Cargo when to re-run the script.
//!
//! [`rustc_link_arg`]
//! [`cargo::rustc-link-arg=FLAG`](https://docs.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg)
//! ---
//! Passes custom flags to a linker for benchmarks, binaries, cdylib crates, examples, and tests.
//!
//! [`rustc_link_arg_bin`]
//! [`cargo::rustc-link-arg-bin=BIN=FLAG`](https://docs.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg-bin)
//! ---
//! Passes custom flags to a linker for the binary BIN.
//!
//! [`rustc_link_arg_bins`]
//! [`cargo::rustc-link-arg-bins=FLAG`](https://docs.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg-bins)
//! ---
//! Passes custom flags to a linker for binaries.
//!
//! [`rustc_link_arg_tests`]
//! [`cargo::rustc-link-arg-tests=FLAG`](https://docs.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg-tests)
//! ---
//! Passes custom flags to a linker for tests.
//!
//! [`rustc_link_arg_examples`]
//! [`cargo::rustc-link-arg-examples=FLAG`](https://docs.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg-examples)
//! ---
//! Passes custom flags to a linker for examples.
//!
//! [`rustc_link_arg_benches`]
//! [`cargo::rustc-link-arg-benches=FLAG`](https://docs.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg-benches)
//! ---
//! Passes custom flags to a linker for benchmarks.
//!
//! [`rustc_link_lib`]
//! [`cargo::rustc-link-lib=LIB`](https://docs.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-lib)
//! ---
//! Adds a library to link.
//!
//! [`rustc_link_search`]
//! [`cargo::rustc-link-search=[KIND=]PATH`](https://docs.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-search)
//! ---
//! Adds to the library search path.
//!
//! [`rustc_flags`]
//! [`cargo::rustc-flags=FLAGS`](https://docs.rust-lang.org/cargo/reference/build-scripts.html#rustc-flags)
//! ---
//! Passes certain flags to the compiler.
//!
//! [`rustc_cfg`]
//! [`cargo::rustc-cfg=KEY[="VALUE"]`](https://docs.rust-lang.org/cargo/reference/build-scripts.html#rustc-cfg)
//! ---
//! Enables compile-time cfg settings.
//!
//! [`rustc_check_cfg`]
//! [`cargo::rustc-check-cfg=CHECK_CFG`](https://docs.rust-lang.org/cargo/reference/build-scripts.html#rustc-check-cfg)
//! ---
//! Register custom cfgs as expected for compile-time checking of configs.
//!
//! [`rustc_env`]
//! [`cargo::rustc-env=VAR=VALUE`](https://docs.rust-lang.org/cargo/reference/build-scripts.html#rustc-env)
//! ---
//! Sets an environment variable.
//!
//! [`rustc_cdylib_link_arg`]
//! [`cargo::rustc-cdylib-link-arg=FLAG`](https://docs.rust-lang.org/cargo/reference/build-scripts.html#rustc-cdylib-link-arg)
//! ---
//! Passes custom flags to a linker for cdylib crates.
//!
//! [`error`]
//! [`cargo::error=MESSAGE`](https://docs.rust-lang.org/cargo/reference/build-scripts.html#error)
//! ---
//! Displays an error on the terminal.
//!
//! [`warning`]
//! [`cargo::warning=MESSAGE`](https://docs.rust-lang.org/cargo/reference/build-scripts.html#warning)
//! ---
//! Displays a warning on the terminal.
//!
//! [`metadata`]
//! [`cargo::metadata=KEY=VALUE`](https://docs.rust-lang.org/cargo/reference/build-scripts.html#metadata)
//! ---
//! Metadata, used by links scripts.


/// ## [`cargo::rerun-if-changed=PATH`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed)
///
/// The `rerun-if-changed` instruction tells Cargo to re-run the build script if the file at the given path has changed.
/// Currently, Cargo only uses the filesystem last-modified "mtime" timestamp to determine if the file has changed.
/// It compares against an internal cached timestamp of when the build script last ran.
///
/// If the path points to a directory, it will scan the entire directory for any modifications.
///
/// If the build script inherently does not need to re-run under any circumstance, then emitting
/// `cargo::rerun-if-changed=build.rs` is a simple way to prevent it from being re-run (otherwise, the default if no
/// `rerun-if` instructions are emitted is to scan the entire package directory for changes). Cargo automatically
/// handles whether or not the script itself needs to be recompiled, and of course the script will be re-run after it
/// has been recompiled. Otherwise, specifying `build.rs` is redundant and unnecessary.
#[macro_export]
macro_rules! rerun_if_changed {
    ($path:expr $(,)?) => {
        _ = include_bytes!($path); // ensure path exists
        println!(concat!("cargo::rerun-if-changed=", $path));
    };
}

/// ## [`cargo::rerun-if-env-changed=NAME`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-env-changed)
///
/// The `rerun-if-env-changed` instruction tells Cargo to re-run the build script if the value of an environment variable of the given name has changed.
///
/// Note that the environment variables here are intended for global environment variables like CC and such, it is not possible to use this for environment variables like TARGET that Cargo sets for build scripts. The environment variables in use are those received by cargo invocations, not those received by the executable of the build script.
#[macro_export]
macro_rules! rerun_if_env_changed {
    ($var:literal $(,)?) => {
        concat!("cargo::rerun-if-env-changed=", $var)
    };
}

/// ## [`cargo::rustc-link-arg=FLAG`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg)
///
/// The `rustc-link-arg` instruction tells Cargo to pass the [`-C link-arg=FLAG` option](https://doc.rust-lang.org/rustc/codegen-options/index.html#link-arg) to the compiler, but only when building supported targets (benchmarks, binaries, cdylib crates, examples, and tests). Its usage is highly platform specific. It is useful to set the shared library version or linker script.
#[macro_export]
macro_rules! rustc_link_arg {
    ($flag:literal $(,)?) => {
        concat!("cargo::rustc-link-arg=", $flag)
    };
}

/// ## [`cargo::rustc-link-arg-bin=BIN=FLAG`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg-bin)
///
/// The `rustc-link-arg-bin` instruction tells Cargo to pass the [`-C link-arg=FLAG` option](https://doc.rust-lang.org/rustc/codegen-options/index.html#link-arg) to the compiler, but only when building the binary target with name BIN. Its usage is highly platform specific. It is useful to set a linker script or other linker options.
#[macro_export]
macro_rules! rustc_link_arg_bin {
    ($bin:literal = $flag:literal $(,)?) => {
        concat!("cargo::rustc-link-arg-bin=", $bin, "=", $flag)
    };
}

/// ## [`cargo::rustc-link-arg-bins=FLAG`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg-bins)
///
/// The `rustc-link-arg-bins` instruction tells Cargo to pass the [`-C link-arg=FLAG` option](https://doc.rust-lang.org/rustc/codegen-options/index.html#link-arg) to the compiler, but only when building a binary target. Its usage is highly platform specific. It is useful to set a linker script or other linker options.
#[macro_export]
macro_rules! rustc_link_arg_bins {
    ($flag:literal $(,)?) => {
        concat!("cargo::rustc-link-arg-bins=", $flag)
    };
}

/// ## [`cargo::rustc-link-arg-tests=FLAG`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg-tests)
///
/// The `rustc-link-arg-tests` instruction tells Cargo to pass the [`-C link-arg=FLAG` option](https://doc.rust-lang.org/rustc/codegen-options/index.html#link-arg) to the compiler, but only when building a tests target.
#[macro_export]
macro_rules! rustc_link_arg_tests {
    ($flag:literal $(,)?) => {
        concat!("cargo::rustc-link-arg-tests=", $flag)
    };
}

/// ## [`cargo::rustc-link-arg-examples=FLAG`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg-examples)
///
/// The `rustc-link-arg-examples` instruction tells Cargo to pass the [`-C link-arg=FLAG` option](https://doc.rust-lang.org/rustc/codegen-options/index.html#link-arg) to the compiler, but only when building an examples target.
#[macro_export]
macro_rules! rustc_link_arg_examples {
    ($flag:literal $(,)?) => {
        concat!("cargo::rustc-link-arg-examples=", $flag)
    };
}

/// ## [`cargo::rustc-link-arg-benches=FLAG`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-arg-benches)
///
/// The `rustc-link-arg-benches` instruction tells Cargo to pass the [`-C link-arg=FLAG` option](https://doc.rust-lang.org/rustc/codegen-options/index.html#link-arg) to the compiler, but only when building a benchmark target.
#[macro_export]
macro_rules! rustc_link_arg_benches {
    ($flag:literal $(,)?) => {
        concat!("cargo::rustc-link-arg-benches=", $flag)
    };
}

/// ## [`cargo::rustc-link-lib=LIB`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-lib)
///
/// The `rustc-link-lib` instruction tells Cargo to link the given library using the compiler’s [`-l` flag]. This is typically used to link a native library using [FFI].
///
/// The LIB string is passed directly to rustc, so it supports any syntax that `-l` does.
/// Currently the fully supported syntax for `LIB` is `[KIND[:MODIFIERS]=]NAME[:RENAME]`.
///
/// The `-l` flag is only passed to the library target of the package, unless there is no library target, in which case it is passed to all targets. This is done because all other targets have an implicit dependency on the library target, and the given library to link should only be included once. This means that if a package has both a library and a binary target, the library has access to the symbols from the given lib, and the binary should access them through the library target’s public API.
///
/// The optional `KIND` may be one of `dylib`, `static`, or `framework`. See the [rustc book] for more detail.
#[macro_export]
macro_rules! rustc_link_lib {
    ($lib:literal $(,)?) => {
        concat!("cargo::rustc-link-lib=", $lib)
    };
}

/// ## [`cargo::rustc-link-search=[KIND=]PATH`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-search)
///
/// The `rustc-link-search` instruction tells Cargo to pass the `-L` flag to the compiler to add a directory to the library search path.
///
/// The optional `KIND` may be one of dependency, crate, native, framework, or all. See the [rustc book] for more detail.
///
/// These paths are also added to the dynamic library search path environment variable if they are within the `OUT_DIR`. Depending on this behavior is discouraged since this makes it difficult to use the resulting binary. In general, it is best to avoid creating dynamic libraries in a build script (using existing system libraries is fine).
#[macro_export]
macro_rules! rustc_link_search {
    ($kind:literal = $path:expr $(,)?) => {
        concat!("cargo::rustc-link-search=", $lib)
    };

    ($path:expr $(,)?) => {
        concat!("cargo::rustc-link-search=", $lib)
    };
}

/// ## [`cargo::rustc-flags=FLAGS`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-flags)
///
/// The `rustc-flags` instruction tells Cargo to pass the given space-separated flags to the compiler. This only allows the `-l` and `-L` flags, and is equivalent to using rustc-link-lib and rustc-link-search.
#[macro_export]
macro_rules! rustc_flags {
    (($flags:literal),+ $(,)?) => {
        concat!("cargo::rustc-flags=", $($flags, " ")+)
    };
}

/// ## [`cargo::rustc-cfg=KEY[="VALUE"]`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-cfg)
///
/// The `rustc-cfg` instruction tells Cargo to pass the given value to the --cfg flag to the compiler. This may be used for compile-time detection of features to enable conditional compilation. Custom cfgs must either be expected using the cargo::rustc-check-cfg instruction or usage will need to allow the unexpected_cfgs lint to avoid unexpected cfgs warnings.
///
/// Note that this does not affect Cargo’s dependency resolution. This cannot be used to enable an optional dependency, or enable other Cargo features.
///
/// Be aware that Cargo features use the form feature="foo". cfg values passed with this flag are not restricted to that form, and may provide just a single identifier, or any arbitrary key/value pair. For example, emitting cargo::rustc-cfg=abc will then allow code to use #[cfg(abc)] (note the lack of feature=). Or an arbitrary key/value pair may be used with an = symbol like cargo::rustc-cfg=my_component="foo". The key should be a Rust identifier, the value should be a string.
#[macro_export]
macro_rules! rustc_cfg {
    ($key:ident $(,)?) => {
        concat!("cargo::rustc-cfg=", stringify!($key))
    };

    ($key:ident = $value:literal $(,)?) => {
        concat!("cargo::rustc-cfg=", stringify!($key), "=", stringify!($value))
    };
}

/// ## [`cargo::rustc-check-cfg=CHECK_CFG`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-check-cfg)
///
/// Add to the list of expected config names and values that is used when checking the reachable cfg expressions with the unexpected_cfgs lint.
///
/// The syntax of CHECK_CFG mirrors the rustc --check-cfg flag, see Checking conditional configurations for more details.
///
/// The instruction can be used like this:
///
/// ```
/// # use build_rs_macros as cargo;
/// // build.rs
/// # let foo_bar_condition: bool = true;
/// println!(cargo::rustc_check_cfg!(cfg(foo = values("bar"))));
/// if foo_bar_condition {
///     println!(cargo::rustc_cfg!(foo="bar"));
/// }
/// ```
///
/// Note that all possible cfgs should be defined, regardless of which cfgs are currently enabled. This includes all possible values of a given cfg name.
///
/// It is recommended to group the cargo::rustc-check-cfg and cargo::rustc-cfg instructions as closely as possible in order to avoid typos, missing check-cfg, stale cfgs…
///
/// See also the conditional compilation example.
#[macro_export]
macro_rules! rustc_check_cfg {
    (cfg($($name:ident),* $(,)? = values(any() $(,)?) $(,)?) $(,)?) => {
        concat!("cargo::rustc-check-cfg=cfg(", $(stringify!($name), ",",)* "values(any()))")
    };

    (cfg($($name:ident),* $(,)? = values(none() $(, $values:literal)* $(,)?) $(,)?) $(,)?) => {
        concat!("cargo::rustc-check-cfg=cfg(", $(stringify!($name), ",",)* "values(none(),", $(stringify!($values),)* "))")
    };

    (cfg($($name:ident),* $(,)? = values($($values:literal),* $(,)?) $(,)?) $(,)?) => {
        concat!("cargo::rustc-check-cfg=cfg(", $(stringify!($name), ",",)* "values(", stringify!($value1), $(",", stringify!($values),)* "))")
    };

    (cfg($($name:ident),* $(,)?) $(,)?) => {
        concat!("cargo::rustc-check-cfg=cfg(", $(stringify!($name), ",",)* ")")
    };
}

/// ## [`cargo::rustc-env=VAR=VALUE`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-env)
///
/// The `rustc-env` instruction tells Cargo to set the given environment variable when compiling the package. The value can be then retrieved by the env! macro in the compiled crate. This is useful for embedding additional metadata in crate’s code, such as the hash of git HEAD or the unique identifier of a continuous integration server.
///
/// See also the environment variables automatically included by Cargo.
///
/// > Note: These environment variables are also set when running an executable with cargo run or cargo test. However, this usage is discouraged since it ties the executable to Cargo’s execution environment. Normally, these environment variables should only be checked at compile-time with the env! macro.
#[macro_export]
macro_rules! rustc_env {
    ($var:ident = $value:literal $(,)?) => {
        concat!("cargo::rustc-env=", stringify!($var), stringify!($value))
    };
}

/// ## [`cargo::rustc-cdylib-link-arg=FLAG`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-cdylib-link-arg)
///
/// The `rustc-cdylib-link-arg` instruction tells Cargo to pass the [`-C link-arg=FLAG` option](https://doc.rust-lang.org/rustc/codegen-options/index.html#link-arg) to the compiler, but only when building a cdylib library target. Its usage is highly platform specific. It is useful to set the shared library version or the runtime-path.
#[macro_export]
macro_rules! rustc_cdylib_link_arg {
    ($flag:literal $(,)?) => {
        concat!("cargo::rustc-cdylib-link-arg=", stringify!($flag))
    };
}

/// ## [`cargo::error=MESSAGE`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#error)
///
/// The `error` instruction tells Cargo to display an error after the build script has finished running, and then fail the build.
///
/// > Note: Build script libraries should carefully consider if they want to use cargo::error versus returning a Result. It may be better to return a Result, and allow the caller to decide if the error is fatal or not. The caller can then decide whether or not to display the Err variant using cargo::error.
#[macro_export]
macro_rules! error {
    ($message:literal $(,)?) => {
        concat!("cargo::error=", $message)
    };
}

/// ## [`cargo::warning=MESSAGE`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#warning)
///
/// The `warning` instruction tells Cargo to display a warning after the build script has finished running. Warnings are only shown for path dependencies (that is, those you’re working on locally), so for example warnings printed out in crates.io crates are not emitted by default, unless the build fails. The -vv “very verbose” flag may be used to have Cargo display warnings for all crates.
#[macro_export]
macro_rules! warning {
    ($message:literal $(,)?) => {
        concat!("cargo::warning=", $message)
    };
}

/// # The links Manifest Key
///
/// The package.links key may be set in the Cargo.toml manifest to declare that the package links with the given native library. The purpose of this manifest key is to give Cargo an understanding about the set of native dependencies that a package has, as well as providing a principled system of passing metadata between package build scripts.
///
/// ```toml
/// [package]
/// # ...
/// links = "foo"
/// ```
///
/// This manifest states that the package links to the libfoo native library. When using the links key, the package must have a build script, and the build script should use the rustc-link-lib instruction to link the library.
///
/// Primarily, Cargo requires that there is at most one package per links value. In other words, it is forbidden to have two packages link to the same native library. This helps prevent duplicate symbols between crates. Note, however, that there are conventions in place to alleviate this.
///
/// Build scripts can generate an arbitrary set of metadata in the form of key-value pairs. This metadata is set with the cargo::metadata=KEY=VALUE instruction.
///
/// The metadata is passed to the build scripts of dependent packages. For example, if the package bar depends on foo, then if foo generates key=value as part of its build script metadata, then the build script of bar will have the environment variables DEP_FOO_KEY=value. See the “Using another sys crate” for an example of how this can be used.
///
/// Note that metadata is only passed to immediate dependents, not transitive dependents.
///
/// > MSRV: 1.77 is required for cargo::metadata=KEY=VALUE. To support older versions, use cargo:KEY=VAUE (unsupported directives are assumed to be metadata keys).
#[macro_export]
macro_rules! metadata {
    ($key:literal = $value:literal $(,)?) => {
        concat!("cargo::metadata=", $key, "=", $value)
    };
}
