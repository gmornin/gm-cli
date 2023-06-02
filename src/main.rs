use log::*;
use std::{
    collections::HashMap,
    env,
    error::Error,
    fs::{self, OpenOptions},
};

use gm_cli::{commands, config::AccountConfig, functions::args_parse, traits::ConfigTriat};
use simplelog::*;

fn main() {
    init().unwrap();
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Trace,
            Config::default(),
            OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(
                    dirs::data_dir()
                        .unwrap()
                        .join(env::var("CARGO_PKG_NAME").unwrap())
                        .join("latest.log"),
                )
                .unwrap(),
        ),
    ])
    .unwrap();

    let args = env::args().skip(1).collect::<Vec<_>>();
    trace!("Running with args {}", args.join(" "));

    if args.is_empty() {
        error!("No command found");
        return;
    }

    let mut args_map = HashMap::new();

    if let Err(e) = config_init(&mut args_map) {
        error!("Failed to load config: {e}");
        info!("Try deleting the problematic config file to regenereate");
        return;
    };
    args_parse(&args, &mut args_map);

    let commands = commands::commands();
    match commands.get(args.first().unwrap().as_str()) {
        Some(command) => match command(args_map) {
            Ok(msg) => trace!("Command finished with message `{msg}`"),
            Err(e) => error!("Command exited with error `{e}`"),
        },
        None => error!("No such command"),
    }
}

fn init() -> Result<(), Box<dyn Error>> {
    let package = env::var("CARGO_PKG_NAME").unwrap();

    fs::create_dir_all(dirs::config_dir().unwrap().join(&package))?;
    fs::create_dir_all(dirs::data_dir().unwrap().join(&package))?;

    Ok(())
}

fn config_init(map: &mut HashMap<String, String>) -> Result<(), Box<dyn Error>> {
    AccountConfig::load()?.extend_map(map);

    Ok(())
}
