use anyhow::Result;
pub mod core;

use core::Fil::Fil;

fn main()  -> Result<()>{
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();


    let action : String = std::env::args()
        .nth(1)
        .expect("Argument 2 needs to be a path");
    
    let path: String = std::env::args()
        .nth(2)
        .expect("Argument 2 needs to be a path");

    let fil: Fil = Fil::new(&path)?;
    fil.run();

    Ok(())

}
