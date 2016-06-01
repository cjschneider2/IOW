use std::env;
use std::path::PathBuf;

fn main() {

    // Setting the search path for the dictionaries
    let iow_dir = {
        match env::var("IOW_DICT") {
            Ok(val) => PathBuf::from(val),
            Err(_)  => {
                let mut path = env::home_dir().unwrap();
                path.push(".iow");
                path
            }
        }
    };
    println!("Looking for dictionaries in: {:?}", iow_dir);

    // TODO: Dictionary finding code

    // TODO: Search the dictionaries for the term

    // TODO: Print out the found term (if any)

}
