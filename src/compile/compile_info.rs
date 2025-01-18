use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CompileInfo {
    pub git_hash: String,
    pub rust_version: String,
    pub git_commit_time: String,
    pub build_time: String,
}

impl CompileInfo {
    pub fn load_from_str(compile_info: &str) -> Result<CompileInfo, String> {
        let info = serde_json::from_str(&compile_info);
        match info {
            Ok(info) => Ok(info),
            Err(err) => Err(format!("{:?}", err)),
        }
    }
    pub fn save_to_str(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
