use {
    app_launcher::{
        func, 
        cmds, 
        consts
    },
    std::{env, process},
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let user = match env::var("USER") {
        Ok(val) => val,
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        },
    };

    let dir = format!("/home/{}/.config/apps", user);
    if func::check_dir(&dir) != Ok(()) {
        println!("{:?}", func::check_dir(&dir));
        process::exit(1);
    }

    let file = format!("{}/list", dir);
    if let Err(_) = func::string_from_file(&file) {
        func::write("\n", &file); 
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
                if let Ok(test) = func::string_from_file(&file) {
                    println!("Added aliases:\n\n{}", test);
                    return;
                } else {
                    let error = format!("{:?}", func::string_from_file(&file)); 
                    func::help_info(&error, 1);
                }
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
            "start" | "s"  => {
                let r = cmds::start(&args[2], &file);
                if r != Ok(()) {
                    if let Err(e) = r {
                        func::help_info(&e, 1);
                    }
                }
                return;
            },
            "remove" | "r" => {
                let r = cmds::remove(&args[2], &file);
                if r != Ok(()) {
                    if let Err(e) = r {
                        func::help_info(&e, 1);
                    }
                }
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
                let r = format!("\"{}\"", &args[2]);
                cmds::new(&r, &args[3], &file);
            },
            _ => func::help_info(consts::HELP_INFO, 1),
        }
    }
}
