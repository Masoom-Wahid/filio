use std::{fs::canonicalize, path::PathBuf};
use anyhow::Result;

pub fn get_abs_path(name : &str)  -> Result<String>{
   /*
   Used for getting the absloute path to anything in the test_dir
    */ 
    let abs_path : String= canonicalize(PathBuf::from(format!("./src/tests/data/{}",name)))?
                            .to_str()
                            .unwrap()
                            .try_into()?;
    Ok(abs_path)
}