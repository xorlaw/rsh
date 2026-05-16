#[derive(Clone)]
pub enum Redirect {
    Overwrite(String),
    Append(String),
    Input(String),
}

#[derive(Clone)]
pub struct Command {
    pub name:       String,
    pub args:       Vec<String>,
    pub redirects:  Vec<Redirect>,
}

#[derive(Clone)]
pub struct Pipeline {
    pub commands: Vec<Command>,
}

pub fn parse(input: &str) -> Option<Pipeline> {
    let segments: Vec<&str> = input.split('|').collect();

    let mut commands = Vec::new();

    for segment in segments {
        let mut tokens = tokenise(segment);
        if tokens.is_empty() {
            continue;
        }

        let name = tokens.remove(0);
        let mut args = Vec::new();
        let mut redirects = Vec::new();
        let mut i = 0;
        while i < tokens.len() {
            match tokens[i].as_str() {
                ">>" => {
                    if let Some(target_ = tokens.get(i + 1)) {
                        redirects.push(Redirect::Append(target.clone()));
                        i += 2;
                    } else {
                        eprintln!("rsh: parse: expected filename after '>>'");
                        i += 1;
                    }
                }
                ">" => {
                    if let Some(target) = tokens.get(i + 1) {
                        redirects.push(Redirect::Overwrite(target.clone()));
                        i += 2;
                    } else {
                        eprintln!("rsh: parse: expected filename after '>'");
                        i += 1;
                    }
                }
                "<" => {
                    if let Some(target) = tokens.get(i + 1) {
                        redirects.push(Redirect::Input(target.clone()));
                        i += 2;
                    } else {
                        eprintln!("rsh: parse: expected filename after '<'");
                        i += 1;
                    }
                }
                _ => { args.push(tokens[i].clone()); i += 1; }
            }
        }
        commands.push(Command { name, args, redirects });
    }

    if commands.is_empty() {
        return None;
    }

    Some(Pipeline { commands })
}

fn tokenise(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_single = false;
    let mut in_double = false;

    for c in input.chars() {
        match c {
            '\'' if !in_double => in_single = !in_single,
            '"'  if !in_single => in_double = !in_double,
            ' ' | '\t' if !in_single && !in_double => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
            }
            _ => current.push(c),
        }
    }

    if !current.is_empty() {
        tokens.push(current);
    }

    tokens
}
