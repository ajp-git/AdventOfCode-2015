use std::io::{self, BufRead, BufReader};
use std::fs::File;

/*
fn santa_global(text: &str) ->  u32 {
    for line in text.split("\n") {
        santa(line.trim());
    }
}
 */
fn santa(text: &str) -> u32 {
    println!("Analysing line : {:?}", text);
    let mut count:u32=2; // 
    text.chars().for_each(|c|{
                count+=1;
                println!("c : {c}, {count} ");
    });
    let mut chars = text.chars().peekable();
    
    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                if let Some(next) = chars.next()  {
                    match next {
                        'x' => { // \xaa = 4 -> 1
                            count-=1;
                            chars.next();
                            chars.next();
                        },
                        _ => {
                            count-=1;
                        },
                    }
                }
            },
            _ => {
                count-=1;
            },
        };
    }
    count
}

fn main() -> io::Result<()> {
    let file = File::open("/home/ajp/kDrive/Rust/AOC/day8-1.txt")?;
    let reader = BufReader::new(file);

    let mut lines = Vec::new();
    let mut tot:u32=0;
    for line in reader.lines() {
        let line = line?;
        lines.push(line);
    }

    // Now `lines` contains all the lines from the file with double quotes
    for line in &lines {
        println!("{}", line);
        tot+=santa(line);
    }

    println!("Total : {tot}");
    Ok(())
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
    /*
        123 -> x means that the signal 123 is provided to wire x.
        x AND y -> z means that the bitwise AND of wire x and wire y is provided to wire z.
        p LSHIFT 2 -> q means that the value from wire p is left-shifted by 2 and then provided to wire q.
        NOT e -> f means that the bitwise complement of the value from wire e is provided to wire f.
    */
    #[test]
    fn fixed_tests() {
        dotest(r#""#, 2); 
        dotest(r#"aaa\"aaa"#, 3); 
        dotest(r#"ab"#, 2); 
        dotest(r#"\""#, 3); 
        dotest(r#"\x30"#, 5); 
        dotest(r#"\x27"#, 5);
        // dotest("123 -> a", 123); // First test
        //dotest("123 -> a\n124 -> a\n130 -> b\na OR b -> a\na AND b -> c\na OR c -> b", 0); // more complex
        //dotest("1 -> a\n2 -> b\na OR b -> c\nb OR c -> d", 1); // more complex
    }

    #[test]
    fn prod_tests(){
        
        dotest("", 2); 
    }
}
/* */
