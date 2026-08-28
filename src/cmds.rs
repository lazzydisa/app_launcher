/* Here is descriptions of the commands `start`, `new` and `remove` */

use {
    crate::{dsc_config_lib::*, func},
    std::process::Command,
};

/* code of the `new` command
 * 
 * path - path to your program
 * name - name of the alias
 * file - ~/.config/apps/list
 */
pub fn new<'a>(path: &'a str, name: &'a str, file: &'a str) -> Result<(), String> {
    if let Ok(v) = Var::parse_file(file) {
        for i in v.iter() {
            if i.get_name() == name {
                let e = format!("The {} alias is already exist!", name);
                return Err(String::from(e));
            }
            else {
                let r = Var::new(name, path);

                match r.write_var(file) {
                    Ok(()) => (),
                    Err(e) => return Err(e),
                }
            }
        }
    }

    Ok(())
}

/* code of the `run` command
 *
 * name - name of the alias in the ~/.config/apps/list file
 * file - the ~/.config/apps/list file
 */
pub fn run<'a>(name: &'a str, file: &'a str) -> Result<(), String> {
    if let Ok(v) = Var::parse_file(file) {
        for i in v.iter() {
            if i.get_name() == name {
                match func::write(i.get_value(), ".cache.sh") {
                    Ok(()) => (),
                    Err(e) => return Err(e),
                }

                match Command::new("chmod").arg("+x").arg(".cache.sh").spawn() {
                    Ok(_) => (),
                    Err(_)  => return Err(String::from("Can't start the command (step 1)")),
                }

                match Command::new("sh").arg(".cache.sh").spawn() {
                    Ok(_) => (),
                    Err(_)  => return Err(String::from("Can't start the command (step 2)")),
                }
            } else {
                let e = format!("the alias {} does not exist", name);
                return Err(String::from(e));
            }
        }
    }

    Ok(())
}

/* code of the `delete` command
 *
 * name - name of the alias you would like to remove from the file
 * file - the ~/.config/apps/list file
 */
pub fn delete<'a>(name: &'a str, file: &'a str) -> Result<(), String> {
    if let Ok(v) = Var::parse_file(file) {
        for i in v.iter() {
            if i.get_name() != name {
                match i.write_var(file) {
                    Ok(()) => (),
                    Err(e) => return Err(e),
                }
            }
        }
    }

    Ok(())
}

/* code of the `search` command
 *
 * name - name of the alias you would like to search in the file
 * file - the ~/.config/apps/list file
 */
pub fn search<'a>(name: &'a str, file: &'a str) {
    if let Ok(v) = Var::parse_file(file) {
        for i in v.iter() {
            if i.get_name() == name {
                println!("{}: {}", i.get_name(), i.get_value());
            } else {
                println!("Alias {} was not found", name);
            }
        }
    }
}
