use std::{
    env, error, fs,
    io::{self, Read, Write},
};
fn main() -> Result<(), Box<dyn error::Error>> {
    let prog = fs::read(env::args().nth(1).unwrap())?;
    let mut pc = 0; /* Program counter tracks location in the code */
    let mut cells = vec![0u8; 8 * 1024 * 1024];  /* memory */
    let mut cc = 0; /* Cell counter (data pointer) points to active location in memory*/

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
                let mut loop_depth = 1;
                while loop_depth > 0 && pc < prog.len() - 1 {
                    pc += 1;
                    match prog[pc] as char {
                        '[' => loop_depth += 1,
                        ']' => loop_depth -= 1,
                        _ => (),
                    }
                }
            },
            ']' if cells[cc] != 0 => {
                let mut loop_depth = 1;
                while loop_depth > 0 && pc > 0 {
                    pc -= 1;
                    match prog[pc] as char {
                        ']' => loop_depth += 1,
                        '[' => loop_depth -= 1,
                        _ => (),
                    }
                }
            },
            '.' => io::stdout().write_all(&cells[cc..cc + 1])?,
            ',' => io::stdin().read_exact(&mut cells[cc..cc + 1])?,
            _ => (), 
        }
        pc += 1;
    }
    Ok(())
}
