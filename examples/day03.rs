use std::io::Read;

fn main() {
    let mut code = String::new();
    std::io::stdin().lock().read_to_string(&mut code).unwrap();

    let mut sizex = 0;
    for line in code.lines() {
        println!("{}", line);
        sizex = line.len();
    }

    let sizey = code.lines().count();

    let trees: std::collections::HashSet<_> = code
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| if c == '#' { Some((x, y)) } else { None })
        })
        .collect();

    // println!("trees: {} {} {:?}", sizex, sizey, trees);

    let slopes = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    let mut res = 1i64;
    for (incy, incx) in slopes.iter() {
        let mut posx = 0;
        let mut posy = 0;
        let mut hit_trees = 0;
        while posy < sizey {
            if trees.contains(&(posx % sizex, posy)) {
                hit_trees += 1;
            }
            posx += incx;
            posy += incy;
        }
        res *= hit_trees;
        println!("hit trees: {} {} {}", incy, incx, hit_trees);
    }
    println!("all: {}", res);
}
