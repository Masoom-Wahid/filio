use notify::{event::{self, CreateKind}, Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::{fs, io, path::Path};
use serde_json::Result;
use std::fs::File;
use std::io::{Read, BufReader};
use serde_json::json;
use std::thread;


fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let path = std::env::args()
        .nth(1)
        .expect("Argument 1 needs to be a path");

    // log::info!("Watching {path}");

    let res = prepare_json(&path).unwrap();

    let mut handles  = Vec::new();

    /*
    idk what to say 
    just disgust
     */
    for (key,value) in res.as_object().unwrap(){
        let path = String::from(value["path"].as_str().unwrap());
        let extension = String::from(value["extension"].as_str().unwrap());
        let output = String::from(value["output"].as_str().unwrap());
        let kind = String::from(value["kind"].as_str().unwrap());
        log::info!("Started Watching path {} for extension of {} and events of {}",path,extension,kind);
        let handle = thread::spawn(move || {
            if let Err(error) = watch(path,&output,&extension,&kind) {
                log::error!("Error: {error:?}");
            }
        });
        handles.push(handle);
    }
    for handle in handles{
        handle.join().unwrap();
    }

}


fn prepare_json(file : &str) -> serde_json::Result<serde_json::Value> {
    /*
    
    open the file and open a buffer for it and then push my way with unwrap
    i should be ashamed
    better error handling and cleaner code is screaming but atleast it works

     */



    let file = File::open(file).unwrap();
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();

    let json: serde_json::Value = serde_json::from_str(&contents)?;
    Ok(json)
}

fn watch<P: AsRef<Path> + std::fmt::Display>(path: P,output:&str,extension:&str,kind : &str) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher: notify::INotifyWatcher = RecommendedWatcher::new(tx, Config::default())?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

    for res in rx {
        match res {
            Ok(event)  => {
                match kind {
                    "create" => {
                        if event.kind.is_create(){
                            let f_path = &event.paths[0];
                            // idk why i am letting this here but we are 
                            let file_name = f_path.file_name().unwrap().to_str().unwrap();
                            if let Some(file_ext) = f_path.extension(){
                                if extension == file_ext{
                                    let input_str : String = format!("{}/{}",path,file_name);
                                    let output_str : String = format!("{}/{}",output,file_name);
                                    let input_file: &Path  = Path::new(&input_str);
                                    let output_file : &Path = Path::new(&output_str);
                                    fs::copy(input_file,output_file).unwrap();
                                    fs::remove_file(input_file).unwrap();

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
