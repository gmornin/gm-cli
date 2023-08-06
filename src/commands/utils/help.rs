use std::{collections::HashMap, error::Error};

const HELP: &str = "\x1b[32mGM CLI commands\x1b[0m

\x1b[91mUTILITY:\x1b[0m
    \x1b[33mhelp\x1b[0m                                     Display this message
    \x1b[33mclean\x1b[0m                                    Remove all cached content
    \x1b[33mversion\x1b[0m                                  Print version info and exit

\x1b[91mACCOUNTS:
    \x1b[33mcreate [username] [email] [password]\x1b[0m     Create an account
    \x1b[33mdelete\x1b[0m                                   Deletes the logged in account
    \x1b[33mlogin [username] [password]\x1b[0m              Login to an account
    \x1b[33mlogout\x1b[0m                                   Remove account from this device
    \x1b[33mregen [password]\x1b[0m                         Regenerate token, all other sessions will be invalidated
    \x1b[33mstatus [status]\x1b[0m                          Set your user status to a custom string
    
\x1b[91mSTORAGE:\x1b[0m
    \x1b[33mcat [path]\x1b[0m                               Open file at path
    \x1b[33mcp [from] [to] (user) (--overwrite)\x1b[0m      Copies item
    \x1b[33mfs\x1b[0m                                       Start fs repl
    \x1b[33mls [path]\x1b[0m                                List directory content
    \x1b[33mmkdir [path]\x1b[0m                             Create new directory
    \x1b[33mmv [from] [to] (--overwrite)\x1b[0m             Moves item
    \x1b[33mrm [path]\x1b[0m                                Removes item
    \x1b[33mtouch [path]\x1b[0m                             Creates blank file at path
    \x1b[33mupload [file] [path] (--overwrite)\x1b[0m       Uploads a file
    \x1b[33mvis [path] [vis]\x1b[0m                         Change item visibility

\x1b[91mTEX:\x1b[0m
\x1b[37m[executable] tex [subcommand..]\x1b[0m
    \x1b[33mcompile [path] [from] [to] (compiler)\x1b[0m    Compiles between formats
    \x1b[33mpfpedit [file] (--reset)\x1b[0m                 Changes your profile img
    \x1b[33mpfedit (--reset)\x1b[0m                         A repl to change your profile
    \x1b[33mprofile [username]\x1b[0m                       View user profile
";

pub fn help(_map: HashMap<String, String>, _args: Vec<String>) -> Result<String, Box<dyn Error>> {
    println!("{HELP}");
    Ok("You're welcome!".to_string())
}
