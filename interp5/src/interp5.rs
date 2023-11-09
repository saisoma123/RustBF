use std::{
    env, error, fs,
    io::{self, Read, Write},
};
enum Ops {
    Left,
    Right,
    Add(u8),
    Sub,
    LBrack(usize),
    RBrack(usize),
    Output,
    Input,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut prog = vec![];
    let bytes = fs::read(env::args().nth(1).unwrap())?;
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] as char {
            '<' => prog.push(Ops::Left),
            '>' => prog.push(Ops::Right),
            '+' => {
                let count = bytes[i..].iter()
                             .take_while(|&&b| b == ('+' as u8))
                             .count();
                prog.push(Ops::Add(count as u8));
                i = i + count - 1;
            },
            '-' => prog.push(Ops::Sub),
            '[' => prog.push(Ops::LBrack(usize::max_value())),
            ']' => prog.push(Ops::RBrack(usize::max_value())),
            '.' => prog.push(Ops::Output),
            ',' => prog.push(Ops::Input),
            _ => ()
        }
        i += 1;
    }

    let mut bstack = vec![];

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

    let mut cells = vec![0u8; 10000];
    let mut cc = 0;
    let mut pc = 0;
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
            Ops::Add(value) => {
                cells[cc] += value; 
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


