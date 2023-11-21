use std::collections::HashMap;

#[derive(Debug, Clone)]

struct CityData {
    roads: Vec<Road>,
    visited: bool,
}

#[derive(Debug, Clone)]
struct Road {
    to: String,
    distance: u32,
}

fn santa(text: &str) -> u32 {

    let mut cities:HashMap<String,CityData>=HashMap::new();

    for line in text.split("\n") {
        add_road(line.trim(), &mut cities);
    }

    println!("Cities : {:?}", cities);
    get_full_map(&cities);
    0
}

fn add_road (text: &str, cities:&mut HashMap<String, CityData>) {
    let parts:Vec<&str>=text.clone().split(' ').collect();
 
    let distance = parts[4].parse::<u32>().unwrap();

    //cities.entry(parts[0].trim().to_string()).or_insert_with(CityData { roads: vec![Road { to: parts[2].trim().to_string(), distance}], visited: false});
    cities.entry(parts[0].trim().to_string()).or_insert_with(||CityData {
        roads: Vec::new(), visited: false})
        .roads
        .push(Road { to: parts[2].trim().to_string(), distance});
    
    cities.entry(parts[2].trim().to_string()).or_insert_with(||CityData {
        roads: Vec::new(), visited: false})
        .roads
        .push(Road { to: parts[0].trim().to_string(), distance});
    
}

fn get_full_map(cities:&HashMap<String, CityData>) {

    // Start by the least connected city
    let mut city_connections:u32=u32::MAX;
    let mut city:String="".to_string();

    for c in cities {
        let cn = c.1.roads.len() as u32;
        if cn<city_connections {
            city_connections=cn;
            city=c.0.to_string();
        }
    };

    println!("Least connected city : {city} with {city_connections} ");
    let mut mut_cities = cities.clone();

    get_path(city, &mut mut_cities, 0);    
}

fn get_path (city: String, cities:&mut HashMap<String, CityData>, current_distance:u32) -> u32 {
    println!("Looking roads for {city}");
    println!("Marking this city as visited");

    let Some(mut c) = cities.get_mut(&city) else{panic!("Not found city {}", city)};
    c.visited=true;

    if cities.len() ==1 {
        cities.iter().next().unwrap().1.roads.iter().for_each(|r| println!("\tFrom {city} to {} = {}",r.to, r.distance));
        return cities.iter().next().unwrap().1.roads[0].distance;
    }

    let mut new_cities:HashMap<String, CityData>=cities
    .iter()
    .filter(|(&ref c, d)|c != &city && d.visited==false)
    .map(|(k,l)| (k.clone(), l.clone()))
    .collect();

    new_cities.iter().for_each(|(&ref k,&ref _l)| {get_path(k.clone(), &mut new_cities.clone());});
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
        let actual = santa(paq);
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
/*        dotest("Faerun to Tristram = 65
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
 */
}

    #[test]
    fn prod_tests(){  
//        dotest("", 2); 
    }
}
/* */
