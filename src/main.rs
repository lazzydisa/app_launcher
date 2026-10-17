use {
    app_launcher::{
        func, 
        cmds, 
        consts::*,
        dsc_config_lib::*,
    },
    std::env,
};

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("{}", HELP_INFO);
        return Ok(())
    }

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

    match &args[1][..] {
        "list" | "l" => {
            println!("List of your apps' aliases\n");
            let v = Var::parse_file(&file)?;
            for i in v.iter() {
                println!("{}: {}", i.get_name(), i.get_value());
            }
            return Ok(())
        },
        "help" | "h" => {
            println!("{}", HELP_INFO);
            return Ok(())
        },
        "run" | "r" => {
            if args.len() != 3 { return Err(String::from(SYNTAX_ERROR)) }
            cmds::run(&args[2], &file)?;
            return Ok(())
        },
        "delete" | "d" => {
            if args.len() != 3 { return Err(String::from(SYNTAX_ERROR)) }
            cmds::delete(&args[2], &file)?;
            return Ok(())
        },
        "search" | "s" => {
            if args.len() != 3 { return Err(String::from(SYNTAX_ERROR)) }
            cmds::search(&args[2], &file)?;
            return Ok(())
        },
        "new" | "n" => {
            if args.len() != 4 { return Err(String::from(SYNTAX_ERROR)) }
            cmds::new(&args[2], &args[3], &file)?;
            return Ok(())
        },
        _ => return Err(String::from(UNKNOWN_COMMAND)),
    }
}
