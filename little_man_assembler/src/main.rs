use std::{
    env,
    fs::File,
    io::{self, BufReader, Read, BufWriter, Write},
    path::Path,
    process::exit,
};

mod lexer;
use lexer::{IntoTokens, Token, Instruction};
mod assemble;
use assemble::assemble;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        exit(1);
    }

    let filename = &args[1];
    let path = Path::new(filename);

    let file = File::open(&path)?;
    let mut reader = BufReader::new(file);

    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer)?;

    let assembly = String::from_utf8_lossy(&buffer).to_string();
    let tokens = assembly.as_str().into_tokens().unwrap();
    let intcode = assemble(tokens).unwrap();
    let mut bytecode = Vec::with_capacity(intcode.len()*2);
    for code in intcode {
        bytecode.push((code & 0xff) as u8);
        bytecode.push((code >> 8) as u8);
    }

    let file = File::create("a.out")?;
    let mut writer = BufWriter::new(file);
    writer.write_all(&bytecode).unwrap();

    Ok(())
}
