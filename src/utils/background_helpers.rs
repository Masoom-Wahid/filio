use anyhow::Result;
use std::process::Command;


/*the function is called for creating a systemd service to keep filio running in the background
 *although there should be better ways for keeping it in background for now it is better this way
 */

pub fn enable_filio(json_path : &str) -> Result<()>{
    /*
    coudlnt get this to work , so for now i am invoking bash directly
     */
    // let output = Command::new("nohup")
    //     .arg("filio")
    //     .arg("start")
    //     .arg("/home/masoom/.filio/data/filio.json")
    //     .arg(">")
    //     .arg("/home/masoom/.filio/logs/out.log")
    //     .arg("2>&1 &")
    //     .output()?;


    let command = format!("nohup filio start {} > ~/.filio/logs/out.log 2>&1 & ",json_path);
    Command::new("/bin/sh")
        .arg("-c")
        .arg(&command)
        .spawn()?;

    Ok(())
}

/*
Kills anything related to filio
*/
pub fn disable_filio() -> Result<()>{
    Command::new("killall")
        .arg("-9")
        .arg("filio")
        .spawn()?;
    Ok(())
}


/*
kill and runs again 
*/
pub fn restart_filio(json_path : &str) -> Result<()>{
    disable_filio()?;
    enable_filio(json_path)?;
    Ok(())
}
