// Copyright Pit Kleyersburg <pitkley@googlemail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified or distributed
// except according to those terms.

use super::ContainerRuntime;
use std::{
    collections::HashMap,
    convert,
    fs::File,
    io::{self, prelude::*, BufReader},
    path::Path,
};

pub(super) fn get_container_runtime() -> Option<ContainerRuntime> {
    chain! {
        docker::get_container_runtime,
        pid_1_environ,
    }
}

fn get_env_of_pid(pid: i32) -> io::Result<HashMap<String, String>> {
    let file = File::open(format!("/proc/{}/environ", pid))?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    Ok(contents
        .split('\0')
        .map(|key_value| {
            key_value.find('=').map(|separator_index| {
                let (key, value) = key_value.split_at(separator_index);
                (key.to_owned(), value[1..].to_owned())
            })
        })
        .filter_map(convert::identity)
        .collect())
}

mod docker {
    use super::*;

    pub(super) fn get_container_runtime() -> Option<ContainerRuntime> {
        chain! {
            dockerenv_exists,
            cgroup,
        }
    }

    pub fn dockerenv_exists() -> Option<ContainerRuntime> {
        println!("running dockerenv_exists");
        if Path::new("/.dockerenv").exists() {
            Some(ContainerRuntime::Docker)
        } else {
            None
        }
    }

    pub fn cgroup() -> Option<ContainerRuntime> {
        println!("running cgroup");
        fn inner() -> io::Result<bool> {
            let file = File::open("/proc/1/cgroup")?;
            let mut reader = BufReader::new(file);
            let mut contents = String::new();
            reader.read_to_string(&mut contents)?;

            Ok(contents.contains("/docker/"))
        }

        if inner().unwrap_or(false) {
            Some(ContainerRuntime::Docker)
        } else {
            None
        }
    }
}

fn pid_1_environ() -> Option<ContainerRuntime> {
    println!("running pid_1_environ");
    get_env_of_pid(1).ok().and_then(|environ| {
        environ
            .get("container")
            .and_then(|env_container| str::parse(env_container).ok())
    })
}
