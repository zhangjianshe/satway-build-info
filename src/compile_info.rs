use serde::{Deserialize, Serialize};

#[derive(Debug, Clone,Serialize,Deserialize,Default)]
pub struct CompileInfo{
    gitHash:String,
    rustVersion:String,
    gitCommitTime:String,
    buildTime:String,
}

impl CompileInfo{
    pub fn load_from_str(compile_info: &str)->Result<CompileInfo,String>{
        let info= serde_json::from_str(&compile_info);
        match info {
            Ok(info)=>Ok(info),
            Err(err)=>Err(format!("{:?}",err)),
        }
    }
    pub fn save_to_str(&self)->String{
        serde_json::to_string(&self).unwrap()
    }
}