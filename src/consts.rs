/*  helping outputs  */

// help info
pub const HELP_INFO: &str = "
App-launcher by lazzydisa

This program can create apps' aliases without using rc file of your shell

Usage:
    app-launcher list|l                             # print the list of you aliases
    app-launcher run|r <name>                       # start the alias by its id nember or its name
    app-launcher delete|d <name>                    # remove the alias by its id nember or its name
    app-launcher new|n <path/to/file --flag> <name> # create new alias through file path and its name
";

// if args.len() < 2    (not only fo this usecase)
pub const ARGS_LEN_SHOULD_BE_2: &str = " 
App-launcher by lazzydisa

For help information use `help` command:
    app-launcher help|h
";

// if args.len() < 3 and args[1] == "start" or "remove"
pub const ARGS_LEN_SHOULD_BE_3: &str = "
App-launcher by lazzydisa

For `start` and `remove` commands you need to add 3th parameter
For `new` command you need 4 command line arguments

Usage:
    app-launcher run|r <name>
    app-launcher delete|d <name>
    app-launcher new|n <path/to/file --flag> <name>
";

// if args.len() < 4 and args[1] == "new"
pub const ARGS_LEN_SHOULD_BE_4: &str = "
App-launcher by lazzydisa

For the `new` command you need file path (and, optionally, some flags) and name for app's alias

Usage:
    app-launcher new|n <path/to/file --flag> <name>
";
