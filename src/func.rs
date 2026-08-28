use std::{
    fs::{self, File},
    io::Write,
};

// info from file (for `list` and `start` commands)
pub fn string_from_file(file: &str) -> Result<String, String> {
    match fs::read_to_string(file) {
        Ok(s) => Ok(s),
        Err(_) => Err(String::from("Error: can't read the file")),
    }
}

// write info to file
pub fn write(st: &str, filename: &str) -> Result<(), String> {
    let mut file = match File::create(filename) {
        Ok(file) => file,
        Err(_) => return Err(String::from("Can't create the file")),
    };

    match file.write_all(st.as_bytes()) {
        Ok(()) => Ok(()),
        Err(_) => Err(String::from("Can't write to the file")),
    }
}

// checks for existing of .config/apps directory (if not then directory will be created)
pub fn check_dir(path: &String) -> Result<(), String> {
    match fs::exists(&path) {
        Ok(true) => Ok(()),

        Ok(false) => {
            // if else if else darkness :/
            if let Ok(test) = fs::create_dir_all(path) {
                if test != () { 
                    Err(String::from("Can't create ~/.config/apps directory"))
                } else {
                    Ok(())
                }
            } else {
                Ok(())
            }
        },

        Err(_) => Err(String::from("Can't read ~/.config directory")),
    }
}

// for printing the help info
#[inline]
pub fn help_info(st: &str, status: i32) {
    println!("{}", st);
    std::process::exit(status);
}

// checks command line arguments
pub fn check_args(args: &Vec<String>, length: usize) -> Result<(), String> {
    use crate::consts::*;

    let consts_arr = [
        HELP_INFO,
        ARGS_LEN_SHOULD_BE_2,
        ARGS_LEN_SHOULD_BE_3,
        ARGS_LEN_SHOULD_BE_4,
    ];

    if args.len() < length {
        Err(String::from(consts_arr[length-1]))
    } else {
        Ok(())
    }
}
