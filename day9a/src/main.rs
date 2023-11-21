use std::{collections::HashMap};
use itertools::Itertools;

fn get_path(path: &str) -> u32{

let mut places = Vec::new();
let mut distances = HashMap::new();

for line in path.lines() {
    let line = line;
    let parts: Vec<&str> = line.split_whitespace().collect();
    let (source, dest, distance) = (parts[0], parts[2], parts[4].parse::<u16>().unwrap());

    places.push(source);
    places.push(dest);
    distances.entry(source).or_insert_with(HashMap::new).insert(dest, distance);
    distances.entry(dest).or_insert_with(HashMap::new).insert(source, distance);
}

places.sort();
places.dedup();

let (mut shortest, mut longest) = (u16::MAX, 0);

for items in places.iter().permutations(places.len()) {
    let dist: u16 = items.windows(2)
                         .map(|w| *distances[w[0]].get(w[1]).unwrap())
                         .sum();
    shortest = shortest.min(dist);
    longest = longest.max(dist);
}

println!("shortest: {}", shortest);
println!("longest: {}", longest);
0
}

fn main() {
}
#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(paq: &str, expected: u32) {
        println!();
        println!("-----------------------");
        let actual = get_path(paq);
        assert!(
            actual == expected,
            "Test failed with \nExpected {expected:?} but got {actual:?}"
        )
    }

    #[test]
    fn fixed_tests() {
        dotest("London to Dublin = 464
        London to Belfast = 518
        Dublin to Belfast = 141", 0); 
        dotest("Faerun to Tristram = 65
        Faerun to Tambi = 129
        Faerun to Norrath = 144
        Faerun to Snowdin = 71
        Faerun to Straylight = 137
        Faerun to AlphaCentauri = 3
        Faerun to Arbre = 149
        Tristram to Tambi = 63
        Tristram to Norrath = 4
        Tristram to Snowdin = 105
        Tristram to Straylight = 125
        Tristram to AlphaCentauri = 55
        Tristram to Arbre = 14
        Tambi to Norrath = 68
        Tambi to Snowdin = 52
        Tambi to Straylight = 65
        Tambi to AlphaCentauri = 22
        Tambi to Arbre = 143
        Norrath to Snowdin = 8
        Norrath to Straylight = 23
        Norrath to AlphaCentauri = 136
        Norrath to Arbre = 115
        Snowdin to Straylight = 101
        Snowdin to AlphaCentauri = 84
        Snowdin to Arbre = 96
        Straylight to AlphaCentauri = 107
        Straylight to Arbre = 14
        AlphaCentauri to Arbre = 46", 0);
 
}

    #[test]
    fn prod_tests(){  
//        dotest("", 2); 
    }
}