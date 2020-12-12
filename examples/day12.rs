use aoc2020::math::*;

fn right(dir: char) -> char {
    match dir {
        'W' => 'N',
        'N' => 'E',
        'E' => 'S',
        'S' => 'W',
        _ => panic!("bad rotation"),
    }
}
fn left(dir: char) -> char {
    match dir {
        'E' => 'N',
        'S' => 'E',
        'W' => 'S',
        'N' => 'W',
        _ => panic!("bad rotation"),
    }
}

fn main() {
    let input = aoc2020::map_input_vec(|l| {
        (
            l.chars().next().unwrap(),
            l.chars()
                .skip(1)
                .collect::<String>()
                .parse::<i32>()
                .unwrap(),
        )
    });
    {
        // part 1
        let mut dir = 'E';
        let mut ship = Vec2(0, 0);
        println!("{:?}", input);
        for (c, v) in input.iter().cloned() {
            match c {
                'F' => ship += Vec2::from(dir) * v,
                'R' => dir = (0..v).step_by(90).fold(dir, |a, _| right(a)),
                'L' => dir = (0..v).step_by(90).fold(dir, |a, _| left(a)),
                'N' | 'S' | 'E' | 'W' => ship += Vec2::from(c) * v,
                _ => panic!("bad command"),
            }
            // println!("{:?} {}", ship, dir);
        }
        println!("{:?}", ship);
        println!("dist: {}", ship.manhattan());
    }
    {
        // part 2

        let mut ship = Vec2(0, 0);
        let mut waypoint = Vec2(10, 1);
        // println!("{:?}", input);
        for (c, v) in input.iter().cloned() {
            match c {
                'F' => ship += waypoint * v,
                'R' => waypoint = (0..v).step_by(90).fold(waypoint, |a, _| a.rotate_right90()),
                'L' => waypoint = (0..v).step_by(90).fold(waypoint, |a, _| a.rotate_left90()),
                'N' | 'S' | 'E' | 'W' => waypoint += Vec2::from(c) * v,
                _ => panic!("bad command"),
            }
            // println!("{:?} {:?}", ship, waypoint);
        }
        println!("{:?}", ship);
        println!("dist: {}", ship.manhattan());
    }
    {
        // part 2 (compulsively functional edition)
        let (ship, _) = input.iter().cloned().fold(
            (Vec2(0, 0), Vec2(10, 1)),
            |(ship, waypoint), (c, v)| match c {
                'F' => (ship + waypoint * v, waypoint),
                'R' => (
                    ship,
                    (0..v).step_by(90).fold(waypoint, |a, _| a.rotate_right90()),
                ),
                'L' => (
                    ship,
                    (0..v).step_by(90).fold(waypoint, |a, _| a.rotate_left90()),
                ),
                'N' | 'S' | 'E' | 'W' => (ship, waypoint + Vec2::from(c) * v),
                _ => panic!("bad command"),
            },
        );

        println!("{:?}", ship);
        println!("dist: {}", ship.manhattan());
    }
}
