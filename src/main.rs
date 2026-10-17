use {
    app_launcher::{
        func, 
        cmds, 
        consts,
        dsc_config_lib::*,
    },
    std::{
        env,
    },
};

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    let user = match env::var("USER") {
        Ok(val) => val,
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        },
    };

    let dir = format!("/home/{}/.config/apps", user);
    let _ = func::check_dir(&dir)?;

    let file = format!("{}/list", dir);
    let _ = func::write("\n", &file)?;

    // WARNING! SAFETY PROCCESSING OT THE COMMAND LINE ARGUMENTS
    let _ = func::check_args(&args, 2)?;
    match &args[1][..] {
        "list" | "l" => {
            println!("List of your apps' aliases\n");
            let v = Var::parse_file(&file)?;
            for i in v.iter() {
                println!("{}: {}", i.get_name(), i.get_value());
            }
            return Ok(());
        },
        "help" | "h" => func::help_info(consts::HELP_INFO, 0),
        _ => (),
    }


    let _ = func::check_args(&args, 3)?;
    match &args[1][..] {
        "run" | "r"  => { cmds::run(&args[2], &file)?; return Ok(()) },
        "delete" | "d" => { cmds::delete(&args[2], &file)?; return Ok(()) },
        "search" | "s" => { cmds::search(&args[2], &file)?; return Ok(()) },
        _ => (),
    }

    let _ = func::check_args(&args, 4);
    match &args[1][..] {
        "new" | "n" => { cmds::new(&args[2], &args[3], &file)?; return Ok(()) },
        _ => func::help_info(consts::HELP_INFO, 1),
    }

    Ok(())
}
