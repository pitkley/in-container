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
        docker::in_container,
    }
}

mod docker {
    use super::*;

    pub(super) fn get_container_runtime() -> Option<ContainerRuntime> {
        chain! {
            winreg_containertype,
            service_cexecsvc,
        }
    }

    fn winreg_containertype() -> Option<ContainerRuntime> {
        use winreg::{enums::*, RegKey};

        RegKey::predef(HKEY_LOCAL_MACHINE)
            .open_subkey(r"SYSTEM\CurrentControlSet\Control")
            .and_then(|subkey| subkey.get_raw_value("ContainerType"))
            .ok()
            .map(|_| ContainerRuntime::Docker)
    }

    fn service_cexecsvc() -> Option<ContainerRuntime> {
        use windows_service::{
            service::ServiceAccess,
            service_manager::{ServiceManager, ServiceManagerAccess},
        };

        ServiceManager::local_computer(None::<&str>, ServiceManagerAccess::CONNECT)
            .and_then(|service_manager| {
                service_manager.open_service("cexecsvc", ServiceAccess::QUERY_STATUS)
            })
            .ok()
            .map(|_| ContainerRuntime::Docker)
    }
}
