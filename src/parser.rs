pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}

pub fn parse(input: &str) -> Option<Command> {
    let mut tokens = tokenise(input);
    if tokens.is_empty() {
        return None;
    }
    let name = tokens.remove(0);
    Some(Command { name, args: tokens })
}

fn tokenise(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut chars = input.chars().peekable();
    let mut in_single = false;
    let mut in_double = false;

    while let Some(c) = chars.next() {
        match c {
            '\'' if !in_double => in_single = !in_single,
            '"'  if !in_single => in_double = !in_double,
            ' ' | '\t' if !in_single && !in_double => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
            }
            _ => current.push(c)
        }
    }

    if !current.is_empty() {
        tokens.push(current);
    }

    tokens
}
