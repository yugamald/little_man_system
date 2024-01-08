use std::collections::BTreeMap;

use super::{Token, Instruction};

#[derive(Debug)]
pub enum AssemblyError {
    IndexOutOfRange(usize),
    LabelDoesNotExist(String),
}

pub fn assemble(input: Vec<Token>) -> Result<Vec<i16>, AssemblyError> {
    let mut labels = BTreeMap::new();
    let mut instructions = Vec::with_capacity(input.len());
    let mut preprocessing = Vec::with_capacity(input.len());

    let mut last_token = None;
    for token in input {
        match token.clone() {
            Token::Instruction(i) => {
                if let Some(Token::Label(l)) = last_token {
                    preprocessing.push((Some(l), i));
                } else {
                    preprocessing.push((None, i));
                }
            },
            Token::Label(_) => { },
        }
        last_token = Some(token);
    }

    for (index, (label, instruction)) in preprocessing.iter().enumerate() {
        if let Some(label) = label {
            labels.insert((*label).clone(), index);
        }
        instructions.push(instruction);
    }

    let mut bincode = Vec::with_capacity(instructions.len());
    for i in instructions {
        let code = match i {
            Instruction::Stop => Ok(000),
            Instruction::Add(n) => assemble_code(100, *n),
            Instruction::Sub(n) => assemble_code(200, *n),
            Instruction::Sto(n) => assemble_code(300, *n),
            Instruction::Sta(n) => assemble_code(400, *n),
            Instruction::Load(n) => assemble_code(500, *n),
            Instruction::B(l) => assemble_label(600, &l, &labels),
            Instruction::Bz(l) => assemble_label(700, &l, &labels),
            Instruction::Bp(l) => assemble_label(800, &l, &labels),
            Instruction::Read => Ok(901),
            Instruction::Print => Ok(902),
        }?;
        bincode.push(code);
    }
    Ok(bincode)
}

fn assemble_label(
    base: i16,
    label: &String,
    labels: &BTreeMap<String, usize>
) -> Result<i16, AssemblyError> {
    if let Some(idx) = labels.get(label) {
        if *idx > 99 {
            return Err(AssemblyError::IndexOutOfRange(*idx));
        }
        Ok(base + (*idx as i16))
    } else {
        Err(AssemblyError::LabelDoesNotExist(label.to_string()))
    }
}

fn assemble_code(base: i16, idx: u8) -> Result<i16, AssemblyError> {
    if idx > 99 {
        return Err(AssemblyError::IndexOutOfRange(idx as usize));
    }
    Ok(base + idx as i16)
}
