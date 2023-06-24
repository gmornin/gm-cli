use log::*;
use std::{
    collections::HashMap,
    env,
    error::Error,
    fs::{self, OpenOptions},
};

use gm_cli::{
    commands,
    config::{AccountConfig, ApplicationsConfig},
    functions::args_parse,
    traits::ConfigTriat,
};
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
                        .join(env!("CARGO_PKG_NAME"))
                        .join("latest.log"),
                )
                .unwrap(),
        ),
    ])
    .unwrap();

    let args = env::args().skip(1).collect::<Vec<_>>();
    debug!("Running with args {}", args.join(" "));

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

    commands::commands().run(args_map, args);
}

fn init() -> Result<(), Box<dyn Error>> {
    let package = env!("CARGO_PKG_NAME");

    fs::create_dir_all(dirs::config_dir().unwrap().join(package))?;
    fs::create_dir_all(dirs::data_dir().unwrap().join(package))?;
    fs::create_dir_all(dirs::cache_dir().unwrap().join(package).join("downloads"))?;

    Ok(())
}

fn config_init(map: &mut HashMap<String, String>) -> Result<(), Box<dyn Error>> {
    AccountConfig::load()?.extend_map(map);
    ApplicationsConfig::load()?;

    Ok(())
}
