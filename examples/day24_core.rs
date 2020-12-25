#![feature(array_value_iter)]
#![no_std]

use aoc2020::bitzet::Bitzet;
use aoc2020::math::Vec2;

fn adjacent(v: Vec2) -> [Vec2; 6] {
    let xshift = v.y().abs() % 2;
    let mut d = [
        Vec2(1, 0),
        Vec2(-1, 0),
        Vec2(-1 + xshift, 1),
        Vec2(0 + xshift, 1),
        Vec2(-1 + xshift, -1),
        Vec2(0 + xshift, -1),
    ];

    d.iter_mut().for_each(|f| *f = *f + v);
    d
}
#[test]
fn test_hex() {
    println!(
        "{:?}",
        std::array::IntoIter::new(adjacent(Vec2(0, 0))).collect::<Vec<_>>()
    );
    println!("{:?}", adjacent(Vec2(0, 1)));
    println!("{:?}", adjacent(Vec2(0, -1)));
}
fn main() {
    let mut black = Bitzet::new();
    for line in input().iter() {
        let mut c = line.chars();
        let mut x = 0i32;
        let mut y = 0i32;

        loop {
            match c.next() {
                Some('e') => x += 1,
                Some('w') => x -= 1,
                Some('s') => match c.next() {
                    Some('e') => {
                        x += (y.abs() % 2);
                        y += 1
                    }
                    Some('w') => {
                        y += 1;
                        x -= (y.abs() % 2);
                    }
                    _ => panic!("unhandled"),
                },
                Some('n') => match c.next() {
                    Some('e') => {
                        x += (y.abs() % 2);
                        y -= 1
                    }
                    Some('w') => {
                        y -= 1;
                        x -= y.abs() % 2;
                    }
                    _ => panic!("unhandled"),
                },
                None => break,

                _ => panic!("unhandled"),
            }
        }
        if black.contains(&Vec2(x, y)) {
            black.remove(&Vec2(x, y));
        } else {
            black.insert(Vec2(x, y));
        }
        // println!("{} {}", x, y);
    }
    // println!("len: {}", black.len());

    for i in 1..=200 {
        let mut black_new = Bitzet::new();

        for v in black.iter() {
            let n = adjacent(v).iter().filter(|v| black.contains(*v)).count();
            if (1..=2).contains(&n) {
                black_new.insert(v);
            }
        }

        let white = black
            .iter()
            .flat_map(|v| core::array::IntoIter::new(adjacent(v)))
            .collect::<Bitzet>();

        let white = white.difference(&black);
        for v in white.iter() {
            let n = adjacent(v).iter().filter(|v| black.contains(*v)).count();
            if n == 2 {
                black_new.insert(v);
            }
        }
        black = black_new;
        // println!("day: {} {}", i, black.len());
    }
}

