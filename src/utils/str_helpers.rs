/*

Any utils with strs should go here
.*/


// Just a helper function to help convert string to vec<string>
pub fn str_to_vec(input : &str) -> Vec<String>{
    input.split(',').map(|s| s.trim().to_string()).collect()
}


/*
 * prints the 'filio help' function'
 */

pub fn print_help(){
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
}
