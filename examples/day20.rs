#![feature(split_inclusive)]

use std::{
    collections::{BTreeMap, HashMap, HashSet},
    io::BufRead,
};

use aoc2020::math::Vec2;
use multimap::MultiMap;
use multiset::HashMultiSet;

fn reverse10(v: u16) -> u16 {
    (0..10).map(|i| ((v >> (9 - i)) & 1) << i).sum::<u16>()
}

fn r10id(v: u16) -> u16 {
    v.min(reverse10(v))
}

fn parse_slice(s: &str) -> u16 {
    s.chars()
        .enumerate()
        .map(|(i, c)| if c == '#' { 1 } else { 0 } << (9 - i))
        .sum()
}

#[derive(Debug)]
struct Tile {
    id: i64,
    edge_pattern: [u16; 4],
    edge_ids: [u16; 4],
    neighbors: Vec<i64>,
}

impl Tile {
    fn new(id: i64, lines: &[String]) -> Self {
        let edge_pattern = [0; 4];
        let edge_ids = [0; 4];

        let top = parse_slice(lines.first().unwrap());
        let bottom = parse_slice(lines.last().unwrap());
        let left = parse_slice(
            &lines
                .iter()
                .map(|l| l.chars().nth(0).unwrap())
                .collect::<String>(),
        );
        let right = parse_slice(
            &lines
                .iter()
                .map(|l| l.chars().nth_back(0).unwrap())
                .collect::<String>(),
        );
        Tile {
            id,
            edge_pattern: [top, bottom, left, right],
            edge_ids: [r10id(top), r10id(bottom), r10id(left), r10id(right)],
            neighbors: Vec::new(),
        }
    }

    fn is_corner(&self) -> bool {
        self.neighbors.len() == 2
    }
    fn is_outer(&self) -> bool {
        self.neighbors.len() == 3
    }
    fn is_inner(&self) -> bool {
        self.neighbors.len() == 4
    }
    fn edge_id_set(&self) -> HashSet<u16> {
        self.edge_ids.iter().cloned().collect::<HashSet<_>>()
    }
}

fn main() {
    let mut tiles = aoc2020::skip_empty_input()
        .chunks(11)
        .map(|tile| {
            let title = tile.first().unwrap();
            assert!(title.starts_with("Tile"));
            let tileid: i64 = title[5..9].parse().unwrap();
            println!("tile: {:?} {:?}", tile, Tile::new(tileid, &tile[1..]));
            Tile::new(tileid, &tile[1..])
        })
        .collect::<Vec<_>>();

    // let all_edge_ids = tiles
    //     .iter()
    //     .flat_map(|t| t.edge_ids.iter())
    //     .cloned()
    //     .collect::<HashMultiSet<_>>();
    {
        let edge_id_map = tiles
            .iter()
            .flat_map(|tile| tile.edge_pattern.iter().map(move |p| (r10id(*p), tile.id)))
            .collect::<MultiMap<_, _>>();
        println!("edge id map: {:?}", edge_id_map);

        for tile in tiles.iter_mut() {
            for edge_id in tile.edge_ids.iter() {
                let ns = edge_id_map.get_vec(edge_id).unwrap();
                if ns.len() == 2 {
                    tile.neighbors
                        .push(if tile.id == ns[0] { ns[1] } else { ns[0] });
                }
            }
        }
    }
    let id_map = tiles
        .iter()
        .map(|tile| (tile.id, tile))
        .collect::<HashMap<_, _>>();

    let res: i64 = tiles
        .iter()
        .filter(|t| t.neighbors.len() == 2)
        .map(|t| t.id)
        .product();

    let corners = tiles
        .iter()
        .filter(|t| t.neighbors.len() == 2)
        .collect::<Vec<_>>();
    let outer = tiles
        .iter()
        .filter(|t| t.neighbors.len() == 3)
        .collect::<Vec<_>>();
    let inner = tiles
        .iter()
        .filter(|t| t.neighbors.len() == 4)
        .collect::<Vec<_>>();

    println!(
        "{} {} {} {}",
        corners.len(),
        outer.len(),
        inner.len(),
        tiles.len()
    );
    let edge_len = outer.len() / 4 + 2;
    assert!(edge_len * edge_len == tiles.len());

    println!("res: {}", res);

    let mut assignment = HashMap::new();
    let mut cur_tile = corners[0];
    // corner_loop.push(cur_tile);
    let mut closed = HashSet::new();

    let directions = [Vec2(1, 0), Vec2(0, 1), Vec2(-1, 0), Vec2(0, -1)];
    let mut pos = Vec2(0, 0);

    for dir in directions.iter() {
        loop {
            assignment.insert(pos, cur_tile);

            pos += *dir;
            // cur_tile
            // next find neighbor along edge that is not closed
            for n in cur_tile.neighbors.iter() {
                if id_map[n].is_inner() || closed.contains(n) {
                    continue;
                }
                cur_tile = id_map[n];
                println!("next: {}", cur_tile.id);
                break;
            }
            closed.insert(cur_tile.id);
            if cur_tile.is_corner() {
                break;
            }
        }
    }

    for (pos, tile) in assignment.iter() {
        println!("{:?} -> {}", pos, tile.id);
    }
    for y in 1..11 {
        for x in 1..11 {
            let pos = Vec2(x, y);
            let left = pos + Vec2(-1, 0);
            let down = pos + Vec2(0, -1);
            assert!(assignment.contains_key(&left) & assignment.contains_key(&down));
            let edge_set = assignment[&left]
                .edge_id_set()
                .union(&assignment[&down].edge_id_set())
                .cloned()
                .collect::<HashSet<_>>();

            for tile in tiles.iter() {
                if edge_set.intersection(&tile.edge_id_set()).count() == 2 {
                    println!("next: {:?} {:?}", pos, tile);
                    assignment.insert(pos, tile);
                }
            }
        }
    }
    // let mut ns = (0..=0b1111111111)
    //     .map(|v| (r10id(v), v))
    //     .collect::<Vec<_>>();

    // ns.sort_by_key(|(k, v)| *k);
    // for (k, v) in ns.iter() {
    //     println!("{:010b} -> {:010b}", v, k);
    // }
}
