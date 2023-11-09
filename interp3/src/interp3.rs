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
    LBrack,
    RBrack,
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
            '[' => prog.push(Ops::LBrack),
            ']' => prog.push(Ops::RBrack),
            '.' => prog.push(Ops::Output),
            ',' => prog.push(Ops::Input),
            _ => ()
        }
    }

    let mut bmap = vec![0; 1024 * 1024 * 8]; // Map from a position in the program to the jump location
    let mut bstack = vec![]; // Used to track nested brackets

    let mut pc = 0;
    let mut cells = vec![0u8; 10000];
    let mut cc = 0;
    
    for pc in 0..prog.len() {
        if let Ops::LBrack = prog[pc] {
            bstack.push(pc);
        }
        else if let Ops::RBrack = prog[pc]{
            if let Some(&last_value) = bstack.last() {
                bmap[last_value] = pc;
                bmap[pc] = last_value;
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
            Ops::LBrack if cells[cc] == 0 => {
                pc = bmap[pc];
            },
            Ops::RBrack if cells[cc] != 0 => {
                pc = bmap[pc];
            },
            Ops::Output => io::stdout().write_all(&cells[cc..cc + 1])?,
            Ops::Input => io::stdin().read_exact(&mut cells[cc..cc + 1])?,
            _ => (), 
        }
        pc += 1;
    }
    Ok(())
}
