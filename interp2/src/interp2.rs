use std::{
    env, error, fs,
    io::{self, Read, Write},
};
fn main() -> Result<(), Box<dyn error::Error>> {
    let prog = fs::read(env::args().nth(1).unwrap())?;

    // "b" is for bracket
    let mut bmap = vec![0; 1024 * 1024 * 8]; // Map from a position in the program to the jump location
    let mut bstack = vec![]; // Used to track nested brackets

    let mut pc = 0;
    let mut cells = vec![0u8; 10000];
    let mut cc = 0;
    
    for pc in 0..prog.len() {
        if prog[pc] as char == '['{
            bstack.push(pc);
        }
        else if prog[pc] as char == ']'{
            if let Some(&last_value) = bstack.last() {
                bmap[last_value] = pc;
                bmap[pc] = last_value;
                bstack.pop();
            }
        }
    }


    while pc < prog.len() {
        match prog[pc] as char {
            '>' => {
                if cc + 1 < cells.len() {
                    cc += 1;
                }
            },
            '<' => {
                if cc > 0 {
                    cc -= 1;
                }
            },
            '+' => {
                cells[cc] += 1; 
            },
            '-' => {
                cells[cc] -= 1;
            },
            '[' if cells[cc] == 0 => {
                pc = bmap[pc];
            },
            ']' if cells[cc] != 0 => {
                pc = bmap[pc];
            },
            '.' => io::stdout().write_all(&cells[cc..cc + 1])?,
            ',' => io::stdin().read_exact(&mut cells[cc..cc + 1])?,
            _ => (), 
        }
        pc += 1;
    }
    Ok(())
}
