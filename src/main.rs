use {
    app_launcher::{
        func, 
        cmds, 
        consts,
        dsc_config_lib::*,
    },
    std::env,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let user = match env::var("USER") {
        Ok(val) => val,
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        },
    };

    let dir = format!("/home/{}/.config/apps", user);
    if let Err(e) = func::check_dir(&dir) {
        func::help_info(&e, 1);
    }

    let file = format!("{}/list", dir);
    if let Err(_) = func::string_from_file(&file) {
        match func::write("\n", &file) {
            Ok(()) => (),
            Err(e) => func::help_info(&e, 1),
        }
    }

    // WARNING! SAFETY PROCCESSING OT THE COMMAND LINE ARGUMENTS
    let result = func::check_args(&args, 2);
    if result != Ok(()) {
        // print info about `help`
        if let Err(e) = result {
            func::help_info(&e, 1);
        }
    } else {
        match &args[1][..] {
            "list" | "l" => {
                println!("List of your apps' aliases\n");

                if let Ok(v) = Var::parse_file(&file) {
                    for i in v.iter() {
                        println!("{}: {}", i.get_name(), i.get_value());
                    }
                }

                return;
            },
            "help" | "h" => func::help_info(consts::HELP_INFO, 0),
            _ => (),
        }
    }

    let result = func::check_args(&args, 3);
    if result != Ok(()) {
        // print info about `start`, `remove` and `new`
        if let Err(e) = result {
            func::help_info(&e, 1);
        }
    } else {
        match &args[1][..] {
            "run" | "r"  => {
                match cmds::run(&args[2], &file) {
                    Ok(()) => (),
                    Err(e) => func::help_info(&e, 1),
                }
                return;
            },
            "delete" | "d" => {
                match cmds::delete(&args[2], &file) {
                    Ok(()) => (),
                    Err(e) => func::help_info(&e, 1),
                }
                return;
            },
            "search" | "s" => {
                cmds::search(&args[2], &file);
                return;
            },
            _ => (),
        }
    }

    let result = func::check_args(&args, 4);
    if result != Ok(()) {
        if let Err(e) = result {
            func::help_info(&e, 1);
        }
    } else {
        match &args[1][..] {
            "new" | "n" => {
                match cmds::new(&args[2], &args[3], &file) {
                    Ok(()) => (),
                    Err(e) => func::help_info(&e, 1),
                }
            },
            _ => func::help_info(consts::HELP_INFO, 1),
        }
    }
}
