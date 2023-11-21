fn santa_path__with_robot(paq: &str) -> u32 {
    println!("following {paq}");

    let mut counter = 0;
    let mut v = Vec::new();
    let mut ts = (0, 0);
    let mut tr = (0, 0);
    v.push(ts);
    for c in paq.chars() {
        match c {
            '>' => {
                if counter % 2 == 0 {
                    ts.0 += 1;
                } else {
                    tr.0 += 1
                }
            }

            '<' => {
                if counter % 2 == 0 {
                    ts.0 -= 1
                } else {
                    tr.0 -= 1
                }
            }
            '^' => {
                if counter % 2 == 0 {
                    ts.1 -= 1
                } else {
                    tr.1 -= 1
                }
            }
            'v' => {
                if counter % 2 == 0 {
                    ts.1 += 1
                } else {
                    tr.1 += 1
                }
            }
            _ => panic!("Unexpected character"),
        }

        if counter % 2 == 0 {
            if !v.contains(&ts) {
                println!("Pushing Santa {:?}", ts);
                v.push(ts);
            }
        } else {
            if !v.contains(&tr) {
                println!("Pushing Robot {:?}", tr);
                v.push(tr);
            }
        }
        counter += 1;
    }
    v.len() as u32
}
fn santa_path(paq: &str) -> u32 {
    println!("following {paq}");

    let mut v = Vec::new();
    let mut t = (0, 0);
    v.push(t);
    for c in paq.chars() {
        match c {
            '>' => t.0 += 1,
            '<' => t.0 -= 1,
            '^' => t.1 -= 1,
            'v' => t.1 += 1,
            _ => panic!("Unexpected character"),
        }

        if !v.contains(&t) {
            println!("Pushing {:?}", t);
            v.push(t);
        }
    }
    v.len() as u32
}
#[warn(dead_code)]
fn santa_ribbon(paq: &str) -> u32 {
    let tot: Vec<&str> = paq.split('\n').collect();
    let res = tot
        .iter()
        .map(|x| {
            println!("\n{:?}\n", x);
            let s: Vec<_> = x.trim().split('x').collect();
            let t: Vec<u32> = s.iter().filter_map(|x| x.parse::<u32>().ok()).collect();
            println!("\n{:?}\n", t);

            let a = t[0];
            let b = t[1];
            let c = t[2];
            let mut v = [a, b, c];
            v.sort();
            let rib = 2 * (v[0] + v[1]) + a * b * c;
            println!("a:{a},b:{b},c:{c} : {}", rib);

            rib
        })
        .sum();
    println!("\n\t\t{res}\n");
    res
}
fn santa_paper(paq: &str) -> u32 {
    let tot: Vec<&str> = paq.split('\n').collect();
    let res = tot
        .iter()
        .map(|x| {
            println!("\n{:?}\n", x);
            let s: Vec<_> = x.trim().split('x').collect();
            let t: Vec<u32> = s.iter().filter_map(|x| x.parse::<u32>().ok()).collect();
            println!("\n{:?}\n", t);

            let a = t[1] * t[0];
            let b = t[0] * t[2];
            let c = t[2] * t[1];
            let res = 2 * (a + b + c) + a.min(b).min(c);
            println!("a:{a},b:{b},c:{c} : {}", res);

            res
        })
        .sum();
    println!("\n\t\t{res}\n");
    res
}
