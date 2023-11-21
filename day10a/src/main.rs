fn santa(path: &str) -> usize {
    let mut st: String=path.to_string();
    let mut output=String::new();
    let mut final_output=String::new();

    for i in 0..50 {
        let mut input = st.chars().into_iter().peekable();
        while let Some(c)=input.next() {
            let mut count=1;
            while input.peek().map_or(false, |&x| x==c) {
                count+=1;       
                input.next();
            }
            output.push_str(format!("{}{}",count,c).as_str());
        }
        st=output.clone();
        final_output.push_str(&output);
        println!("Output {i} : {output} {}", output.len());

        output.clear();
    }

    println!("end : {}", final_output.len());
    final_output.len()
}

fn main() {
}
#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(paq: &str, expected: usize) {
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
        dotest("1", 2);
 
}

    #[test]
    fn prod_tests(){  
        dotest(&"3113322113", 0);
    }
}