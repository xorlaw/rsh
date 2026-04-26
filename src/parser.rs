pub struct Command {
    pub name:   String,
    pub args:   Vec<String>,
}

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
        commands.push(Command { name, args: tokens });
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
