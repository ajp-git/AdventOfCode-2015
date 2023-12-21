use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum Instruction {
    hlf(char),
    tpl(char),
    inc(char),
    jmp(i32),
    jie(char, i32),
    jio(char, i32),
}

fn main() -> io::Result<()>{

    // Initialize an empty vector to hold the instructions
    let mut instructions: Vec<Instruction> = Vec::new();

    let mut registers:HashMap<char, i32>=[('a',1), ('b', 0)].into_iter().collect();

    let instructions_file=File::open("input-23.txt")?;
    let b_reader=io::BufReader::new(instructions_file);

    for line in b_reader.lines() {
        let line=line?;
        let parts:Vec<&str>=line.split(|c: char| c.is_whitespace()||c==',')
                .filter(|s| !s.is_empty())
                .collect();

        match parts.as_slice() {
            ["hlf", r] => instructions.push(Instruction::hlf(r.chars().next().unwrap())),
            ["tpl", r] => instructions.push(Instruction::tpl(r.chars().next().unwrap())),
            ["inc", r] => instructions.push(Instruction::inc(r.chars().next().unwrap())),
            ["jmp", a] => instructions.push(Instruction::jmp(a.parse::<i32>().unwrap())),
            ["jie", r, val] => instructions.push(Instruction::jie(r.chars().next().unwrap(), val.parse::<i32>().unwrap())),
            ["jio", r, val] => instructions.push(Instruction::jio(r.chars().next().unwrap(), val.parse::<i32>().unwrap())),
            _ => panic!("Unkown instruction {}", line),
        }
    }

    let mut pos: i32 = 0;
    while pos < instructions.len() as i32 {
        let instruction=instructions.get(pos as usize).unwrap();
        println!("Instruction {:?} : {:?}", pos, instruction);
        match instruction {
            Instruction::hlf(r) => {
                let val = registers.get(r).unwrap();
                registers.insert(*r, val/2);
                pos+=1;
            },
            Instruction::tpl(r) => {
                let val=registers.get(r).unwrap();
                registers.insert(*r, val*3);
                pos+=1;
            },
            Instruction::inc(r) => {
                let val = registers.get(r).unwrap();
                registers.insert(*r, val+1);
                pos+=1;
            },
            Instruction::jmp(offset) => {
                pos=pos + *offset;
            },
            Instruction::jie(r,offset) => {
                let val=registers.get(r).unwrap();
                if val%2==0 {
                    pos+=*offset;
                } else {
                    pos+=1;
                }
            },
            Instruction::jio(r, offset) => {
                let val=registers.get(r).unwrap();
                if *val==1 {
                    pos+=*offset;
                } else {
                    pos+=1;
                }
            }
        }
    } 

    println!("Registers {:?}", registers);
    Ok(())
}

