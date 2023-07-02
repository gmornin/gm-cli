use std::collections::HashMap;

pub fn args_parse(cmd_args: &mut Vec<String>, map: &mut HashMap<String, String>) {
    let mut to_remove = Vec::new();

    cmd_args.iter().enumerate().for_each(|(i, arg)| {
        if !arg.starts_with("--") {
            return;
        }

        if to_remove.last() != Some(&i) {
            to_remove.push(i);
        }

        if cmd_args.len() > i + 1 {
            to_remove.push(i + 1);
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

    to_remove.iter().rev().for_each(|i| {
        cmd_args.remove(*i);
    });
}
