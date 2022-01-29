# Changelog

<!-- next-header -->

## Unreleased

* Support building on all platforms, not just FreeBSD, Linux and Windows.

    `in_container::get_container_runtime()` will always return `None`, `in_container::in_container()` always `false`.

* Minimum supported Rust version (MSRV) bumped to 1.52.1 for the library and to 1.54.0 for the binary.

## 1.0.0 (2020-07-17)

Initial release supporting Docker, Jails, LXC, and systemd-nspawn on FreeBSD, Linux and Windows.
Includes both a binary which you can simly drop into your environment, but is also a library that you can consum from other Rust applications or libraries.
