# in-container

`in-container` is a binary and a library that can be used to detect if you are running inside a container.
Executing the binary will by default return exit-code 0 if it was run inside a container and exit-code 1 if it wasn't.
The library can be included in an application of your choice, allowing you to determine whether your application is running inside a container or not.

(Please note that some of the detection mechanisms only work if `in-container` is executed in a privileged context.)

## Supported operating systems/containerization solutions

* FreeBSD
    * [Jails](https://www.freebsd.org/doc/handbook/jails.html)
* Linux
    * [Docker](https://docs.docker.com/engine/)
    * [LXC](https://linuxcontainers.org/)
    * [systemd-nspawn](https://www.freedesktop.org/software/systemd/man/systemd-nspawn.html)
* Windows
    * [Docker](https://docs.docker.com/docker-for-windows/install/)

If you are missing support for an operating system or container runtime, feel free to [open a feature request](https://github.com/pitkley/in-container/issues/new) or [open a pull request](https://github.com/pitkley/in-container/pull/compare).

## Usage as a library

Add `in-container` as a dependency to your project's `Cargo.toml`:

```toml
[dependencies]
in-container = "^1"
```

You can then use `in_container::in_container()` which will return `true` if you are running inside a container and `false` otherwise.
In case you are interested in the container-runtime that was detected, you can call `in_container::get_container_runtime()` instead, which will return an `Option<ContainerRuntime>`.
The `Option` is `None` when not running in a container, otherwise it will contain the detected runtime.

## <a name="versionbumppolicy"></a> Version bump policy

In general, the versioning scheme follows the semantic versioning guidelines:

* The patch version is bumped when backwards compatible fixes are made (this includes updates to dependencies).
* The minor version is bumped when new features are introduced, but backwards compatibility is retained.
* The major version is bumped when a backwards incompatible change was made.

Special case:

* A bump in the minimum supported Rust version (MSRV), which is currently 1.46.0, will be done in minor version updates (i.e. they do not require a major version bump).

## <a name="license"></a> License

This project is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

### <a name="license-contribution"></a> Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
