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
    let mut v = Var::parse_file(file)?;

    // will check for an existing alias
    for i in v.iter() {
        if i.get_name() == name {
            let e = format!("The {} alias is already exist!", name);
            return Err(String::from(e));
        }
    }

    // will write the new alias in the file
    v.push(Var::new(name, &format!("\"{}\"", path)));
    Var::write_var_vec(&v, file)?;

    Ok(())
}

/* code of the `run` command
 *
 * name - name of the alias in the ~/.config/apps/list file
 * file - the ~/.config/apps/list file
 */
pub fn run<'a>(name: &'a str, file: &'a str) -> Result<(), String> {
    let v = Var::parse_file(file)?;
    for i in v.iter() {
        if i.get_name() == name {
            func::write(i.get_value(), ".cache.sh")?;

            match Command::new("sh").arg(".cache.sh").spawn() {
                Ok(_) => (),
                Err(_)  => return Err(String::from("Can't start the command")),
            }

            break;
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
    let mut vec: Vec<Var> = Vec::new();

    let v = Var::parse_file(file)?;
    for i in v.iter() {
        if i.get_name() != name {
            let st = format!("{}: \"{}\"", i.get_name(), i.get_value());
            vec = Var::collect_to_vec(&st);
        }
    }

    Var::write_var_vec(&vec, file)?;

    Ok(())
}

/* code of the `search` command
 *
 * name - name of the alias you would like to search in the file
 * file - the ~/.config/apps/list file
 */
pub fn search<'a>(name: &'a str, file: &'a str) -> Result<(), String> {
    let v = Var::parse_file(file)?;
    for i in v.iter() {
        if i.get_name() == name {
            println!("{}: {}", i.get_name(), i.get_value());
            break;
        }
    }

    Ok(())
}
