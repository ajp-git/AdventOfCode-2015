use std::{io, fs::File};
use std::io::{BufRead, BufReader};

fn main() -> io::Result<()>{
    let mut packages: Vec<u32> = Vec::new();

    let packages_file=File::open("input-24.txt")?;
    let b_reader=io::BufReader::new(packages_file);

    for line in b_reader.lines() {
        let line=line?;
        packages.push(line.parse::<u32>().unwrap());
    }

    let packages_total_weight:u32=packages.iter().map(|w| w).sum();
    println!("{:?}", packages);
    println!("Total : {} /4 = {}", packages_total_weight, packages_total_weight/4);


    let mut current_combination:Vec<u32>=Vec::new();

    let mut results:Vec<Vec<u32>> = Vec::new();

    find_combinations(&packages, &mut results, &mut current_combination, 0, packages_total_weight/4);

//    println!("{:?}", results);


    // Calculate the products with details
    let mut products_with_details = calculate_products_with_details(results);

    // Sort the products with details by the product in ascending order
    products_with_details.sort_unstable_by_key(|&(prod, _)| prod);

    // Take the first 10 products with details
    let first_ten = products_with_details.iter().take(10);

    // Display the first 10 products with the numbers that were multiplied
    for (index, &(product, ref factors)) in first_ten.enumerate() {
        println!("Product {}: {} (factors: {:?})", index + 1, product, factors);
    }
    Ok(())
}

fn calculate_products_with_details(combinations: Vec<Vec<u32>>) -> Vec<(u64, Vec<u32>)> {
    combinations
        .into_iter()
        .filter_map(|inner_vec| {
            // Calculate the product as u64 to avoid overflow.
            let product = inner_vec.iter().fold(Some(1_u64), |acc, &num| {
                acc.and_then(|acc| acc.checked_mul(num as u64))
            });
            product.map(|prod| (prod, inner_vec.clone())) // Pair the product with the combination
        })
        .collect()
}

fn find_combinations(boxes: &Vec<u32>, results:& mut Vec<Vec<u32>>, current_combination: &mut Vec<u32>, index:u32, wanted_sum:u32 )
{
    let current_sum:u32=current_combination.iter().sum();
    if current_sum > wanted_sum{return;}
    if current_sum == wanted_sum { results.push(current_combination.clone()); return; }

    //println!("Trying : {:?}", current_combination);

    for i in index..(boxes.len() as u32){
        current_combination.push(boxes[i as usize]);
        find_combinations(boxes, results, current_combination, i+1, wanted_sum);
        current_combination.pop();
    }
}