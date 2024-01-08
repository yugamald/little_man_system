use std::num::ParseIntError;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Stop,
    Add(u8),
    Sub(u8),
    Sto(u8),
    Sta(u8),
    Load(u8),
    B(String),
    Bz(String),
    Bp(String),
    Read,
    Print,
}

#[derive(Debug, Clone)]
pub enum Token {
    Instruction(Instruction),
    Label(String),
}

#[derive(Debug)]
enum InstructionError {
    ParseIntError(ParseIntError),
    InvalidInstruction,
}

impl Instruction {
    fn from_token_parts(token_parts: &[&str]) -> Result<Token, InstructionError> {
        match token_parts {
            ["stop"] => Ok(Token::Instruction(Instruction::Stop)),
            ["read"] => Ok(Token::Instruction(Instruction::Read)),
            ["print"] => Ok(Token::Instruction(Instruction::Print)),
            ["add", idx] => Ok(Token::Instruction(Instruction::Add(idx.parse().map_err(InstructionError::ParseIntError)?))),
            ["sub", idx] => Ok(Token::Instruction(Instruction::Sub(idx.parse().map_err(InstructionError::ParseIntError)?))),
            ["sto", idx] => Ok(Token::Instruction(Instruction::Sto(idx.parse().map_err(InstructionError::ParseIntError)?))),
            ["sta", idx] => Ok(Token::Instruction(Instruction::Sta(idx.parse().map_err(InstructionError::ParseIntError)?))),
            ["load", idx] => Ok(Token::Instruction(Instruction::Load(idx.parse().map_err(InstructionError::ParseIntError)?))),
            ["b", label] => Ok(Token::Instruction(Instruction::B(label.to_string()))),
            ["bz", label] => Ok(Token::Instruction(Instruction::Bz(label.to_string()))),
            ["bp", label] => Ok(Token::Instruction(Instruction::Bp(label.to_string()))),
            _ => {
                if !token_parts.is_empty() && !token_parts[0].is_empty() && token_parts[0].starts_with(".") {
                    let token = token_parts[0];
                    if !token.is_empty() {
                        return Ok(Token::Label(token[1..token.len()].to_string()));
                    }
                }
                Err(InstructionError::InvalidInstruction)
            }
        }
    }
}

pub trait IntoTokens {
    fn into_tokens(&self) -> Result<Vec<Token>, (String, usize)>;
}

impl IntoTokens for &str {
    fn into_tokens(&self) -> Result<Vec<Token>, (String, usize)> {
        let mut tokens = Vec::new();
        for (index, line) in self.lines().enumerate() {
            let line = if let Some((code, _)) = line.split_once(';') {
                code.trim()
            } else {
                line.trim()
            };
            if line.is_empty() {
                continue;
            }

            let (mnemonic, rest) = line.split_once(' ').unwrap_or((line, ""));
            let mut parts: Vec<String> = vec![mnemonic.to_lowercase()];

            let mut rest_parts: Vec<String> = rest.split(',')
                .map(|s| s.trim().to_string())
                .filter(|p| !p.is_empty())
                .collect();
            parts.append(&mut rest_parts);
            drop(rest_parts);

            let parts_ref: Vec<&str> = parts.iter().map(|s| s.as_str()).collect();
            match Instruction::from_token_parts(&parts_ref) {
                Ok(instruction) => {
                    tokens.push(instruction);
                },
                Err(why) => {
                    return Err((format!("line {}: {why:?}", index+1), index));
                },
            }
        }
        Ok(tokens)
    }
}
