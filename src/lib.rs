
//! satway_build_info
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
//! ```
//! use serde::{Deserialize, Serialize};
//! use std::process::Command;use satway_build::CompileInfo;
//!
//! let compile_info=CompileInfo::load_from_env();
//! let json_str=compile_info.save_to_str(true);
//! println!("{}",json_str);
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
