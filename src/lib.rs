// Copyright Pit Kleyersburg <pitkley@googlemail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified or distributed
// except according to those terms.

#![forbid(missing_docs, unsafe_code)]

//! # in-container
//!
//! `in-container` is a binary and a library that can be used to detect if you are running inside a
//! container. Executing the binary will by default return exit-code 0 if it was run inside a
//! container and exit-code 1 if it wasn't. The library can be included in an application of your
//! choice, allowing you to determine whether your application is running inside a container or not.
//!
//! (Please note that some of the detection mechanisms only work if `in-container` is executed in a
//! privileged context.)
//!
//! ## Supported operating systems/containerization solutions
//!
//! * FreeBSD
//!     * [Jails](https://www.freebsd.org/doc/handbook/jails.html)
//! * Linux
//!     * [Docker](https://docs.docker.com/engine/)
//!     * [LXC](https://linuxcontainers.org/)
//!     * [systemd-nspawn](https://www.freedesktop.org/software/systemd/man/systemd-nspawn.html)
//! * Windows
//!     * [Docker](https://docs.docker.com/docker-for-windows/install/)
//!
//! If you are missing support for an operating system or container runtime, feel free to [open a
//! feature request](https://github.com/pitkley/in-container/issues/new) or
//! [open a pull request](https://github.com/pitkley/in-container/pull/compare).
//!
//! ## Usage as a library
//!
//! Add `in-container` as a dependency to your project's `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! in-container = "^1"
//! ```
//!
//! You can then use `in_container::in_container()` which will return `true` if you are running
//! inside a container and `false` otherwise. In case you are interested in the container-runtime
//! that was detected, you can call `in_container::get_container_runtime()` instead, which will
//! return an `Option<ContainerRuntime>`. The `Option` is `None` when not running in a container,
//! otherwise it will contain the detected runtime.
//!
//! ## <a name="license"></a> License
//!
//! This project is licensed under either of
//!
//! * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
//! <https://www.apache.org/licenses/LICENSE-2.0>)
//! * MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)
//!
//! at your option.
//!
//! ### <a name="license-contribution"></a> Contribution
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
//! this project by you, as defined in the Apache-2.0 license, shall be dual licensed as above,
//! without any additional terms or conditions.

use std::{fmt::Display, str::FromStr};

/// Returns `true` if called from inside a container, `false` otherwise.
pub fn in_container() -> bool {
    get_container_runtime().is_some()
}

/// Optionally returns the detected [`ContainerRuntime`] if called from inside a container.
///
/// [`ContainerRuntime`]: enum.ContainerRuntime.html
pub fn get_container_runtime() -> Option<ContainerRuntime> {
    #[cfg(target_os = "freebsd")]
    return freebsd::get_container_runtime();
    #[cfg(target_os = "linux")]
    return linux::get_container_runtime();
    #[cfg(target_os = "windows")]
    return windows::get_container_runtime();
}

/// The detected container runtime.
#[derive(Debug, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ContainerRuntime {
    /// Docker container runtime
    Docker,
    /// BSD jail
    Jail,
    /// Linux Containers
    Lxc,
    /// systemd-nspawn
    SystemdNspawn,
    /// The detected container runtime is unknown
    Unknown(String),
}

impl Display for ContainerRuntime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContainerRuntime::Docker => write!(f, "docker"),
            ContainerRuntime::Jail => write!(f, "jail"),
            ContainerRuntime::Lxc => write!(f, "lxc"),
            ContainerRuntime::SystemdNspawn => write!(f, "systemd-nspawn"),
            ContainerRuntime::Unknown(name) => write!(f, "unknown({})", name),
        }
    }
}

impl From<&str> for ContainerRuntime {
    fn from(s: &str) -> Self {
        match s {
            "docker" => Self::Docker,
            "jail" => Self::Jail,
            "lxc" => Self::Lxc,
            "systemd-nspawn" => Self::SystemdNspawn,
            name => Self::Unknown(name.to_owned()),
        }
    }
}

impl FromStr for ContainerRuntime {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

macro_rules! chain {
    ( $fn:path $(,)+ $( $tail:path $(,)* )* ) => {
        $fn().or_else(|| chain!( $($tail , )* ))
    };
    ( $fn:path ) => {
        $fn()
    };
    () => { None};
}

#[cfg(target_os = "freebsd")]
mod freebsd;
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn containerruntime_from_str() {
        assert_eq!(ContainerRuntime::Docker, "docker".into());
        assert_eq!(ContainerRuntime::Jail, "jail".into());
        assert_eq!(ContainerRuntime::Lxc, "lxc".into());
        assert_eq!(ContainerRuntime::SystemdNspawn, "systemd-nspawn".into());
        assert_eq!(
            ContainerRuntime::Unknown("garbage".to_owned()),
            "garbage".into()
        );
        assert_eq!(ContainerRuntime::Unknown("".to_owned()), "".into());
    }
}
