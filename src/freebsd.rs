// Copyright Pit Kleyersburg <pitkley@googlemail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified or distributed
// except according to those terms.

use super::ContainerRuntime;

pub(super) fn get_container_runtime() -> Option<ContainerRuntime> {
    chain! {
        jail::get_container_runtime,
    }
}

mod jail {
    use super::*;
    use sysctl::{Ctl, CtlValue, Sysctl, SysctlError};

    pub(super) fn get_container_runtime() -> Option<ContainerRuntime> {
        chain! {
            sysctl_securityjailjailed,
        }
    }

    fn sysctl_securityjailjailed() -> Option<ContainerRuntime> {
        fn inner() -> Result<bool, SysctlError> {
            let ctl = Ctl::new("security.jail.jailed")?;
            Ok(match ctl.value()? {
                CtlValue::Int(1) => true,
                _ => false,
            })
        }

        if inner().unwrap_or(false) {
            Some(ContainerRuntime::Jail)
        } else {
            None
        }
    }
}
