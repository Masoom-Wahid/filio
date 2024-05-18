/*

Any utils with strs should go here
.*/


// Just a helper function to help convert string to vec<string>
pub fn str_to_vec(input : &str) -> Vec<String>{
    input.split(',').map(|s| s.trim().to_string()).collect()
}