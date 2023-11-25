use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone)]
struct Reindeer {
    name: String,
    speed: u32,
    running_time: u32,
    resting_time: u32,
    points:u32,
}

impl Reindeer {
    fn get_distance (&self, seconds: u32) -> u32 {
        let total_time = self.running_time+self.resting_time;
        let cycle=seconds/total_time;
        let last_cycle = seconds%total_time;
        let running_per_cycle=self.running_time*self.speed;
        let last_cycle_distance = self.speed * if last_cycle >= self.running_time {self.running_time}else{last_cycle};

        cycle*running_per_cycle+last_cycle_distance
    }
}
impl Reindeer{
    fn add_one_point(&mut self)
    {
        self.points+=1;
    }
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

    let path = Path::new("input14.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut reindeers: Vec<Reindeer>=Vec::new();

    for line in reader.lines() {
        match line {
            Ok(l) => {
                let parts: Vec<&str> = l.split_whitespace().collect();
                let name = parts[0].to_string();
                // Vixen can fly 8 km/s for 8 seconds, but then must rest for 53 seconds.
                // 0     1   2   3 4    5   6 7        8   9    10   11   12  13 14
                let speed: u32 = parts[3].parse::<u32>().unwrap();
                let running_time: u32 = parts[6].parse::<u32>().unwrap();
                let resting_time: u32 = parts[13].parse::<u32>().unwrap();
                let reindeer = Reindeer{name, speed, running_time, resting_time,points:0};
                reindeers.push(reindeer);
            },      
            Err(e) => {
                println!("Error on line {}", e);
            },
        }
    }
    println!("Reindeers : {:?}",reindeers);

    for i in 1..=2503 {
        if let Some(&mut ref mut fastest_reindeer) = reindeers.iter_mut().max_by_key(|r|r.get_distance(i)){
            let current_distance=fastest_reindeer.get_distance(i);

            for reindeer in &mut reindeers {
                if reindeer.get_distance(i) == current_distance {
                    reindeer.add_one_point();
                }
            }
        }
    }
    for r in reindeers {
        println!("Reindeer {} has {} points", r.name, r.points)
    }

    Ok(())
}
