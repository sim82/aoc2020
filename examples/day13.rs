fn main() {
    {
        // part 1

        let a = 1005526;
        let b = [37, 41, 587, 13, 19, 23, 29, 733, 17];

        let res = b
            .iter()
            .cloned()
            .map(|i| {
                let c = i - (a % i);
                (c, c * i)
            })
            .min_by_key(|(c, _)| *c)
            .unwrap();

        println!("res 1: {}", res.1);
    }

    let input = "7,13,x,x,59,x,31,19";
    let input = "67,7,59,61";
    let input = "17,x,13,19";
    let input = "67,x,7,59,61";
    let input = "37,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,41,x,x,x,x,x,x,x,x,x,587,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,13,19,x,x,x,23,x,x,x,x,x,29,x,733,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,17";

    let input = input
        .split(",")
        .enumerate()
        .filter_map(|(i, v)| match v.parse::<i64>() {
            Ok(v) => {
                let i = i as i64;
                if i == 0 {
                    Some((v, i))
                } else {
                    Some((v, v - (i % v)))
                }
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    // let input = [(3, 0), (4, 3), (5, 4)];
    if true {
        println!("input: {:?}", input);
        println!("res: {:?}", chinese_remainder(&input[..]));
    }
}

fn chinese_remainder_naive(input: &[(i64, i64)]) {
    // naive solution
    let start = input.iter().max_by_key(|(v, _)| *v).unwrap().0 + 1;
    println!("input: {:?}", input);
    println!("start: {:?}", start);
    for i in 0.. {
        if input.iter().all(|(v, p)| {
            let m = i % *v;
            if *p == 0 {
                m == 0
            } else {
                m == *p
            }
        }) {
            println!("found: {}", i);
            break;
        }
    }
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    println!("egcd: {} {}", a, b);
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let x0 = x;
    let (g, x, y) = egcd(x, n);
    println!("egcd: {} {} -> {} {} {}", x0, n, g, x, y);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(input: &[(i64, i64)]) -> Option<i64> {
    // 'inspired by (TM)' https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
    let prod = input.iter().map(|(m, _)| *m).product::<i64>();

    if false {
        // compulive functional disorder version
        Some(
            input
                .iter()
                .map(|(m, r)| {
                    let p = prod / *m;
                    Some(r * mod_inv(p, *m)? * p)
                })
                .collect::<Option<Vec<_>>>()? // neat trick: iter over Option can be collected to Option of container
                .iter()
                .sum::<i64>()
                % prod,
        )
    } else {
        let mut sum = 0;
        for (m, r) in input {
            let p = prod / *m;

            println!("mod inv: {} {} -> {}", p, m, mod_inv(p, *m)?);
            sum += r * mod_inv(p, *m)? * p;
        }

        Some(sum % prod)
    }
}
