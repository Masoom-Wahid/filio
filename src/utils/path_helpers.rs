/*
 * A helper function to check if the second arg is a path 
 * otherwise returns the default path which is ~/.filio
 *
 */

use std::process::Command;
use anyhow::Result;

pub fn get_json_path() -> Result<String>{
    match std::env::args().nth(2){
        Some(p) => Ok(p),
        None => {
            /*
            if the user is root then ~ for it is really helpfull and would not probbably require
            the sudo perm 
            instead of that if it is a regular user then we go the /home/{username} path
             */
            let whomami = get_username()?;
            let username = whomami.trim();
            match username {
                "root" => Ok(String::from("~/.filio/data/filio.json")),
                 _ => Ok(format!("/home/{}/.filio/data/filio.json",&username))
            }
            
        }
    }
}


pub fn get_username() -> Result<String>{
    let whoami = Command::new("whoami").output()?;
    let username = String::from_utf8_lossy(&whoami.stdout).to_string();
    Ok(username)
}
