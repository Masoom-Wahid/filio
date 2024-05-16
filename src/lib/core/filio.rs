/*

How the file system should be structed 
a sanitized hashmap of the files the user has in his .json
and then implement methods for that

smth like 

filio{
    input
    output
    ext
    name
    kind
    status
}

*/

use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::{fs, path::{Path, PathBuf}};
use serde_derive::{Deserialize, Serialize};
use anyhow::{Error, Result};

#[derive(Deserialize, Serialize,Debug)]
pub struct Filio{
    pub input : String,
    pub output : String,
    pub extension   : String,
    pub action  : String,
    pub prefix : String,
}


impl Filio{
    pub fn new(input:String,output:String,extension:String,action:String,prefix:String) -> Self{
        return Self{
            input,
            output,
            extension,
            action,
            prefix
        }
    }

    fn mov(&self,file_name : &str)-> Result<()>{   
        fs::rename(
            Path::new(&format!("{}/{}",self.input,file_name)),
            Path::new(&format!("{}/{}{}",self.output,self.prefix,file_name))
            )?;
        Ok(())
    }

    fn del(&self,file_name : &str) -> Result<()>{
        fs::remove_file(Path::new(&format!("{}/{}",self.input,file_name)))?;
        Ok(())
    }


    fn copy(&self,file_name : &str) -> Result<()>{
        fs::copy(
            Path::new(&format!("{}/{}",self.input,file_name)),
            Path::new(&format!("{}/{}{}",self.output,self.prefix,file_name))
            )?;
        Ok(())
    }

    fn get_file_name(&self,event_path : &PathBuf) -> Result<String>{
        let event_file_name: &str = match event_path.file_name() {
            // TODO: fix the unwrap to a more secure way of handling it 
            Some(s) => s.to_str().unwrap(),
            None => return Err(Error::msg("Invalid file name")),
        };
        Ok(event_file_name.to_string())
    }
 

    pub fn listen<P: AsRef<Path> + std::fmt::Display>(&self,path: P) -> Result<()>{
        // create an async channel to watch the changes
        let (tx, rx) = std::sync::mpsc::channel();
    
        let mut watcher: notify::INotifyWatcher = RecommendedWatcher::new(tx, Config::default())?;
    
        // Add a path to be watched. All files at that path and
        // below will be monitored for changes.
        watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;
    
        for res in rx {
            match res {
                Ok(event)  => {
                    // TODO : check if any other event is worthy of changing
                    if event.kind.is_create() || event.kind.is_modify(){
                        // Neccesary information about the event that just happened
                        let event_path : &PathBuf= &event.paths[0];
                        let event_file_ext: Option<&std::ffi::OsStr> = event_path.extension();
                        let event_file_name: String = match  self.get_file_name(event_path){
                                Ok(name) => name,
                                Err(e) => {
                                    log::error!("Error {}",e);
                                    break;
                                }
                        };
                        match self.action.as_str() {
                            "mov" => {
                                match event_file_ext {
                                    Some(ext) => {
                                        if self.extension.as_str() == ext{
                                            match  self.mov(&event_file_name) {
                                                    Ok(_) => {},
                                                    Err(e) => {
                                                        log::info!("Error: {}",e);
                                                    }
                                                
                                            }
                                        }
                                    },
                                    None => {}
                                }
                            },
                            "del"  => {
                                match self.del(&event_file_name){
                                    Ok(_) => {},
                                    Err(e) => {
                                        log::info!("Error: {}",e);
                                    }
                                
                            }
                            },
                            "copy"  => {
                                match event_file_ext {
                                    Some(ext) => {
                                        if self.extension.as_str() == ext{
                                            match self.copy(&event_file_name){
                                                Ok(_) => {},
                                                Err(e) => {
                                                    log::info!("Error: {}",e);
                                                }
                                            
                                            }
                                        }
                                    },
                                    None => {}
                                }
                            },
                            _ => {}
                        }
                    }

                },
                Err(error) => log::error!("Error: {error:?}")
            }
        }
    
        Ok(())
    }
}