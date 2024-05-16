use anyhow::Result;
mod lib;
use lib::core::fil::Fil;

fn main()  -> Result<()>{
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();



    let _action : String = std::env::args()
        .nth(1)
        .expect("Argument 2 needs to be a path");
    
    let path: String = std::env::args()
        .nth(2)
        .expect("Argument 2 needs to be a path");

    let fil: Fil = Fil::new(&path)?;
    fil.run();

    Ok(())

}
