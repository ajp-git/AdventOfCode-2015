use std::collections::{HashMap, HashSet};
use std::{path::Path, fs::File, io};
use std::io::BufRead;

struct Key {
    name:String,
    replacements:Vec<String>,
}

fn main() -> io::Result<()> {
    part2()
}
fn part2() -> io::Result<()>{

    let path = Path::new("input-19.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut keys:HashMap<String, String>=HashMap::new();

    // Read file and fill the HashMap
    reader
        .lines()
        .filter_map(Result::ok)
        .for_each(|l|{
            if let Some((left,right)) = l.split_once(" => ") {
                keys.insert(right.to_string(), left.to_string());
            }
        });

    println!("{keys:?}");

    // parse input formula and push result in a HasMap to avoid duplicates
    static  INPUT_FORMULA:&str = "CRnCaCaCaSiRnBPTiMgArSiRnSiRnMgArSiRnCaFArTiTiBSiThFYCaFArCaCaSiThCaPBSiThSiThCaCaPTiRnPBSiThRnFArArCaCaSiThCaSiThSiRnMgArCaPTiBPRnFArSiThCaSiRnFArBCaSiRnCaPRnFArPMgYCaFArCaPTiTiTiBPBSiThCaPTiBPBSiRnFArBPBSiRnCaFArBPRnSiRnFArRnSiRnBFArCaFArCaCaCaSiThSiThCaCaPBPTiTiRnFArCaPTiBSiAlArPBCaCaCaCaCaSiRnMgArCaSiThFArThCaSiThCaSiRnCaFYCaSiRnFYFArFArCaSiRnFYFArCaSiRnBPMgArSiThPRnFArCaSiRnFArTiRnSiRnFYFArCaSiRnBFArCaSiRnTiMgArSiThCaSiThCaFArPRnFArSiRnFArTiTiTiTiBCaCaSiRnCaCaFYFArSiThCaPTiBPTiBCaSiThSiRnMgArCaF";

    let mut formula=INPUT_FORMULA.clone().to_string();

    let mut done=false;
    let mut count=0;
    while !done {
        done=true;
        for (key,val) in &keys{
            if let Some(pos) = formula.find(key){
                let (left, right)=formula.split_at(pos);
                    let right=right.to_string().split_off(key.len());
                    formula=format!("{left}{val}{right}");
                    done=false;
                    count+=1;
                }
            }
    }

    println!("Count {count}");
    Ok(())
}

fn part1() -> io::Result<()> {
    let path = Path::new("input-19.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut keys:HashMap<String, String>=HashMap::new();

    // Read file and fill the HashMap
    reader
        .lines()
        .filter_map(Result::ok)
        .for_each(|l|{
            if let Some((left,right)) = l.split_once(" => ") {
                keys.insert(right.to_string(), left.to_string());
            }
        });

    println!("{keys:?}");

    // parse input formula and push result in a HasMap to avoid duplicates
    static  INPUT_FORMULA:&str = "CRnCaCaCaSiRnBPTiMgArSiRnSiRnMgArSiRnCaFArTiTiBSiThFYCaFArCaCaSiThCaPBSiThSiThCaCaPTiRnPBSiThRnFArArCaCaSiThCaSiThSiRnMgArCaPTiBPRnFArSiThCaSiRnFArBCaSiRnCaPRnFArPMgYCaFArCaPTiTiTiBPBSiThCaPTiBPBSiRnFArBPBSiRnCaFArBPRnSiRnFArRnSiRnBFArCaFArCaCaCaSiThSiThCaCaPBPTiTiRnFArCaPTiBSiAlArPBCaCaCaCaCaSiRnMgArCaSiThFArThCaSiThCaSiRnCaFYCaSiRnFYFArFArCaSiRnFYFArCaSiRnBPMgArSiThPRnFArCaSiRnFArTiRnSiRnFYFArCaSiRnBFArCaSiRnTiMgArSiThCaSiThCaFArPRnFArSiRnFArTiTiTiTiBCaCaSiRnCaCaFYFArSiThCaPTiBPTiBCaSiThSiRnMgArCaF";

    let formula=INPUT_FORMULA.clone();

    let mut h:HashSet<String>=HashSet::new();

    for (key,val) in &keys{
        for mol in formula.match_indices(val.as_str()) {
            let (left,right)=formula.split_at(mol.0);
            let right=right.to_string().split_off(val.len());
            h.insert(format!("{left}{key}{right}"));
            println!("{formula}");
            println!("Val : {val}");
            println!("left : {} + Key: {key} + right : {}", left, right);
        }     
    }
    
    println!("Count : {}", h.len());
    println!("All keys{:?} : Count {}",keys, keys.len());
    Ok(())
}
