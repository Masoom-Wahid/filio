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
use std::{fs, path::Path};
use serde_derive::{Deserialize, Serialize};
#[derive(Deserialize, Serialize,Debug)]
pub struct Filio{
    pub input : String,
    pub output : String,
    pub extension   : String,
    pub kind  : String,
}


impl Filio{
    pub fn new(input:String,output:String,extension:String,kind:String) -> Self{
        return Self{
            input,
            output,
            extension,
            kind
        }
    }

    pub fn listen<P: AsRef<Path> + std::fmt::Display>(&self,path: P) -> notify::Result<()> {
        let (tx, rx) = std::sync::mpsc::channel();
    
        let mut watcher: notify::INotifyWatcher = RecommendedWatcher::new(tx, Config::default())?;
    
        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;
    
        for res in rx {
            match res {
                Ok(event)  => {
                    match self.kind.as_str() {
                        "create" => {
                            if event.kind.is_create(){
                                let f_path = &event.paths[0];
                                // idk why i am letting this here but we are 
                                let file_name = f_path.file_name().unwrap().to_str().unwrap();
                                if let Some(file_ext) = f_path.extension(){
                                    if self.extension.as_str() == file_ext{
                                        fs::rename(
                                            Path::new(&format!("{}/{}",path,file_name)),
                                            Path::new(&format!("{}/{}",self.output,file_name))
                                            
                                            )?;
                                    }
                                }
                            }
                        },
                        _ => {}
                    }
                },
                Err(error) => log::error!("Error: {error:?}"),
                _ => {},
            }
        }
    
        Ok(())
    }
}