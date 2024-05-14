use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::thread;
use std::fs::File;
use std::io::{Read, BufReader};
use std::path::Path;
use anyhow::{Error, Result};
use std::collections::HashMap;
use crate::core::Filio::Filio;


#[derive(Deserialize, Serialize,Debug)]
pub struct Fil{
    #[serde(default)]
    filios : HashMap<String,Filio>
}


impl Fil{
    pub fn new(path:&str) -> Result<Fil>{
        let json_obj : Value = Self::to_json(path)?;
        let mut filio_data :HashMap<String,Filio> = HashMap::new();

        for (key,value) in json_obj.as_object().expect("Invalid Syntax"){
            let name = key.clone();
            let input = value["input"].as_str().expect("
                Missing Key : 'input'
            ");

            let output = value["output"].as_str().expect("
            Missing Key : 'output'
            ");

            let kind = value["kind"].as_str().expect("
            Missing Key : 'kind'
            ");

            let extension = value["extension"].as_str().expect("
            Missing Key : 'extension'
            ");

            if !(Path::new(input).is_dir() && Path::new(output).is_dir()){
                return Err(
                    Error::msg(                 
                            format!(
                                "Please make sure that either dirs are correct\nInput : {},\nOutput : {}
                                ",input,output
                            )
                        )
                    )
            }

            let filio: Filio = Filio::new(
                String::from(input),
                String::from(output),
                String::from(extension),
                String::from(kind)
            );

            filio_data.insert(name, filio);
        }
        Ok(
            Fil{
                filios : filio_data
            }
         )
    }


    fn to_json(path : &str) -> Result<Value>{
        let file: File = File::open(path)?;
        let mut buffer: BufReader<File> = BufReader::new(file);
        let mut contents: String = String::new();
        buffer.read_to_string(&mut contents)?;
    
        let json: serde_json::Value = serde_json::from_str(&contents)?;
        Ok(json)
    }

    pub fn run(&self){
        thread::scope(|s| {
            for (_, filio) in &self.filios {
                s.spawn(move || {
                    log::info!("Started Watching path {} for extension of {} and events of {}",filio.input,filio.extension,filio.kind);
                    if let Err(error) = filio.listen(&filio.input) {
                        log::error!("Error: {:?}", error);
                    }
                });
            }
        });
    }
}
