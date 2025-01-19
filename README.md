# satway-build
  provide build information for rust application

## information
![github build](https://github.com/zhangjianshe/satway-build-info/actions/workflows/rust.yml/badge.svg)

Current Version=0.1.14

## usage

```shell
    cargo add satway_build 
```

 This crate collects building information from all kinds of environments in to an object [CompileInfo]
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
 when build project,we write the information to a file like 'src/context/compile_info.txt'
 then we can use it in our program with
 CompileInfo::load_from_str(!include("/src/context/compile_info.txt"));

 CompileInfo serialize and Deserialize by serde_json library.

the following code is build.rs,which will write the compile information to src/context/info.txt
```rust
use std::fmt::{format, Debug};
use std::fs;
use satway_build::CompileInfo;

pub fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let compile_info:CompileInfo=CompileInfo::load_from_env();
    fs::write("src/context/info.txt",compile_info.save_to_str(true)).expect("Unable to write file");
}
```

the following function will load the compile information from src/context/info.txt
```rust
use satway_build::CompileInfo;
fn main(){
    let compile_info=CompileInfo::load_from_str(!include("/src/context/info.txt"));
    println!("{}",compile_info);
}
``` 


## dev: how to release a new version
```shell
  # this command will update the version in Cargo.toml and Cargo.lock
  # and push the new version to github
  # in github action's will trigger a tag action to publish this crate to crates.io
  ./release.sh "VERSION"
```