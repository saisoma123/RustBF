use std::{
    env, error, fs,
    io::{self, Read, Write},
};

#[derive(PartialEq)]
enum Ops {
    Left,
    Right,
    Add,
    Sub,
    LBrack(usize),
    RBrack(usize),
    Output,
    Input,
}



fn main() -> Result<(), Box<dyn error::Error>> {
    /* Notice: prog is now a vec of OpCodes, not a string */
    let mut prog = vec![];

    /* First parse the program into a sequence of opcodes */
    for b in fs::read(env::args().nth(1).unwrap())? {
        match b as char {
            '<' => prog.push(Ops::Left),
            '>' => prog.push(Ops::Right),
            '+' => prog.push(Ops::Add),
            '-' => prog.push(Ops::Sub),
            '[' => prog.push(Ops::LBrack(usize::max_value())),
            ']' => prog.push(Ops::RBrack(usize::max_value())),
            '.' => prog.push(Ops::Output),
            ',' => prog.push(Ops::Input),
            _ => ()
        }
    }

    let mut bstack = vec![]; // Used to track nested brackets

    let mut pc = 0;
    let mut cells = vec![0u8; 10000];
    let mut cc = 0;
    
    for pc in 0..prog.len() {
        if let Ops::LBrack(_) = prog[pc] {
            bstack.push(pc);
        }
        else if let Ops::RBrack(_) = prog[pc]{
            if let Some(&last_value) = bstack.last() {
                prog[pc] = Ops::RBrack(last_value);
                prog[last_value] = Ops::LBrack(pc);
                bstack.pop();
            }
            
        }
    }


    while pc < prog.len() {
        match prog[pc] {
            Ops::Right => {
                if cc + 1 < cells.len() {
                    cc += 1;
                }
            },
            Ops::Left => {
                if cc > 0 {
                    cc -= 1;
                }
            },
            Ops::Add => {
                cells[cc] += 1; 
            },
            Ops::Sub => {
                cells[cc] -= 1;
            },
            Ops::LBrack(value) if cells[cc] == 0 => pc = value,
            Ops::RBrack(value) if cells[cc] != 0 => pc = value,
            Ops::Output => io::stdout().write_all(&cells[cc..cc + 1])?,
            Ops::Input => io::stdin().read_exact(&mut cells[cc..cc + 1])?,
            _ => (), 
        }
        pc += 1;
    }
    Ok(())
}