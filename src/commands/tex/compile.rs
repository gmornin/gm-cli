use std::any::Any;
use std::path::PathBuf;
use std::{collections::HashMap, error::Error};

use crate::config::AccountConfig;
use crate::error::Error as CError;
use crate::functions::{map_args, post, prompt_not_present};

use goodmorning_bindings::services::v1::{
    Compiler, FromFormat, ToFormat, V1Compile, V1Error, V1Response,
};
use goodmorning_bindings::structs::TexCompileRes;
use log::*;

const ARGS: &[&str] = &["path", "from", "to", "compiler"];

pub fn compile(
    mut map: HashMap<String, String>,
    args: Vec<String>,
) -> Result<String, Box<dyn Error>> {
    map_args(&mut map, ARGS, args)?;
    if !AccountConfig::is_loggedin_map(&map) {
        error!("You must be logged in to view user files");
        return Err(CError::StrErr("not logged in").into());
    }

    prompt_not_present("Path (`/tex` omitted)", "path", &mut map);
    prompt_not_present("From format", "from", &mut map);
    prompt_not_present("To format", "to", &mut map);

    let path = PathBuf::from(map.get("path").unwrap());
    if !path.has_root() {
        error!("User file paths must start with root `/`");
        return Err(CError::StrErr("invalid file path").into());
    }

    let path = path.to_str().unwrap().to_string();

    let instance = map.get("instance").unwrap();
    let token = map.get("token").unwrap().to_string();

    let url = format!("{instance}/api/compile/v1/simple");

    let from = match map.get("from").unwrap().as_str() {
        "md" | "markdown" => FromFormat::Markdown,
        "latex" => FromFormat::Latex,
        _ => {
            error!("Valid formats are: `markdown` and `latex`");
            return Err(CError::StrErr("invalid from format").into());
        }
    };
    let to = match map.get("to").unwrap().as_str() {
        "html" => ToFormat::Html,
        "pdf" => ToFormat::Pdf,
        _ => {
            error!("Valid formats are: `html`");
            return Err(CError::StrErr("invalid to format").into());
        }
    };
    prompt_not_present("Compiler (leave blank for default)", "compiler", &mut map);

    let body = V1Compile {
        path,
        token,
        from,
        to,
        compiler: match map.get("compiler").unwrap().as_str() {
            "" => None,
            "pulldown" | "pulldown-cmark" => Some(Compiler::PulldownCmark),
            "pdflatex" => Some(Compiler::Pdflatex),
            _ => {
                error!(
                    "Unknow compiler, accept `pulldown` (md to html) and `pdflatex` (latex to pdf)"
                );
                return Err(CError::StrErr("Invalid compiler").into());
            }
        },
    };

    let res: V1Response = post(&url, body, map.contains_key("http"))?;

    match res {
        // V1Response::Compiled { id, newpath } => {
        //     info!("Compile success");
        //     info!("New path: {newpath}");
        //     info!("Job ID: {id}");
        // }
        V1Response::Any { value } => {
            let value: Box<dyn Any> = value;
            let res = value.downcast_ref::<TexCompileRes>().unwrap();
            info!("Compile success");
            info!("New path: {}", res.newpath);
            info!("Job ID: {}", res.id);
        }
        V1Response::TexCompiled { id, newpath } => {
            info!("Compile success");
            info!("New path: {newpath}");
            info!("Job ID: {id}");
        }
        V1Response::Error {
            kind: V1Error::CompileError { content },
        } => {
            error!("Error compiling:\n{content}");
            return Err(CError::StrErr("Not compiled").into());
        }
        V1Response::Error { kind } => {
            return Err(kind.into());
        }
        _ => unreachable!(),
    }

    Ok(String::from("Finished"))
}
