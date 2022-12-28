use std::{convert, str};

enum Operation {
    Add,
    Multiply,
}

impl convert::TryFrom<char> for Operation {
    type Error = anyhow::Error;

    fn try_from(c: char) -> anyhow::Result<Self> {
        match c {
            '*' => Ok(Operation::Multiply),
            '+' => Ok(Operation::Add),
            _ => Err(anyhow::Error::msg(format!("no such operation `{}`", c))),
        }
    }
}

impl Operation {
    fn complete(&self, left: i64, right: i64) -> i64 {
        match self {
            Operation::Add => left + right,
            Operation::Multiply => left * right,
        }
    }
}

enum Identifier {
    Constant(i64),
    Variable,
}

impl Identifier {
    fn evaluate_or(&self, variable_value: i64) -> i64 {
        match self {
            Identifier::Constant(v) => *v,
            Identifier::Variable => variable_value,
        }
    }
}

impl str::FromStr for Identifier {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let trimmed_s = s.trim();
        if trimmed_s.is_empty() {
            Err(anyhow::Error::msg("empty identifier"))
        } else if let Ok(constant) = trimmed_s.parse() {
            Ok(Identifier::Constant(constant))
        } else {
            Ok(Identifier::Variable)
        }
    }
}
pub struct Phrase {
    left: Identifier,
    right: Identifier,
    operation: Operation,
}

impl Phrase {
    pub fn process(&self, variable: i64) -> i64 {
        self.operation.complete(
            self.left.evaluate_or(variable),
            self.right.evaluate_or(variable),
        )
    }
}

impl str::FromStr for Phrase {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut token = String::new();
        let mut phrase = Phrase {
            left: Identifier::Variable,
            right: Identifier::Variable,
            operation: Operation::Add, // temp value
        };
        let mut is_operation_found = false;
        for c in s.trim().chars().filter(|c| c.is_alphanumeric()) {
            if let Ok(op) = Operation::try_from(c) {
                // left-hand side
                if token.is_empty() {
                    return Err(anyhow::Error::msg(
                        "phrase malformed: missing left hand side",
                    ));
                }
                if let Ok(number) = token.parse() {
                    phrase.left = Identifier::Constant(number);
                }
                token.clear();

                // operation
                phrase.operation = op;
                is_operation_found = true;
            }
            token.push(c);
        }
        if !is_operation_found {
            return Err(anyhow::Error::msg("phrase malformed: operation not found"));
        }

        // right-hand side
        if token.is_empty() {
            return Err(anyhow::Error::msg(
                "phrase malformed: missing right hand side",
            ));
        }

        if let Ok(number) = token.parse() {
            phrase.right = Identifier::Constant(number);
        }

        Ok(phrase)
    }
}
