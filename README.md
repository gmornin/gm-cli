# GM CLI

This is a simple CLI for interacting with the [GM services](https://github.com/gmornin/services) ([API docs](https://siriusmart.github.io/gm-services)).

## Overview

GM CLI is the easier way to access info from a GM server compared to using `curl`. Instead of using the web interface provided by whatever website you are using, the CLI provides an unifying experience when using different instances or specialised services.

### Installation

Make sure your have Rust/Cargo installed:

```sh
cargo install --git https://github.com/gmornin/gm-cli
```

> You may add an `alias gmtex=gm-cli tex` to your `.bashrc` (or `.zshrc`) to speed things up when using the `tex` subcommand.

### Commands

> You can run `gm-cli help` to see all the commands.

#### Utility

```
help                                     Display this message
clean                                    Remove all cached content
version                                  Print version info and exit
```

#### Accounts

```
create [username] [email] [password]     Create an account
delete                                   Deletes the logged in account
login [username] [password]              Login to an account
logout                                   Remove account from this device
regen [password]                         Regenerate token, all other sessions will be invalidated
status [status]                          Set your user status to a custom string
``` 

#### Storage

```
cat [path]                               Open file at path
cp [from] [to] (user) (--overwrite)      Copies item
fs                                       Start fs repl
ls [path]                                List directory content
mkdir [path]                             Create new directory
mv [from] [to] (--overwrite)             Moves item
rm [path]                                Removes item
touch [path]                             Creates blank file at path
upload [file] [path] (--overwrite)       Uploads a file
vis [path] [vis]                         Change item visibility
```

#### Tex

```
[executable] tex [subcommand..]
```

```
compile [path] [from] [to] (compiler)    Compiles between formats
pfpedit [file] (--reset)                 Changes your profile img
pfedit (--reset)                         A repl to change your profile
profile [username]                       View user profile
publish [path] [title] [desc]            Publish a file
publishes [usernaem] [page] (per_page)   View user published files
```

#### Arguments

Some command allows for arguments, arguments are represented with **square brackets** (`[]`).

If the argument you want to pass in contains *whitespaces*, follow the standard Bash syntax and use single or double quotes to show that it is the same argument. If no arguments, or not enough arguments are entered, you will be **prompted to enter the missing ones**.

> It is suggested to ***enter passwords in prompt*** rather than passing in as an argument.

#### Flags

Alternative to arguments, you may also pass in flags.

> **For example**, for `cp [from] [to]`: you may run `cp --from /from/path --to /to/path`.

Some additional flags includes:

- overwrite
- reset
- ***http*** (uses http instead of https, can be used in all commands)

### Code structure

```
└── src
    ├── commands
    │   ├── accounts
    │   │   ├── create.rs
    │   │   ├── delete.rs
    │   │   └── ...
    │   ├── jobs/
    │   ├── storage/
    │   └── tex/
    └── main.rs
```

See [`commands/mod.rs`](https://github.com/gmornin/gm-cli/blob/master/src/commands/mod.rs), showing how commands are added:

```rs
pub fn commands() -> Command {
    let mut map: HashMap<&str, Command> = HashMap::new();
    map.insert("tex", tex::commands().into()); // tex is a subcommand
    Command::from(accounts::commands()).extend_map(&mut map); // others are added as top level commands
    Command::from(storage::commands()).extend_map(&mut map);
    Command::from(utils::commands()).extend_map(&mut map);
    Command::from(jobs::commands()).extend_map(&mut map);
    map.into()
}
```

> To add your own commands, check out how it's done in other files.

### Development

GM CLI can help with the development of the API server ([GM services](https://github.com/gmornin/services)). To do so, clone this repo and add new commands for testing the API.

You can install the modified version using

```
cargo install --path .
```
