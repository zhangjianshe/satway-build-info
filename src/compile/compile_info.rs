use std::process::Command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CompileInfo {
    pub git_commit_email: String,
    pub git_commit_hash: String,
    pub git_commit_message:String,
    pub git_commit_time: String,
    pub rust_version: String,
    pub build_time: String,
    pub build_os:String
}

impl CompileInfo {
    /// load from string return a new CompileInfo object.
    ///
    pub fn load_from_str(compile_info: &str) -> Result<CompileInfo, String> {
        let info = serde_json::from_str(&compile_info);
        match info {
            Ok(info) => Ok(info),
            Err(err) => Err(format!("{:?}", err)),
        }
    }
    /// save this object to a string
    ///
    pub fn save_to_str(&self,pretty:bool) -> String {
        if  pretty{
            format!("{}", serde_json::to_string_pretty(&self).unwrap())
        }
        else {
            serde_json::to_string(&self).unwrap()
        }
    }

    /// execute some command from the environments
    pub fn load_from_env()->Self{
        let output = Command::new("git")
            .args(&["log", "HEAD","--format=%h%n%s%n%ce%n%ci"])
            .output();
      let output=  match  output{
            Ok(output) => output,
            Err(err) => panic!("Failed to execute git log command: {:?}", err),
        };

        let git_log = String::from_utf8(output.stdout)
            .expect("Failed to convert git log output to string");
        let lines=git_log.lines().collect::<Vec<&str>>();

        let build_os=if cfg!(target_os = "windows") {
            "windows"
        } else if cfg!(target_os = "linux") {
            "linux"
        } else { "unknown" };

        let rustc_cmd=Command::new("rustc").args(&["--version"]).output().expect("Failed to execute rustc");
        let rustc_output=String::from_utf8(rustc_cmd.stdout).expect("Failed to convert rustc output to string");

        CompileInfo{
            git_commit_email:lines[2].to_string(),
            git_commit_hash: lines[0].to_string(),
            git_commit_message: lines[1].to_string(),
            git_commit_time: lines[3].to_string(),
            rust_version:rustc_output.trim().to_string(),
            build_time: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            build_os:build_os.to_string()
        }
    }
}
