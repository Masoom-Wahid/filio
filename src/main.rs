use anyhow::Result; 
use filio::core::fil::Fil;
use filio::utils::background_helpers::{enable_filio,disable_filio,restart_filio};
use filio::utils::path_helpers::get_json_path;
use filio::utils::str_helpers::print_help;

fn main()  -> Result<()>{
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let version = "1.1";


    let action : String = std::env::args()
        .nth(1)
        .expect("Argument 2 needs to be a path");

    match action.as_str() {
        "cwd" => println!("{}",get_json_path()?),
        "help" => {
            print_help();
        },
        "version" => {
            println!("{}",version)
        },
        "enable" => {

        },
        "start" => {
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
        "run" => {
            // for now for default we use the standard which comes with installation
            let path = get_json_path()?;
            let fil: Fil = Fil::new(&path)?;
            fil.run();
        }
        _ => {print_help();}
    }
    


    Ok(())

}
