/*  helping outputs  */

pub const HELP_INFO: &str = "
App-launcher by lazzydisa

This program can create apps' aliases without using rc file of your shell

Usage:
    app-launcher list|l                             # print the list of you aliases
    app-launcher run|r <name>                       # start the alias by its id nember or its name
    app-launcher delete|d <name>                    # remove the alias by its id nember or its name
    app-launcher new|n <path/to/file --flag> <name> # create new alias through file path and its name
";

pub const UNKNOWN_COMMAND: &str = "Unknown command, try `app-launcher help|h` for more information";

pub const SYNTAX_ERROR: &str = "Command syntax error, try `app-launcher help|h` for more information";
