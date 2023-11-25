use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use itertools::Itertools;

fn factorial(n: usize) -> usize {
    (1..=n).product()
}

fn main() -> io::Result<()> {
    match env::current_dir() {
        Ok(current_path) => {
            println!("The current directory is {}", current_path.display());
        }
        Err(e) => {
            println!("Error occurred while getting the current directory: {}", e);
        }
    }

    let path = Path::new("input13.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut neighbors: HashMap<(String, String), i32> = HashMap::new();

    for line in reader.lines() {
        match line {
            Ok(l) => {
                let parts: Vec<&str> = l.split_whitespace().collect();
                let name = parts[0].to_string();
                let happiness_change =
                    parts[3].parse::<i32>().unwrap() * if parts[2] == "lose" { -1 } else { 1 };
                let mut neighbor = parts[10].to_string();
                neighbor.remove(parts[10].len()-1);
                neighbors.insert((name.clone(), neighbor.clone()), happiness_change);
                //neighbors.insert((neighbor, name), happiness_change); // Add reverse pair
            },      
            Err(e) => {
                println!("Error on line {}", e);
            }
        }
    }

    println!("Neighbors {:?}", neighbors);
/*    for n in neighbors.clone().keys(){
        print!("\t{:?}",n );
    } */
    let mut persons: Vec<String> = neighbors.iter().map(|((p, _n), _h)| p.clone()).collect();
    persons.sort();
    persons.dedup();

//    println!("Persons {:?}", persons);

    let mut permutation_count:u32 = 0;
    let fact=factorial(persons.len());
    let mut total: i32 = 0;

    for p in persons.iter().permutations(persons.len()) {
        let mut local_total: i32 = 0;
        let pp = p.iter().peekable();
        for (curr_person, next_person) in pp.clone().zip(pp.clone().cycle().skip(1)).map(|(a, b)| (a.to_string(), b.to_string())) {
//            println!("Looking for key: {:?}", (curr_person.clone(), next_person.clone()));

            if let Some(happy) = neighbors.get(&(curr_person.clone(), next_person.clone())) {
                local_total += *happy;
            }
            if let Some(happy) = neighbors.get(&(next_person.clone(), curr_person.clone())) {
                local_total += *happy;
            }
        }
        println!("Best so far : {:?} : {local_total}", pp);

//        println!("Found Total : {}", local_total);
        if local_total > total {
            total = local_total;
        }
        permutation_count+=1;
/*        if permutation_count%100==0 {
            println!("Permutations Count: {} / {}", permutation_count, fact);
        }  */
    }
    println!("\n\n\tTotal : {}", total);

    Ok(())
}
