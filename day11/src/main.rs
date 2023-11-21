fn find_next_password(mut pass: String) -> String{
    loop {
        pass= next_password(pass);
        if is_valid_pass(&pass) {
            return pass;        
        }
    }
}

fn next_password(pass: String) -> String {
    //
    let mut v:Vec<char> = pass.chars().rev().collect();

    let index=0;

    for mut i in 0..8 {
        let a = v[i] as u8 +1 -'a' as u8;
        if a > 25 { 
            v[i]='a';
            i+=1;
        } else {
            v[i]=(('a' as u8) + a ) as char;
            return v.into_iter().rev().collect();
        }
    }
    return "".to_string();
}

fn is_valid_pass (pass: &str) -> bool {

    let v_pass:Vec<char>=pass.chars().collect();

    if pass.contains('i')||pass.contains('o') || pass.contains('l') {return false};
    
    let mut suite=false;
    for i in 0..(pass.len()-2) {
        if v_pass[i] as u8 +1 == v_pass[i+1] as u8 && v_pass[i] as u8 +2 == v_pass[i+2] as u8 {
            suite=true;
            break;
        }
    }
    let mut dups=0;
    let mut i =0;

    while i < v_pass.len()-1 {
        if v_pass[i]==v_pass[i+1] {
            dups+=1;
            i+=2;
        }
        else {
            i+=1;
        }
    }
     dups>=2 && suite
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotestnext(paq: String, expected: String) {
        println!();
        println!("-----------------------");
        let actual = find_next_password(paq);
        assert!(
            actual == expected,
            "Test failed with \nExpected {expected:?} but got {actual:?}"
        )
    }

    fn dotestcheck(paq: String, expected: bool) {
        println!();
        println!("-----------------------");
        let actual = is_valid_pass(&paq);
        assert!(
            actual == expected,
            "Test failed with \nExpected {expected:?} but got {actual:?}"
        )
    }

    #[test]
    fn fixed_tests() {
        dotestnext("hxbxxyzz".to_string(), "abcdffaa".to_string());
        /*dotestnext("abcdefgh".to_string(), "abcdffaa".to_string());
        dotestnext("ghijklmn".to_string(), "ghjaabcc".to_string());*/
 
}

    #[test]
    fn prod_tests(){  
        dotestcheck("abcdeggg".to_string(), false);
        dotestcheck("hijklmmn".to_string(), false);
        dotestcheck("abbceffg".to_string(), false);
        dotestcheck("abbcegjk".to_string(), false);
        dotestcheck("abbceffg".to_string(), false);
        dotestcheck("abcceffg".to_string(), true);

    }
}