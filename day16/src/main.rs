use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

#[derive(Debug, Clone)]
struct Sue {
    number:u32,
    attributes:HashMap<String,u8>,
}

fn main() -> io::Result<()> {
    part2()
}

fn part2() -> io::Result<()> {
    match env::current_dir() {
        Ok(current_path) => {
            println!("The current directory is {}", current_path.display());
        }
        Err(e) => {
            println!("Error occurred while getting the current directory: {}", e);
        }
    }

    let path = Path::new("input16.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let re_sue=Regex::new(r"Sue (\d+): (.*)").unwrap();
    let re_data=Regex::new(r"(\w+): (\d+)").unwrap();

    let mut v_sue:Vec<Sue>=Vec::new();
    
    for line in reader.lines() {
        match line {
            Ok(l) => {
                if let Some(sue) = re_sue.captures(&l) {
                    let sue_number:u32=sue[1].parse().unwrap();
                    let sue_attributes=&sue[2];

                    let mut h_attrib: HashMap<String, u8> = HashMap::new();
                    for attrib in re_data.captures_iter(&sue_attributes) {
                        let attrib_name = attrib[1].to_string();
                        let attrib_value:u8=attrib[2].parse().unwrap();
                        h_attrib.insert(attrib_name, attrib_value);

                    }
                    v_sue.push(Sue { number: sue_number, attributes: h_attrib});
                }
            }
            Err(e) => {panic!("Error reading line: {}", e)},
        }
    }
    let mut max_match_value=0;
    let mut max_match_sue;

    let v_find_sue:HashMap<String,u8>=vec![
        ("children".to_string(),3),
        ("cats".to_string(),7),
        ("samoyeds".to_string(),2),
        ("pomeranians".to_string(),3),
        ("akitas".to_string(),0),
        ("vizslas".to_string(),0),
        ("goldfish".to_string(),5),
        ("trees".to_string(),3),
        ("cars".to_string(),2),
        ("perfumes".to_string(),1)].into_iter().collect();

    for s in v_sue.clone() {
        let m = v_find_sue.iter().filter(|(&ref k, &v)| 
            if let Some(&a) = s.attributes.get(k) {
                match k.as_str() {
                    "cats" | "trees" => { a > v },
                    "pomeranians" | "goldfish" => {a < v },
                    _ => a==v,
                }
            } else {
                false
            }
        ).count();
        if m>max_match_value{
            max_match_value=m;
            max_match_sue=s.number;
            println!("Found a better sue : {} matches : {}", max_match_sue, max_match_value);
        }
    }

    Ok(())}

fn part1() -> io::Result<()> {
        match env::current_dir() {
        Ok(current_path) => {
            println!("The current directory is {}", current_path.display());
        }
        Err(e) => {
            println!("Error occurred while getting the current directory: {}", e);
        }
    }

    let path = Path::new("input16.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let re_sue=Regex::new(r"Sue (\d+): (.*)").unwrap();
    let re_data=Regex::new(r"(\w+): (\d+)").unwrap();

    let mut v_sue:Vec<Sue>=Vec::new();
    
    for line in reader.lines() {
        match line {
            Ok(l) => {
                if let Some(sue) = re_sue.captures(&l) {
                    let sue_number:u32=sue[1].parse().unwrap();
                    let sue_attributes=&sue[2];

                    let mut h_attrib: HashMap<String, u8> = HashMap::new();
                    for attrib in re_data.captures_iter(&sue_attributes) {
                        let attrib_name = attrib[1].to_string();
                        let attrib_value:u8=attrib[2].parse().unwrap();
                        h_attrib.insert(attrib_name, attrib_value);

                    }
                    v_sue.push(Sue { number: sue_number, attributes: h_attrib });
                }
            }
            Err(e) => {panic!("Error reading line: {}", e)},
        }
    }
    let mut max_match_value=0;
    let mut max_match_sue=0;

    let v_find_sue:HashMap<String,u8>=vec![
        ("children".to_string(),3),
        ("cats".to_string(),7),
        ("samoyeds".to_string(),2),
        ("pomeranians".to_string(),3),
        ("akitas".to_string(),0),
        ("vizslas".to_string(),0),
        ("goldfish".to_string(),5),
        ("trees".to_string(),3),
        ("cars".to_string(),2),
        ("perfumes".to_string(),1)].into_iter().collect();

    for s in v_sue.clone() {
        let m = v_find_sue.iter().filter(|(&ref k, &v)| 
            if let Some(&a) = s.attributes.get(k) {
                a==v
            } else {
                false
            }
        ).count();
        if m>max_match_value{
            max_match_value=m;
            max_match_sue=s.number;
            println!("Found a better sue : {} matches : {}", max_match_sue, max_match_value);
        }
    }

    Ok(())
}