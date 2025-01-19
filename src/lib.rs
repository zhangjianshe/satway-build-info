
//! satway_build
//!
//! This crate collects building information from all kinds of environments in to an object [CompileInfo]
//!
//! generally, this is used in the build.rs file
//! when build project,we write the information to a file like 'src/context/compile_info.txt'
//! then we can use it in our program with
//! CompileInfo::load_from_str(!include("/src/context/compile_info.txt"));
//!
//! CompileInfo serialize and Deserialize by serde_json library.
//!
//!
//! #[example]
//!
//! build.rs  write build information to a file
//! ```
//!
//! use satway_build::CompileInfo;
//! use std::fmt::Debug;
//! use std::path::Path;
//! use std::{env, fs};
//!
//! pub fn main() {
//!
//!     println!("cargo:rerun-if-changed=build.rs");
//!     let compile_info: CompileInfo = CompileInfo::load_from_env();
//!
//!     let out_file_path = Path::new(&env::var("OUT_DIR").unwrap()).join("compile_info.txt");
//!     println!("cargo:rustc-env=COMPILE_INFO_FILE={}", out_file_path.as_path().to_str().unwrap());
//!     fs::write(out_file_path, compile_info.save_to_str(true)).expect("Unable to write file");
//! }
//! ```
//!
//! use it in an api server
//! ```
//! use satway_build::CompileInfo;
//!
//! pub fn version_info() {
//!     let json = include_str!(env!("COMPILE_INFO_FILE"));
//!     let compile_info = CompileInfo::load_from_str(json);
//!     println!("{:#?}", compile_info);
//! }
//!
//! ```
//!
//!
mod compile;
pub use compile::CompileInfo;

#[cfg(test)]
mod tests {
    use crate::CompileInfo;

    #[test]
    fn test_build(){
        let compile_info=CompileInfo::load_from_env();
        let json_str=compile_info.save_to_str(true);
        println!("{}",json_str);
    }
}
