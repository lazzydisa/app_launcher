/* Here is descriptions of the commands `start`, `new` and `remove` */

use {
    crate::{parser::*, func},
    std::process::Command,
};

/* code of the `new` command
 * 
 * path - path to your program
 * name - name of the alias
 * file - ~/.config/apps/list
 */
pub fn new<'a>(path: &'a str, name: &'a str, file: &'a str) {
    let buffer = func::string_from_file(file);

    if let Ok(s) = buffer {
        if s != "\n" {
            if s.contains(name) == false || s.contains(path) == false {
                let result = format!("{}\n\n{}: {}", s, name, path);
                func::write(&result, file);
            } else {
                println!("The {} alias is already exist!", name);
            }
        } else {
            let r = format!("{}: {}", name, path);
            func::write(&r, file);
        }
    }
}

/* code of the `run` command
 *
 * name - name of the alias in the ~/.config/apps/list file
 * file - the ~/.config/apps/list file
 */
pub fn run<'a>(name: &'a str, file: &'a str) -> Result<(), String> {
    let mut vec: Vec<(String, String)> = Vec::new();

    if let Ok(info) = func::string_from_file(file) {
        for line in info.lines() {
            let r = parser(line);
            if r.0 != "".to_string() && r.1 != "".to_string() {
                vec.push(r);
            }
        }
    } else {
        return Err(String::from("Can't read the file"));
    }

    for (n, val) in vec.iter() {
        if *n == name {
            func::write(&val, ".cache.sh");

            match Command::new("chmod").arg("+x").arg(".cache.sh").spawn() {
                Ok(_) => (),
                Err(_)  => return Err(String::from("Can't start the command (step 1)")),
            }

            match Command::new("sh").arg(".cache.sh").spawn() {
                Ok(_) => (),
                Err(_)  => return Err(String::from("Can't start the command (step 2)")),
            }
        }
    }

    Ok(())
}

/* code of the `delete` command
 *
 * name - name of the alias you would like remove from file
 * file - the ~/.config/apps/list file
 */
pub fn delete<'a>(name: &'a str, file: &'a str) -> Result<(), String> {
    if let Ok(st) = func::string_from_file(&file) {
        let mut r = String::new();

        for line in st.lines() {
            if !line.contains(name) {
                r.push_str(line);
            }
        }

        func::write(&r, file);
    } else {
        return Err(String::from("Can't read the file"));
    }

    Ok(())
}

/* code of the `search` command
 *
 */
pub fn search<'a>(name: &'a str, file: &'a str) -> Result<(), String> {
    if let Ok(st) = func::string_from_file(&file) {
        for line in st.lines() {
            if line.contains(name) {
                println!("{}", line);
            }
        }
    } else {
        return Err(String::from("Can't read the file"));
    }
    Ok(())
}
