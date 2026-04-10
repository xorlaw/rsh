use crate::error::RshError;
use std::env;
use std::path::PathBuf;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

static ALIASES: LazyLock<Mutex<HashMap<String, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

const MAX_NAME_LEN: usize = 64;
const MAX_VALUE_LEN: usize = 2048;

fn is_valid_name(name:&str) -> bool {
    !name.is_empty()
        && name.len() <= MAX_NAME_LEN
        && name.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-')
}

fn alias_set(name: &str, value: &str) -> Result<(), String> {
    if !is_valid_name(name) {
        return Err(format!("rsh: alias: invalid name '{name}' - only letters, numbers, _ and - allowed"));
    }
    if value.len() > MAX_VALUE_LEN {
        return Err(format!("rsh: alias: value too long (max {MAX_VALUE_LEN} characters)"));
    }
    ALIASES.lock().unwrap().insert(name.to_string(), value.to_string());
    Ok(())
}



pub fn expand_alias(name: &str, seen: &mut std::collections::HashSet<String>) -> Option<String> {
    if !seen.insert(name.to_string()) {
        return None;
    }
    ALIASES.lock().unwrap().get(name).cloned()
}



fn alias_remove(name: &str) {
    ALIASES.lock().unwrap().remove(name);
}

fn alias_list() -> Vec<(String, String)> {
    let mut entries: Vec<_> = ALIASES
        .lock()
        .unwrap()
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    entries
}

// Runs a builtin if the command matches one.
// Returns None if it's not a builtin.
pub fn run(name: &str, args: &[&str]) -> Option<Result<(), RshError>> {
    match name {
        "exit" | "quit" => std::process::exit(0),

        "cd" => Some(cd(args)),

        "export" => Some(export(args)),




        "help" => {
            println!("rsh - the small, secure shell");
            println!("builtins: cd, exit, quit, help");
            Some(Ok(()))
        }
        
        "alias" => Some(alias(args)),
        "unalias" => Some(unalias(args)),

        _ => None,
    }
}

fn cd(args: &[&str]) -> Result<(), RshError> {
    let target = args.first().copied().unwrap_or("~");

    let path = if target == "~" {
        env::var("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/"))
    } else {
        PathBuf::from(target)
    };

    env::set_current_dir(&path)
        .map_err(|e| RshError::CdFailed(path.display().to_string(), e))
}

fn export(args: &[&str]) -> Result<(), RshError> {
    if args.is_empty() {
        eprintln!("rsh: export: expected KEY=VALUE");
        return Ok(());
    }

    for arg in args {
        match arg.split_once('=') {
            Some((key, val)) if !key.is_empty() => {
                env::set_var(key, val);
            }
            Some((_, _)) => {
                eprintln!("rsh: export: invalid variable name in '{arg}'");
            }
            None => eprintln!("rsh: export: expected KEY=VALUE, instead got '{arg}'"),
        }
    }
    Ok(())
}

fn alias(args: &[&str]) -> Result<(), RshError> {
    if args.is_empty() {
        for (name, value) in alias_list() {
            println!("alias {name}='{value}'");
        }
        return Ok(());
    }

    let joined = args.join(" ");

    match joined.split_once('=') {
        Some((name, value)) => {
            let name = name.trim();
            let value = value.trim();
            if let Err(e) = alias_set(name, value) {
                eprintln!("{e}");
            }
        }
        None => {
            match expand_alias(joined.trim(), &mut std::collections::HashSet::new()) {
                Some(val) => println!("alias {}='{val}'", joined.trim()),
                None      => eprintln!("rsh: alias: {}: not found", joined.trim()),
            }
        }
    }

    Ok(())
}

fn unalias(args: &[&str]) -> Result<(), RshError> {
    if args.is_empty() {
        eprintln!("rsh: unalias: expected at least one name");
        return Ok(());
    }
    
    for arg in args {
        alias_remove(arg);
    }
    Ok(())
}


