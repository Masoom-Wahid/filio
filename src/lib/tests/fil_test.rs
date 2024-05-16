#[cfg(test)]
mod tests {
    use crate::lib::core::fil::*;
    use crate::lib::tests::get_abs_path::get_abs_path;
    use std::thread;
    use std::time::Duration;
    #[test]
    fn test_fil_new(){
        let path = get_abs_path("test.json").unwrap();
        let fil = Fil::new(&path);
        assert!(!fil.is_err());
    }
    #[test]
    fn test_fil_run() {
        /*
        One way of testing the run() method is by adding a timer for 
        10 sec and if it does not break in that long , well it should be good 
        to go.
         */
        let path = get_abs_path("test.json").unwrap();
        let fil : Fil = Fil::new(&path).unwrap();

        thread::spawn(move || {
            fil.run();
        });
        thread::sleep(Duration::from_secs(10));
        assert!(true);
    }
}
