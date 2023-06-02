use std::collections::HashMap;

pub fn args_parse(cmd_args: &[String], map: &mut HashMap<String, String>) {
    cmd_args.iter().enumerate().for_each(|(i, arg)| {
        if !arg.starts_with("--") {
            return;
        }

        let key = arg[2..].to_string();
        if map.contains_key(&key) {
            panic!("duplicated argument: {}", &key);
        }

        let _ = match cmd_args.get(i + 1) {
            Some(next) if !next.starts_with("--") => map.insert(key, next.clone()),
            None | Some(_) => map.insert(key, String::new()),
        };
    });
}
