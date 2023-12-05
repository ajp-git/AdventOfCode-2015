fn main(){
    static PUZZLE_INPUT:u32=33_100_000;

    let mut MAX_ELVES:u32=786_241;
    let mut houses:Vec<u32>=vec![0;MAX_ELVES as usize];
    let counter:u32=0;

    let mut done=false;
    let mut max_gifts :u32=0;
    for i in 1..MAX_ELVES{
        let mut house_count=0;
        for e in (i..MAX_ELVES).step_by(i as usize)
        {
            if house_count>50 {
                break;
            }
            house_count+=1;
            houses[e as usize]+=11*i;
        }
    }
    houses.sort();
    houses.iter().enumerate().for_each(|(f,n)| if n>=&PUZZLE_INPUT{println!("House {} has {} gifts", f,n )});
        
}
fn part1() {

    static PUZZLE_INPUT:u32=33_100_000;

    let mut MAX_ELVES:u32=779_000;
    let mut houses:Vec<u32>=vec![0;MAX_ELVES as usize];
    let counter:u32=0;

    let mut done=false;
    let mut max_gifts :u32=0;
    for i in 1..MAX_ELVES{
        for e in (i..MAX_ELVES).step_by(i as usize)
        {
            houses[e as usize]+=10*i;
            if max_gifts<houses[e as usize]{
                max_gifts=houses[e as usize];
            }
        }
    }
    houses.iter().enumerate().for_each(|(f,n)| if n>=&PUZZLE_INPUT{println!("House {} has {} gifts", f,n )});
    
}