fn input() -> &'static [&'static str] {
    &[
        "swswswswswnwswswswsweswsesw",
        "wneswseseneswnweneswwswseswwnwseswswe",
        "ewswswswwswnwnwsweswwwwwswwwswwsw",
        "senwseseswseseswswesewseseseseseswsese",
        "seswnwseseswseseswswseseswswseswesese",
        "swnwnenwnenwnwnwswnwnwnwnwnwnwnwenwnwnw",
        "seseeseneswnwseseneeeseeswsewseseese",
        "senenwnwnewnwnwnwnenwnenwnwnwnenwswnenw",
        "eneneneweeneneeneneneeneneenesew",
        "nenesenewwenwseseseseswsesewneseweswse",
        "swswweswswswswswswswseswnenwswswsweswsw",
        "nwswnwnwnwnwnwnenenenwnwnw",
        "seseeneseeesenesewseswsewseeeese",
        "wwseswwnwneswsewswwswswswwswswswswne",
        "wwewwnwwwswsenwnewnwwnwsewwww",
        "wnwnwnwenwnwnwnenwnweswnwnwwswnwnwnw",
        "neeneneswnenenenenwnenenwnenenenenewnwe",
        "nenwsenwnwnwnwnwswswnwnwnwenwnwnwnwnwnw",
        "swenwsenweenwswnwswseseeswswesenwne",
        "ewseswsewswneewneneneeneseneeswnwsw",
        "nwswnwswewswswswesesweswswse",
        "swswswswenwswswswwswneswneswswswsesww",
        "seswswseseswswsenwswse",
        "wwwwwwwswsewnewwnwwwwwew",
        "swwwswewswswswswswnwswswswswseswsww",
        "sesewsewneseneseseseswswseeseseswnesw",
        "wwswswwwwswwewwswwwsewnwnww",
        "nwnenwnenenwnenwnwnwsenwnwne",
        "neewwwneswseseeeneeseesesesewee",
        "senwneeneneenewneneneneneneneswnenenene",
        "swswswneswwswswwswswsewneswesw",
        "nenenenenwnenesewnenenenenenenwnwnenwne",
        "nwnwnwsesenwnwwsewnwnenwnewnwnwwwnwnw",
        "nwnenwwnweswseseeswenenwweneneeswswsw",
        "eeesweeeenweseseweeneeseeese",
        "swswseseswseswnwseseswswswseseseneswswsese",
        "swnwnwewnewwwwnwnwwnwwswnwse",
        "sesenwnenenwnenwnwnwnenwnewnwnwnw",
        "neswneneeneneenenenenewnwneneswnenwewne",
        "nwnenwswnwenwnwnwnwnwnwnwswnwnwnwnwnwnwnw",
        "nwwwewwneewswneswnwwwwwsewsew",
        "wswswswswswweswwwwswseswswswneswswe",
        "wnwwnwnwnewnwnwnwwnwwsenwwnwnwwnw",
        "wswwwswswwwwnesew",
        "nwnwneenenwnwswnwnwnwswnwsenwnwwenenenwnw",
        "swsewseseswswseswneseseswseseswseseseese",
        "swseswseswnwsewneweseseseeswsesesesew",
        "sweswwwneswswwswesenenwsweswswswswse",
        "wswseneeseweeeeweseesenwnwseew",
        "wwnwnwwwnwnwnwnwwnwnwnwnwswwnwwe",
        "swnwnwwseneneneneneneswswswnewenwsese",
        "seseseseseesesenwseseneesewswseswnwnw",
        "neswswsenwesenesenesenesesewwneswnwswnwne",
        "nesesesenenwnwnwnenwnwswnwneenewnenwne",
        "neneneneneeneenewseewnenesenenenenene",
        "eseseseseneseeseseesenwseseseseseseesww",
        "neeenenesweeeeeeweneeeeeene",
        "enwwwwwsewwwnwnwwnwwwwwnwnwnw",
        "neseeeeseseseeseeseneeseseseenwwsw",
        "enenesewneenenenwseseneeweeswenwene",
        "wneneseneneneenenenenenwnenenenenenene",
        "wnwneeswewnwnwseswswsweswsenewswsw",
        "eseeswsenwenweseseeseseseesenesesese",
        "wswswseseseseneenesesesewsesenewsew",
        "wnwwwwwwewwswwwwwwwswsww",
        "swenewwwseswwnesesenewneswseneswswswsw",
        "eswseseseseesewseseneseenwsewseee",
        "swneneswneneneneeneeenwneeenenwwenene",
        "neswnenesenenwnenenenenenenenenenenenwnene",
        "sesesesesewsesesweseseseseseseesesenee",
        "nwnwnwnwnwnwswnwnesewnwnwnwwnwnwnwnenw",
        "eenesewwwwnwnwwwwnwwwsw",
        "swnenwwswswswneseeseneswneseswwseswsw",
        "neneenenwnenenwnwnwseswnwnwnenwnenenenenw",
        "neenenwneneseneeneneeneneeeeneswnee",
        "swswswswneseswwswswseswswweneenwnew",
        "neseewsesesewswsese",
        "enenweneeneneeeeneeswswneneeeneenw",
        "seeeeneeeenwseewswenewnwseesew",
        "sweeeneesenweswewsweseeseenenwee",
        "seesesewswenenwnweswsewseneenwsesw",
        "swwswwswwwneswwsww",
        "wwwwwwwwwwwwewwwweww",
        "neneneseneeneneeswneenesenwnewnenwe",
        "nwnwnwnwnwnenwnwnwnwsenwnwnwwnwnwnwnwnw",
        "eeeeeseseeenweeeesweeenee",
        "nwnenwnwneneseneswnenwnenwwnwnwnenenwnwnw",
        "sewswswwsewwneesewnewnewne",
        "nwnwwnweswnwneswswnwnwnwnenwnweswenwe",
        "nwneenenenenwneneneeneseeneneswnenenene",
        "swwwwswswswswnewwwwwsw",
        "neneneneeneeneeneneeewnwneswneneesene",
        "nenenewnenenenenenenewnenwnwneesenene",
        "swnwswswwwswswwweswnewswswwswnwswsee",
        "neseseeseseeseseseeeseseewesewse",
        "nwnenenesesenwnwswnwwswwnwnwnenwnwsew",
        "seesesewseneseseseseseseswsesesenwswsese",
        "nwsenwwnenwneseswwwnwwnewesewwswnw",
        "sesesesenesesesesesewsewsesesesesesesesee",
        "seswwnwseeswnwwneeneseweeseenee",
        "nwseenwnwnwnewswnw",
        "nwnwnenwsenwnwnwnwnww",
        "wnwwwnwenwnwwsenewnwnwnwwsewwnenw",
        "sesenwsenwseswseseseseeewsenesesesesese",
        "seeneeeeseeenwnwsweeeeneeene",
        "eeeeseeeeswneeneeneesweenew",
        "swseneswswnwseswswseswswswswneswswswsesw",
        "nesesweneseseneeweesenwnwseeswnwswse",
        "senwswnenenwnwneeesw",
        "nweeseeenweesweeeeeseeswese",
        "sweneenwneeneeneesenwnwnwnesweswswnee",
        "swswnwewswnwswwwwswwswwwewswswswe",
        "swswswsenwswwswsweswswnwsweswswesesw",
        "swswsesesenenwewseseswseeswseseswsesesw",
        "seswnewewswnenwenwnenenenenenenwwene",
        "wnwswswwwsewnwnwswweseswsenewwww",
        "wswswweswwwnwsewwswswwswsw",
        "swswseswsweswswswswswswswswswswnwswswsw",
        "nenwnewnenenenwnenenwnwnenwneneneenese",
        "seswswswswseneswsesw",
        "neseneneenewsenwwnwnwwswwnwwee",
        "swswswwswswswwnwnewwwneeneswwnwse",
        "nwswwwwnwswesewsenwseswwswwwwwnew",
        "wsenweeeseeeseeseeeeeeeseese",
        "seseseseseseeswseesesesenesesewnesese",
        "nenesesewsenwneseseseneseswseswwswsesese",
        "sewseseewsenenweewswseseneeswsee",
        "swseewnwsenwnenwswneswnwwswnesenwnewnw",
        "neeeewseseneswnenwneneweneneseseew",
        "nenwnenwswneneenenwnesesenewneneswnenwnw",
        "nenewnenesweneneenesenenenenenenenwneee",
        "wnwsenwnwsenenenesesesenesewnwnwnwnwnenw",
        "nwswswswnewnwsweseswnesenweswenwesww",
        "nwnwnenesenwnwwnwneswnenwnwnwnwnwenwnw",
        "seswseseswswneswnwswseswsesesenwswswswswsw",
        "nenwnwsenwnwnwnenwnenenenwsenwwnwnwnenwnwne",
        "eswwswswwswswswswswswwsww",
        "nenesenwenwsweesweeneenw",
        "nenenesenenenenwnwnenenewnenwswnwnenenwne",
        "eeseeeneseewneeeeseeweeeese",
        "swswwseseswsenwseeswswswsesesw",
        "nwnwsenwsenenenwswseswsewseswswwnwswnew",
        "nwwnwnwnwenwnwwwnwwnwwwnwnwwwsw",
        "wewnwwswswnwwnwnwnwnwewswnwesenwwne",
        "nenenwnwnwnwenewenwsenwnenenwnwnewnwne",
        "seseseenwseseseseseswwseswnwsee",
        "swseswseseseswsesesesenwseseswsese",
        "swswneseswseswwswswswseswsweswswswswswsw",
        "seneneeseseswseseswsenwseeseseswsesee",
        "nwneneseneneneneneenesenewseneewnene",
        "nwwnenwnenenwnwsenwnwsenwnwnwnwnenenenw",
        "wwnwwsewwwwwsewwwnwnwnwnwwnese",
        "seseswnwseswseswsesesenwsesesese",
        "wnweswsenwnwswnenwnwnwswwsewnenwnwwnw",
        "wseswnwseswswseswswsenwswswsesene",
        "neeeneenwneneeeeneeesweenenweesw",
        "wwwnwnwwwwwwwwwnwwwwesew",
        "neseeneneneneneneneneneneenwnenwneneswnene",
        "neeeesenweeswenewne",
        "nwswnwenwwwwnwswwwwwwnwnwenwnwe",
        "wwwwswwwwwwswwwwwewsw",
        "eeseeswswswneeeweewneneneenenene",
        "swswswesesesenwswseswnese",
        "sesewswseseseswseseseseseseseseseswnese",
        "neeeeeneeweeeesweeenenwe",
        "nwwnesenwneneswenenenenenwnwnenwnwwnene",
        "neeenwneneswneneneswneeeneswneene",
        "nwwewwnwnwnwswwswwwnwwwwwwnew",
        "nenwnwnwwswnwwnwnenwesee",
        "wswswswswwseswswswswnewswswnwswwswsww",
        "neneneneswsenenenwne",
        "neeeweeeeeeeseneneneneeesewe",
        "nwswnwnwnwnwenwneeswnwnwwnwnw",
        "swsesenwseswseseswwseneseneseseesesesese",
        "nweeweeeeeseeseeswseeee",
        "nwsesesesesewesenwnenwswneeseneswsesw",
        "nwneneenenewsweeneneneneneneeewnene",
        "swseeseneswwneswnwwswneewnesw",
        "neseseswswseswseswseswsenwsweseswsesesese",
        "seswswwswwwwewswswwnewwwswswnww",
        "senwseesesesesesesesesesesesenwsenwsese",
        "seswnwsesesewseseneseseseesenwsesesesewse",
        "neeenwnesweenwneeeeeeeeeneswene",
        "seenwsenweswnwswwnwnwnwnwnewnwswenw",
        "swswswswswswswswswswwswswseswne",
        "enwswswnwswneswsewnwswseneseenwwnwsw",
        "swneswnenenwnwneeeneneneeswneswnenesew",
        "nwswnwwswwseewseewsenenwnwwwnwwwse",
        "eeeseesenesesewseeseswseneesesesese",
        "swnewswseswwwnwwewew",
        "eeeeeeseeeeeswnwe",
        "nenenenenwnwneeneswwneeneneneswnenenesw",
        "eswwswswswsweswnw",
        "newnwsenwnwnenwnwenwnenenwnwneswnwwswnene",
        "eeneeneneeeneeweeswneeneseeewne",
        "nwwnenewenenwnwnwnwe",
        "neneswwneswneneneswene",
        "swwneneweneeswwneenesenwnwenenese",
        "seesewsesesesesenesesesesewsesesesesese",
        "nesenwesesesenwwswsesewesesesesesesesese",
        "nwsenwneeweenweswnwnwnwwwenwwne",
        "seswswswnwswseswswswseswenwswswswswnesw",
        "swnwsesweswswseswwswswwseeswswsenesesw",
        "ewseseneeseeeneeenwnweeesenwne",
        "neswswswswswswswswseseswwenwswnwswswsw",
        "nwswswwseeswswswswswswswswswswswswswne",
        "nenwnenenenenwnenenenewnenenwneenwnw",
        "senweeeenwseeseeenwseeseseeeese",
        "seseseswseseswswseseseneneseseswswwseswswse",
        "swswwswwswwswswswswwwwswwswwe",
        "sesesewseswseseseseseseseesesesese",
        "nwwnwnwnwnwenwwnwnwnwwnwnw",
        "nwnwnwnwnewnenwnenwnenwnesenenwnenenwnw",
        "nwwseneesenwwswwneseenwseswwwnwe",
        "enwnwswsesenwswsesweeseseseseewnwswse",
        "eeeeseseeesweneeeewewwse",
        "ewneneeseeneseswswnwswwseswesesene",
        "eeeneeeneeweeneeeeeeweese",
        "nwnesenwnwnewnwnenwnenwnwneeswnwnwneswe",
        "swseweeeseseneeeeseneseseeneeew",
        "seewnenenwenesesewnwnwnwseswe",
        "nesenewsesweseseswwsesewneswswswese",
        "neenenesenwneeneeneswswneneenenenene",
        "wneswsenenwsewseeswseswswnesewswnese",
        "senwnwnenenewwwnenwenwnenwseenenenw",
        "nwwwsewneswswwnenwswnewnenwseesenenwnw",
        "eneneneeneseswweeneeewnweneee",
        "seenenwswswswnwnwwseneneseeeeeswsw",
        "nenenesenewnenenwneneneneneneenenenenene",
        "wswwwwwwswwswwnwwswwswewwew",
        "neenweneneseeneswwswnew",
        "eeeenweeseneeneeeeeneeswee",
        "nwsesewsesesewnwseswneseseesesesenwse",
        "wnwnenenenenwwesweeneneswsenwnwswsene",
        "swswwswswswswswswwsweneswnwwnwswesw",
        "wwewwwwnwww",
        "eeeeneswsweneenwnenwneneneeneene",
        "sewseswswswswswseseswseewnwnenwsw",
        "neeneseweeneeneesweeneenweese",
        "sweeeeeeneeeeeeee",
        "wnwnwwnenewwnwnwwsewswwwwnwnwwnww",
        "seswwseseseswnwswseeswswswnwseswseswswse",
        "eseswnenwweeswnwwneeswnese",
        "sesesewnewwwwneneww",
        "nweneesweneesenewnenw",
        "eesesesenwseseeseseseseseese",
        "neswswnewenenenenenenwenenenwne",
        "swnenwnwnwsenwwnwwwneewwnwwwnenwwse",
        "seswswsesewweneenenwsesewswsw",
        "seeseeeswenwswnweeeswnweeeeee",
        "sweswneswwwwswne",
        "seseswseswswseneswswseswseswneswswseswsese",
        "wnwwnwnwnwwwwwnwwwwnwenww",
        "wswseswswswswsweseswswnwseswneswswswsw",
        "enwwnwnwwseeesewnww",
        "nwnwsweseswswswwswswseswswweswnwswe",
        "swseswswswwwswswnewsw",
        "wswnewnewnewenwsewwwseseswnwnwwse",
        "eseneseseeesesweesesewseseesesewse",
        "neewseeseeeswswsesesenenw",
        "newneweenenesewseewwnwwsenesesee",
        "wnenwwswwswswnwnweeenwneswseenwnwnw",
        "swnwnwnwnwenwnenwnwnwnwnwnw",
        "neneesewnweseeneeneseewsewnwswne",
        "wwwwwenwwwwwwswwwwwnwsew",
        "seesweneseeseweeseneseeseseeee",
        "nwnwnwsewwnwwnwnwnwnwnwnwnwnwnwneswwse",
        "newnwswnwwnwnwnwnwwwnwnwnwwwnwnwenw",
        "seweeeseeeseeseeeeeseswnweseene",
        "eseseeeeeswesesweeseneeesenesee",
        "eseswwsesenwseseswseseseeswsenese",
        "wnwnwnwnwnenwnwenenw",
        "neneenesweswsenwseneneswnenwwwneswneww",
        "wswneswewswseswswswswwsw",
        "seseeneeseswsewneswnwseneswswneenwnee",
        "nwwnesenwsenwnenwnwnesewnwswswnwneswwnw",
        "enweneeeneeneneeneneswseneseenwne",
        "nwnenwnenenwnwnwsenwnwnewwnwnenwene",
        "wwnwneswwewnwweswwneseswwswnesew",
        "enewneewsewenenenenesenenewene",
        "nwsenwnwnwnenenenenenenenwnwnwnenw",
        "nenenenenenwnwneneneswne",
        "enwneseeneneneneswwneeneneswe",
        "swswwswsewsewwwwneeeswwwswnwww",
        "swnwseenenenwenwnenwnwwnwswneswnwnwnenw",
        "wswswwswwwswswswswswswwswweswnwsw",
        "seseswwneswwswswswswswswneswswswswswsww",
        "seseswnwnwnwnwnwnwnwnwwewnwnw",
        "wwnwnwwwnwnewnwsewwnwnwnwnwnwewsw",
        "wseswnwwneswnenewwseewseswswwwne",
        "wswwwwwwwwwnwewwwnw",
        "senwswseneseseseswswswsweswswseswseswsw",
        "wwwswwwswwwsewnwswwnewwswswswsw",
        "swseswswwswseswswseswseseswseswne",
        "ewswswsweswswnewswseneseswseswsenwnwe",
        "wwwwwsenwwwwwwwwwwne",
        "nwsewwwwsewnwwwnwne",
        "swswswswswnewswwwwwswswswswswsw",
        "nwwseenwsenenwwnwnwnwnwenwwnwnwwsw",
        "eswseswswsenenenenwswseswswswseswsenwswse",
        "wswwswwswwsewwwswswswwnwwnene",
        "sesesesesesenewwswwseneseseeswsesesese",
        "swnwnwnwnwnwnwnwnenwnwnwenenwnwnwwnwnwnw",
        "wneenenwneenwwnenwnwnwneneneneeneswnw",
        "swwwwwwwwwwwwwwwnewseww",
        "senwneswseswsesenwsesee",
        "neswwseseeswseseswswsesww",
        "nwswseswseseswseneeseseneseseseswseswnesw",
        "eneeeneneneewenenenenenweenesene",
        "wnwwnenenwnenwswnwnenweenwnwnenese",
        "wswseswswneswswswnweswswsweseseswsene",
        "senwwseneseeesewnesewswsewsesenwnese",
        "esenesesesesesesesesesesewsesesewsesw",
        "neeneneseswnwnesenwwnwnwswnenwne",
        "neenenwnenewneswnewneneneenenenenenwe",
        "swseswswswswseswswnwswsweswsesesesesw",
        "nwenwsewnwnenenwnwwnwnwnwswnwnwnwnwwnw",
        "wsenwwnwswneweswwweewnewesene",
        "swwnwwswwwnwnwewewewwwnwnwnww",
        "wsewwnwswsweswswswswswswswswswswswswsw",
        "seswwswwwewswewwwwnewwwwne",
        "esesesesesewneseeseseseseeswseneesesese",
        "nwnwnenenwneneeneweneneneseseswnenwnenw",
        "neneneeneswseswwnenenenenenwneneneneenene",
        "nwnwnwnwnwnenwsenwnwnwnenwnwwnwnenw",
        "eeeeeeewnweeeseeeeeeee",
        "wwwnwwnwewnwwnwwnwswnwwnwnewwnw",
        "swenwswnwswesesenwswswswswnwwnweseesw",
        "neeneneeneneswneneneswneneneneenenenene",
        "nwsenwseeeesesese",
        "wnwwnwnwwnweesewnwswnwswnwnwseswneww",
        "nwseseswseseeneseneseseseseseswswesesee",
        "neneeenenwswneeeswneneneenwnenenene",
        "seswseeswwswswswswswsweswnwse",
        "neeeneneeneswenenwnenwneeneweswnee",
        "ewewwswnwneewwnwswsewswwenew",
        "swwswswswsweswswswswswswswsenewswswsesw",
        "nwsewnwswnwnwsenwnewwnenwnwnwwwnwewnw",
        "nwseswwwwwswswswwswneswwswwwwsww",
        "nenwnwnwnenenenwnenenenenenesenenenene",
        "eseseswsweseswseswsenesesewwsesesesese",
        "sesesesesesesesenewnwseseseseseseesese",
        "nenenenesenewnesenewwenesenenwnenene",
        "wewewswwswnwwwwwwwswwww",
        "enwwnwnwnwwwswnewnwnwwnwseewnwsenew",
        "swseseenwnweswseseswswnenenwnesesewsw",
        "wwnwwwwsewwwwwwewwwwwww",
        "nwewnwnwwenwswnwnwenwnwnwnwnwsw",
        "wsesesenwsenwneeseseewseeswseseese",
        "enweneeeswnewneswewneneeneeneee",
        "seswwseswseseseswswesesesesesesesenwsese",
        "swwnwswwnewsewesesenwwewnew",
        "nweneeneseneneneenenee",
        "newnenenenenenwnenesenenenenenene",
        "wwwwwwwwswwnwswwwsewwwwwne",
        "swnwswnwenesenwnenew",
        "nwwwwwwwwwwsewwwwwnewww",
        "swwsweswnwnenwnwnenwnenewewswsenwswnese",
        "enenewneswneneneeeeneeseneneneneenee",
        "nenwnenenenenwnenwwnenenenesesenenwnewne",
        "swnenenenesweswnenwneswswnwswnwnwneswswne",
        "wwwwsesewneswwwnewwnewwnewsew",
        "seswseseseseseeseseswsesesesenenwsesesese",
        "wwwswwnwnwewwnwwnwwww",
        "seseseseswsesenwswsese",
        "swnwswswswswswnwswsweswswswswswswswseswswsw",
        "nwnenwswnenwnwnwnwsenenwnwnwenwnwnwnwnwne",
        "seneneneneenwneneswwnwswneneswnesenenw",
        "swswnewwnenwewswwswswswsesewswsww",
        "neeswwnwnwnwnwswnewenenwnwnwnwnwesewne",
        "nwswseswewnewswnwswwsweswseeneseswne",
        "sesenwswesenweenwwnwesw",
        "seeseseseswnwseseseswseseswsesesesesese",
        "esewseeesweseseseseesenenweseese",
        "nwswseenweeneeeeeseenwseenwsesesw",
        "enweeeseeesenwsweee",
        "nenwneneeeneneneeneeseeneeneneneew",
        "swswseswneswseswswswse",
        "enenenenwnwnwneswnenwnwnwnwneeneswnenenw",
        "nenenwneswnenwnwnenwnweswnwnenwnwnenenwne",
        "swseesewseeenwenwnwsenenwweseese",
        "swseenenenenenenenenenwwnwnenwwenenene",
        "nwnwnweneneewwnwsenenenenwswswnesene",
        "seeswswesenwneeneeenweswesesenww",
        "eeeeeneseeeeweeeeeesweee",
        "enwswswneswswswwswswswswseswswswswswswsw",
        "sewsesesesesesenesesesesesesesesenesesw",
        "nesenesenenewnweneeenenenenewnee",
        "swseseseswsweswsesesesweswnwswswseswsesenw",
        "eeeeeeeweeeeeeweeeeee",
        "nwnwnwnwnenwnenwnwesenwwnwnwnenwnwnwnw",
        "wswwnwewnwwwswswwwswswswwswswe",
        "eeenweeneeeeeeeweeeseseee",
        "seswseseseswsesesesesenesesesese",
        "wnwnwwsewswwwewwwnwwneweseswne",
        "nwnwnwnweswnwnwnwnwwnewnwswnwnwnwnwnw",
        "seseeseewseswsewsesesewneeseswsenene",
        "seseneseswseesenwsesesenenwseseswwsesese",
        "wswwnwenwwnwswnewwwwwnwnenwwnw",
        "ewewwwwwwswwwwwewnwwsww",
        "wwsewswwwwnewneswwwswwnenwwne",
        "wsewnwnwnwwwwwnweeswnewnwwnwnw",
        "nwnwwnwnwnwnwnwswnwenwewswnwwwwnwnw",
        "newwwewswsenwnwnwwwsew",
        "swswwseswnwnweswwsweswwwwswnwswnwse",
        "eseseseenweewseseseseseeseeeenwe",
        "eseseeeswesesewnwenwseenwseseesene",
        "wwnwwnwseewnwnwnenwsesenwnwswnwsene",
        "eseeseseewseseseeseese",
        "nwenwneswweneeeswseeseenwneee",
        "wweswwwswwseswwswswswwwswwnew",
        "seenwwseswwseeneseenwsenwswe",
        "nenenenwwneswnenenwnwsesewseneneneenwswnw",
        "nwnenenenenwneneneswnwnenwnwenene",
        "sewnwswesewsewneneswwwsenwne",
        "wnwnwnwnwnenwnenwwswnwnwnwnwnwwwswnw",
        "sweseneseseswswswswseswnwswseswwswsww",
        "wenwwwsenwseswnwnwwwnenwnwnwnwnwww",
        "seneswswswswneswswswswswswswwswswwsw",
        "nwnwnwenwnwnwwnwnwnwnwnwnwnwnwnwnwswnw",
        "nwnwesenenenwnenwnwnewnenewnenwnwswnwsenw",
        "nwwnwwenwnwnwsenwnwnwwewwnwnwwnw",
        "nwnwnwnwnwnwswnwnwnwnenwnwnenwswnenw",
        "nwnwnwnenwnwnwnwnwnesenenenenwnwnwnww",
        "swsenwnwwnesenwnwwnwnwsewnwseenwnwne",
        "swswenwswseswswswwswswswseswswnwseswswse",
        "wneneneweseneswswswsenww",
        "nwneswswnwswnwenenwwneswnwnenenenesenwne",
        "nenenwneseswnenenenenwneneeneenewee",
        "nwwwewwnwnwnewsewswnewesewnww",
        "nwswneswseseseswseseswseseswseseseseswse",
        "swswwwwwswwwnwswsew",
        "swsweswneswswswnweswwswswswswswwsww",
        "swswswswswswswswenwwwswswwswneswswsw",
        "seeeeseweswswenwneseeseenwnwswseww",
        "enweeneeenweneeseseneeweseee",
        "nwswnwenenwnwnwnwswnwnwnwenwsenwwwse",
        "nweewwnwesenewswnwwswsenwwnwsww",
        "enwnwswwseswnewsewneeeswseew",
        "neneenesweeneeneneneneewnwenenese",
        "swwneneseenesewsesesenewesesesesewsenw",
        "wnwwnwwwwwewwwwsww",
        "wnwswwnwsesewwwwwnewnenww",
        "nwwnwnwnenwswnwneneenwnwswnenwnwnenenw",
        "eswswewwswwnenwewneneswswewsesw",
        "neswwswswwswswswwswswswwswwewswww",
        "eseenwweseseeeeneeeseeenwese",
        "neeeeweeeeeneenwesesweswneswse",
        "nwwwnwwwewwnwswnwnwwenwsenwwwse",
        "nwnwnwnwnwnenenwnwnwnwnwnwnwnenwnesenwsene",
        "nwseswswswswseswswseswswswswswswswswneswsw",
        "wnwnenwwenwnwwewwenwwsenwwswwswnw",
        "seseswneswnwswwswwseseswneenenenesww",
        "esesewseeneseswwsesesesesesesesesese",
        "wwnwnwnewnwnwsenwnwwnwenwnwsenwsenw",
        "swwwneswwwwwwwwwewswwwww",
        "seneneeseneewswnwswswnewnenwwnenenwswne",
        "swswseswwwswwwswwswwwwwwwnew",
        "eeseseewseseseenwseeeeeseseesese",
        "enenenenwnwswnenwnenwnenenwneswswnenw",
        "nesewswsesesesesenesesenwseeseneseswswe",
        "neeneesweswneeswneenwenweenenwsw",
        "swnenwwsenwneswneneswnenenenenesweene",
        "wnwewwnwwwwwnwnwsew",
        "nwnwneswwnwnwswneenwnenwneneeenenwnw",
        "neeeneseeewneseswnwseswnw",
        "nwseenenewenwnewneswnenwenwswsenene",
        "wnenwnewnwnwnwnwnwenwnesenwnwnenenwnwnwnw",
        "newwwwswwnwwswwswswewsewweww",
        "neeenenenwneneseneseseenenenwnewnene",
        "wnweseswseseenwnw",
        "senwswseswnweseseswseswswswswswswswenw",
        "nwnwnwnwnwsenwswnwnwnwnenwwnwnwnenesenw",
        "eneneneenesenesweeeeeeeeeewe",
        "nwnenwneneswnwnwnwnwnenwnwnenenwewnwnee",
        "neeenwseeeneeeeneeeeseenenew",
        "wsweeswseswswswswweswewnwneswseswnw",
        "swswnwseswswswnwswswswswswswwwswswenwse",
        "wnweswwswswnwseswswneswswsewswswsw",
        "nwnenenwnenwnwnenenwewnenwnwnwsenwnwnesw",
        "wnenenesenwnenenenenesenenenenewnenenenene",
        "swnwnewswswseseswseswwnwnwsweneswsew",
        "neeeeneenenenenenwneneswweenenenene",
        "nwewswswnenenwnwwswweweneeesenwne",
        "sesewenesesewsewsesenesesenwnesesenwsw",
        "nwewwnwwwwnwwwwswwwwe",
        "eseseesewsesesesewneseneeseeseesesew",
        "neneneeneneeeeeneenenesw",
        "newewswsesewnenwwwswwwnwwsw",
        "sewwnwewwsesenenwnwswswnenenw",
        "nwnwnwsewwswesenee",
        "ewnwseewswnwwswwwwsw",
        "wwwenewwesewwwwwwwwwww",
        "wnwseeswenenwww",
        "neeseseeesesewwneeeeseswneeseswe",
        "wwsewswwswwswwswwswnewswswswwsw",
        "eeseeweseenweseseswsesenesesesese",
        "senenwwnwwwnwswnwewwnwwswnwwnww",
        "nwnwnwswnwnenwnwnwsesenenenwnwnenenenenw",
        "wswnwswswewnwwwnwseneesenewswwew",
        "nweeswesweeeeeeeeeeeenwe",
        "eeneeenenwenweeeeseneesweew",
        "seewswseswsweswwenwswswseswswnweswswsw",
        "seswenewwswswnewnwwsenenewswswwwse",
        "nenewweeneeseneeneeswnenenewnene",
        "neweswnenwnwswneenwwsenwsenwenewnw",
        "nwnwneseswnenweenenenenesewnenwsenenwwe",
        "nesweswenesweneenweenewsenenenwnw",
        "seeeseeeseseseenenweeseeenweseew",
        "nenewneeswneneneeneneseneeeneneenenee",
        "swswswswnwwseseseswnwsesenwseseeesesene",
        "swnwsesenwneseenwsesesesewsesweseeee",
        "nwneneneneseneneneneneneneew",
        "wswwwwwwwsenwnwwnwnwwwwneww",
    ]
}
