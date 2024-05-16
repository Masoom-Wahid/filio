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

use crate::lib::utils::str_helpers::str_to_vec;

#[derive(Deserialize, Serialize,Debug)]
pub struct Filio{
    pub input : String, // the input dir , where the watcher is listen to evetns to 
    pub output : String, // where the actions takes the data to 
    pub extensions   : Vec<String>, // which extension to look for 
    pub action  : String, // what action to do , it is |mov|copy|del
    pub prefix : String, // this is purely optional , if it is given , the moven data will have the prefix of the said
    pub names : Vec<String> // match the given name 
}


impl Filio{
    pub fn new(input:String,output:String,extensions:String,action:String,prefix:String,names:String) -> Self{
        return Self{
            input,
            output,
            extensions : str_to_vec(&extensions),
            action,
            prefix,
            names :  str_to_vec(&names)
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
 

    /*
    
    this is very much ineffecient right now , but considering the fact that the user will not give more than 
    3 or 4 extensions it is still usefull although at large scale might cause performence issues so
    TODO : use hashmap instead of Vecs
     */
    fn check_extension_and_name_exists(
        &self,
        ext : &std::ffi::OsStr,
        file_name : &str
            ) -> bool{
        let mut extension_exists : bool = false;
        let mut name_exists : bool = false;
        for extension in &self.extensions{
            if *ext == *extension.as_str(){
                extension_exists = true;
                break
            }
        }

        for name in &self.names{
            // what on gods name is this ?
            // borrowing from a pointer ?
            // well it works
            if  file_name.contains(&*name){
                name_exists = true;
                break
            }
        }
        
        extension_exists && name_exists
    }

    pub fn listen<P: AsRef<Path> + std::fmt::Display>(&self,path: P) -> Result<()>{
        // create an async channel to watch the changes
        let (tx, rx) = std::sync::mpsc::channel();
    
        let mut watcher: notify::INotifyWatcher = RecommendedWatcher::new(tx, Config::default())?;
    
        // Add a path to be watched. All files at that path
        // below will be monitored for changes.
        watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;
    
        for res in rx {
            match res {
                Ok(event)  => {
                    // TODO : check if any other event is worthy of changing
                    if event.kind.is_create() || event.kind.is_modify(){
                        // Neccesary information about the event that just happened
                        let event_path : &PathBuf= &event.paths[0];
                        let event_file_ext_option: Option<&std::ffi::OsStr> = event_path.extension();
                        let event_file_name: String = match  self.get_file_name(event_path){
                                Ok(name) => name,
                                Err(e) => {
                                    log::error!("File Name Error {}",e);
                                    continue;
                                }
                        };

                        /*
                         for now we just check if the given event_file_name concludes the thing user is asking , if so 
                         we can just continue , since the name field isnt just for one action , it can be done here .
                         although for future use cases some sort of better handling would be better , but for now it is 
                         sufficent
                        */

                        // Keep this for debugging purposes
                        // println!("before");
                        // log::info!("name  is {:?}",self.names);
                        // log::info!("file_name is {:?}",event_file_name);

                        // println!("after");
                        let event_file_ext: &std::ffi::OsStr = {
                            match event_file_ext_option {
                                Some(ext) => ext,
                                _ => continue
                            }
                        };
                        if !self.check_extension_and_name_exists(event_file_ext, &event_file_name){
                            continue;
                        }
                        
                        // log::info!("Extension is {:?}",event_file_ext);

                        match self.action.as_str() {
                            "mov" => {
                                match  self.mov(&event_file_name) {
                                        Ok(_) => {},
                                        Err(e) => {
                                            log::error!("Mov Error: {}",e);
                                            continue;
                                        }
                                    
                                }
                            },
                            "del"  => {
                                match self.del(&event_file_name){
                                    Ok(_) => {},
                                    Err(e) => {
                                        log::error!("Del Error: {}",e);
                                        continue;
                                    }
                                
                            }
                            },
                            "copy"  => {
                                match self.copy(&event_file_name){
                                    Ok(_) => {},
                                    Err(e) => {
                                        log::error!("Copy Error: {}",e);
                                        continue;
                                    }
                                
                                }
                            },
                            _ => {}
                        }
                    }

                },
                Err(error) => log::error!("Base Error: {error:?}")
            }
        }
    
        Ok(())
    }
}