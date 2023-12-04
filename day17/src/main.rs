use itertools::Itertools;

fn main() {
    const LITERS: u32 = 150;
    let recipients = vec![50, 44,
    11,
    49,
    42,
    46,
    18,
    32,
    26,
    40,
    21,
    7,
    18,
    43,
    10,
    47,
    36,
    24,
    22,
    40];
    let mut match_count=0;

    for size in 1..=recipients.len() {
        let combs = recipients.iter().combinations(size);
        for comb in combs {

            // Calculate the total liters for the current combination
            let total_liters: u32 = comb.iter().map(|&&x| x).sum();
            if total_liters == LITERS {
                println!("{:?} matches", comb);
                match_count+=1;
            }
        }
        if match_count !=0 {
            break;
        }
    }
    println!("Matches {}", match_count);
}
