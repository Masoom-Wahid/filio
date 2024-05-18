use anyhow::Result; 
use filio::core::fil::Fil;

fn main()  -> Result<()>{
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();



    let action : String = std::env::args()
        .nth(1)
        .expect("Argument 2 needs to be a path");

    match action.as_str() {
        "cwd" => println!("\nThe standard installation file is at /usr/share/filio/filio.json\nu can also start by -> 'filio start path_to_json_file'\n"),
        "help" => {
                    println!(
            "
            Filio , A Background File Organizer\n
            List Of Available Commands
            Commands\n
            cwd -> show where the standard filio.json is located at,
            help -> the helper command running right now,
            enable -> enable filio in background,
            start -> starts the filio but only starting rn,
            disable -> disables filio from background"
                    )
        },
        "enable" => {todo!("enable is not implemented right now")},
        "disbale" => {todo!("disbale is not implemented right now")},
        "start" => {
            // for now for default we use the standard which comes with installation
                let path = {
                    match std::env::args().nth(2) {
                        Some(p) => p,
                        None => String::from("/usr/share/filio/filio.json")
                    }
                };


                let fil: Fil = Fil::new(&path)?;
                fil.run();
        }
        _ => {log::error!("Invalid Action , Please Select A Valid Action")}
    }
    


    Ok(())

}
