use std::{collections::HashMap, env};

/// Parser argument struct
pub struct Argument {
    /// The name of each argument
    pub name: String,
}

/// Parser struct itself
pub struct Parser {
    pub arguments: Vec<Argument>,
}

impl Parser {
    /// Create a new parser
    pub fn new() -> Parser {
        Parser { arguments: vec![] }
    }

    /// Add an argument to the parser
    pub fn add_arg(&mut self, name: &str) -> &mut Self {
        self.arguments.push(Argument {
            name: name.to_string(),
        });

        self
    }

    /// Parse the arguments based in expected added arguments
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
