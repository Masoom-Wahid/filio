use anyhow::Result; 
use filio::core::fil::Fil;
use filio::utils::background_helpers::{enable_filio,disable_filio,restart_filio};
use filio::utils::path_helpers::get_json_path;

fn main()  -> Result<()>{
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let version = "1.1";


    let action : String = std::env::args()
        .nth(1)
        .expect("Argument 2 needs to be a path");

    match action.as_str() {
        "cwd" => println!("{}",get_json_path()?),
        "help" => {
            let help_output = r#"
            Filio , A Background File Organizer
            List Of Available Commands
            Commands
            enable -> start filio in background,
            disable -> disables filio from background
            start -> starts the filio but not on backgrounds,
            restart -> restart the filio running in background,
            cwd -> show where the standard filio.json is located at,
            help -> the helper command running right now,
            version -> shows the current version,
            "#;
            println!("{}",help_output);
        },
        "version" => {
            println!("{}",version)
        },
        "enable" => {
            let path = get_json_path()?;
            enable_filio(&path)?;
        },
        "disable" => {
            disable_filio()?;
        },
        "restart" => {
            let path = get_json_path()?;
            restart_filio(&path)?;
        },
        "start" => {
            // for now for default we use the standard which comes with installation
            let path = get_json_path()?;
            let fil: Fil = Fil::new(&path)?;
            fil.run();
        }
        _ => {log::error!("Invalid Action , Please Select A Valid Action")}
    }
    


    Ok(())

}
