![github build](https://github.com/zhangjianshe/satway-build-info/actions/workflows/rust.yml/badge.svg)
![github.com](https://img.shields.io/github/license/zhangjianshe/satway-build-info)
![Crates.io Total Downloads](https://img.shields.io/crates/d/satway_build)
[![Documentation](https://docs.rs/satway_build/badge.svg)](https://docs.rs/crate/satway_build/latest)


<div align="center">
  <h1>satway-build</h1>
  <p>
    <strong>provide build information for rust application</strong>
  </p>
</div>

Current Version=0.1.21

## usage

```shell
    cargo add satway_build@0.1.21
```

 This crate collects building information from all kinds of environments into an object [CompileInfo]
 which can be used in the program to get the building information. 
![information flow](doc/information_flow.png)
 
 current collect information like this
```json
{
  "gitCommitEmail": "zhangjianshe@gmail.com",
  "gitCommitHash": "731b4ce",
  "gitCommitMessage": "release version: 0.1.14",
  "gitCommitTime": "2025-01-19 13:29:40 +0800",
  "rustVersion": "rustc 1.83.0 (90b35a623 2024-11-26)",
  "buildTime": "2025-01-19 05:32:42",
  "buildOs": "linux"
}
```
 generally, this is used in the build.rs file
 when build project,we write the information to a file like 'OUT_DIR/compile_info.txt'
 then we can use it in our program with
 CompileInfo::load_from_str(!include("OUT_DIR/compile_info.txt"));

 CompileInfo serialize and Deserialize by serde_json library.

the following code is build.rs,which will write the compile information to `OUT_DIR/compile_info.txt`
and set env COMPILE_INFO_FILE=`OUT_DIR/compile_info.txt`
```rust
use satway_build::CompileInfo;
use std::fmt::Debug;
use std::path::Path;
use std::{env, fs};

pub fn main() {

    println!("cargo:rerun-if-changed=build.rs");
    let compile_info: CompileInfo = CompileInfo::load_from_env();

    let out_file_path = Path::new(&env::var("OUT_DIR").unwrap()).join("compile_info.txt");
    println!("cargo:rustc-env=COMPILE_INFO_FILE={}", out_file_path.as_path().to_str().unwrap());
    fs::write(out_file_path, compile_info.save_to_str(true)).expect("Unable to write file");
}
```
the following function will load the compile information from env COMPILE_INFO_FILE's file
```rust
use satway_build::CompileInfo;

pub fn version_info() {
    let json = include_str!(env!("COMPILE_INFO_FILE"));
    let compile_info = CompileInfo::load_from_str(json);
    println!("{:#?}", compile_info);
}
``` 


## dev: how to release a new version
```shell
  # this command will update the version in Cargo.toml and Cargo.lock
  # and push the new version to github
  # in github action's will trigger a tag action to publish this crate to crates.io
  ./release.sh "VERSION"
```