use std::cmp::{min,max};
use std::{path::Path, fs::File, io};
use std::io::BufRead;

struct LightRoom {
    lights:Vec<Vec<bool>>,
}

impl LightRoom
{
    fn step(&mut self) {

        let mut new_lights:Vec<Vec<bool>>=vec![vec![false;self.lights.len()];self.lights.len()];

        let xsize=self.lights.len();
        let ysize=xsize;
        for (x, light_row) in self.lights.iter().enumerate() {
            for (y, light) in light_row.iter().enumerate() {
                let mut status_count=0;

                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue; // Skip the current light itself
                        }
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;

                        // Skip out-of-bounds indices
                        if nx >= 0 && nx < xsize as i32 && ny >= 0 && ny < ysize as i32 {
                            if self.lights[nx as usize][ny as usize] {
                                status_count += 1;
                            }
                        }
                    }
                }
                if *light == true {
                        new_lights[x][y]=status_count==2 || status_count==3;

                } else {
                    new_lights[x][y]=status_count==3;

                }
            }
        }
        self.lights=new_lights;
    }

    fn count_lit(&self) -> u32{
        let mut total: u32=0;
        for x in 0..self.lights.len() {
            for y in 0..self.lights.len() {
                if self.lights[x][y]==true{
                    total+=1;
                }
            }
        }
        total
    }
// For part2
    fn lit_corners(&mut self)
    {
        let l = self.lights.len()-1;
        self.lights[0][0]=true;
        self.lights[l][0]=true;
        self.lights[0][l]=true;
        self.lights[l][l]=true;
        
    }
// End for part 2
}

fn main() -> io::Result<()> {
    let path = Path::new("input18.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let grid:Vec<Vec<bool>>=reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| 
            line.chars().map(|c| c=='#').collect()
        )
        .collect();
    
    println!("Grid : {grid:?}");
    print_grid(&grid);

    let mut lr=LightRoom{lights:grid};
    
    lr.lit_corners(); // For part2

    for i in 0..100 {
        lr.step();
        lr.lit_corners(); // For part2

        print!("\rStep {i}");
    }
    print_grid(&lr.lights);

    let total=lr.count_lit();
    println!("Lit : {}", total);

    Ok(())
}

fn print_grid(v:&Vec<Vec<bool>>) {
    for x in 0..v.len() {
        for y in 0..v.len() {
            print!("{}",if v[x][y] { '#'} else { '.' });
        }
        println!();            
    }
}
