use std::{collections::HashMap, env};

pub struct Argument {
    pub name: String,
}

pub struct Parser {
    pub arguments: Vec<Argument>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser { arguments: vec![] }
    }

    pub fn add_arg(&mut self, name: &str) -> &mut Self {
        self.arguments.push(Argument {
            name: name.to_string(),
        });

        self
    }

    pub fn parse(&self) -> HashMap<String, String> {
        let args: Vec<_> = env::args().skip(1).collect();

        args.into_iter()
            .zip(&self.arguments)
            .fold(HashMap::new(), |mut params, (value, arg)| {
                params.insert(arg.name.to_string(), value);
                params
            })
    }
}
