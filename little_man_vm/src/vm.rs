use std::{collections::VecDeque, usize};

#[derive(Debug)]
pub struct LittleManVirtualMachine {
    input: VecDeque<i16>,
    output: Option<i16>,
    mailboxes: [i16; 100],
    ic: usize,
    acc: i16,
    zero: bool,
    positive: bool,
}

#[derive(Debug)]
pub enum LmvmError {
    ProgramDoesNotFit,
    InstructionCounterOutOfBounds,
    NumberOutOfRange(i16),
    InvalidInstruction(i16),
}

/// uhh based
impl LittleManVirtualMachine {
    pub fn new() -> LittleManVirtualMachine {
        LittleManVirtualMachine {
            input: VecDeque::new(),
            output: None,
            mailboxes: [0i16; 100],
            ic: 0,
            acc: 0,
            positive: true,
            zero: true,
        }
    }

    /// executes one instruction, returns true if halt.
    pub fn execute_one(&mut self) -> Result<bool, LmvmError> {
        if self.ic > 99 {
            return Err(LmvmError::InstructionCounterOutOfBounds);
        }

        let instruction = self.mailboxes[self.ic];
        self.ic += 1;

        match instruction {
            000 => return Ok(true),
            100..=199 => {
                let mailbox_idx = instruction - 100;
                let mailbox_value = self.mailboxes[mailbox_idx as usize];
                self.acc += mailbox_value;
                self.check_acc()?;
                self.set_flags();
            },
            200..=299 => {
                let mailbox_idx = instruction - 200;
                let mailbox_value = self.mailboxes[mailbox_idx as usize];
                self.acc -= mailbox_value;
                self.check_acc()?;
                self.set_flags();
            },
            300..=399 => {
                let mailbox_idx = instruction - 300;
                self.mailboxes[mailbox_idx as usize] = self.acc;
            },
            400..=499 => {
                let mailbox_idx = instruction - 400;
                self.mailboxes[mailbox_idx as usize] = self.acc % 100;
            },
            500..=599 => {
                let mailbox_idx = instruction - 500;
                self.acc = self.mailboxes[mailbox_idx as usize];
                self.check_acc()?;
                self.set_flags();
            },
            600..=699 => {
                self.ic = (instruction - 600) as usize;
            },
            700..=799 => {
                let new_ic = instruction - 700;
                if self.zero {
                    self.ic = new_ic as usize;
                }
            },
            800..=899 => {
                let new_ic = instruction - 800;
                if self.positive {
                    self.ic = new_ic as usize;
                }
            },
            901 => {
                if let Some(value) = self.input.pop_back() {
                    self.acc = value;
                    self.check_acc()?;
                    self.set_flags();
                }
            },
            902 => {
                self.output = Some(self.acc);
            },
            _ =>  return Err(LmvmError::InvalidInstruction(instruction)),
        };

        if let Some(value) = self.output {
            println!("OUTPUT: {value}");
            self.output = None;
        }

        Ok(false)
    }

    /// Loads an input into self
    pub fn with_input(mut self, input: &[i16]) -> LittleManVirtualMachine {
        for item in input {
            self.input.push_front(*item);
        }
        self.input.push_front(0);
        self
    }

    /// Loads the mailboxes with values in `program` at index `at`
    pub fn with_program(mut self, program: &[i16], at: usize) -> Result<LittleManVirtualMachine, LmvmError> {
        if at + program.len() > 99 {
            Err(LmvmError::ProgramDoesNotFit)
        } else {
            for (idx, unit) in program.iter().enumerate() {
                self.mailboxes[idx + at] = *unit;
            }
            Ok(self)
        }
    }

    /// sets flags based on acc
    fn set_flags(&mut self) {
        if self.acc == 0 {
            self.zero = true;
            self.positive = true;
        } else if self.acc < 0 {
            self.zero = false;
            self.positive = false;
        } else {
            self.zero = false;
            self.positive = true;
        }
    }

    /// yields an error if the number in `acc` is not representable
    fn check_acc(&self) -> Result<(), LmvmError> {
        if self.acc < -500 || self.acc > 499 {
            return Err(LmvmError::NumberOutOfRange(self.acc));
        }
        Ok(())
    }
}

