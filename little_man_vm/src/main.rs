use std::{
    env,
    fs::File,
    io::{self, BufReader, Read},
    path::Path,
    process::exit,
};

mod vm;
use vm::LittleManVirtualMachine;

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

    let mut program = Vec::with_capacity(buffer.len());

    for pair in buffer.chunks_exact(2) {
        let n = i16::from_le_bytes([pair[0], pair[1]]);
        program.push(n);
    }

    let mut lmc = LittleManVirtualMachine::new()
        .with_program(&program, 0)
        .unwrap();

    loop {
        match lmc.execute_one() {
            Ok(halt) => {
                if halt {
                    break;
                }
            },
            Err(err) => {
                eprintln!("{err:?}");
                exit(1);
            },
        }
    }

    exit(0);
}
