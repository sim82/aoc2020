#![feature(split_inclusive)]

use std::collections::{HashMap, HashSet};

use aoc2020::math::Vec2;
use multimap::MultiMap;

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
    neighbors: [Option<i64>; 4],
    lines: Vec<Vec<bool>>,
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
            neighbors: [None; 4],
            lines: lines
                .iter()
                .map(|s| s.chars().map(|c| c == '#').collect::<Vec<_>>())
                .collect(),
        }
    }

    fn is_corner(&self) -> bool {
        self.neighbors.iter().filter_map(|f| *f).count() == 2
    }
    fn is_outer(&self) -> bool {
        self.neighbors.iter().filter_map(|f| *f).count() == 3
        // self.neighbors.len() == 3
    }
    fn is_inner(&self) -> bool {
        self.neighbors.iter().filter_map(|f| *f).count() == 4
        // self.neighbors.len() == 4
    }
    fn edge_id_set(&self) -> HashSet<u16> {
        self.edge_ids.iter().cloned().collect::<HashSet<_>>()
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Edge {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Axis {
    X,
    Y,
    Diag,
}
impl Edge {
    fn main_axis(&self) -> Axis {
        match self {
            Edge::Top | Edge::Bottom => Axis::X,
            Edge::Left | Edge::Right => Axis::Y,
        }
    }
    fn flip_diagonal(&self) -> Edge {
        match self {
            Edge::Top => Edge::Left,
            Edge::Left => Edge::Top,
            Edge::Bottom => Edge::Right,
            Edge::Right => Edge::Bottom,
        }
    }
    fn flip_x(&self) -> Edge {
        match self {
            Edge::Top => Edge::Bottom,
            Edge::Bottom => Edge::Top,
            _ => *self,
        }
    }
    fn flip_y(&self) -> Edge {
        match self {
            Edge::Left => Edge::Right,
            Edge::Right => Edge::Left,
            _ => *self,
        }
    }
    fn flip(&self, axis: Axis) -> Edge {
        match axis {
            Axis::X => self.flip_x(),
            Axis::Y => self.flip_y(),
            Axis::Diag => self.flip_diagonal(),
        }
    }
    fn from_index(i: usize) -> Edge {
        match i {
            0 => Edge::Top,
            1 => Edge::Bottom,
            2 => Edge::Left,
            3 => Edge::Right,
            _ => panic!("unhandled"),
        }
    }
    fn into_index(&self) -> usize {
        match self {
            Edge::Top => 0,
            Edge::Bottom => 1,
            Edge::Left => 2,
            Edge::Right => 3,
        }
    }
}

fn rotate_to(mut s0: Edge, e0: Edge, mut s1: Edge, e1: Edge) -> Vec<Axis> {
    if s0.main_axis() == s1.main_axis() || e0.main_axis() == e1.main_axis() {
        return vec![];
    }
    let mut ret = Vec::new();
    while s0 != e0 || s1 != e1 {
        let next_flip = if s0.main_axis() != e0.main_axis() {
            Axis::Diag
        } else if s0 != e0 {
            s0.main_axis()
        } else if s1 != e1 {
            s1.main_axis()
        } else {
            panic!("unhandled");
        };
        // println!("flip: {:?}", next_flip);
        ret.push(next_flip);
        s0 = s0.flip(next_flip);
        s1 = s1.flip(next_flip);
    }
    ret
}

fn apply_flips<T: Copy>(mut a: [T; 4], flips: &Vec<Axis>) -> [T; 4] {
    for flip in flips {
        a = match flip {
            Axis::Diag => [a[2], a[3], a[0], a[1]],
            Axis::X => [a[1], a[0], a[2], a[3]],
            Axis::Y => [a[0], a[1], a[3], a[2]],
        };
    }
    a
}

fn apply_flips_image(mut image: Vec<Vec<bool>>, flips: &Vec<Axis>) -> Vec<Vec<bool>> {
    for flip in flips {
        match flip {
            Axis::Diag => {
                for y in 0..image.len() {
                    assert!(image[y].len() == image.len());
                    for x in 0..y {
                        let tmp = image[x][y];
                        image[x][y] = image[y][x];
                        image[y][x] = tmp;
                    }
                }
            }
            Axis::X => image.reverse(),
            Axis::Y => image.iter_mut().for_each(|l| l.reverse()),
        }
    }
    image
}

#[test]
fn test_edge() {
    assert_eq!(
        rotate_to(Edge::Right, Edge::Top, Edge::Bottom, Edge::Left),
        vec![Axis::Diag, Axis::X, Axis::Y]
    );
    assert_eq!(
        rotate_to(Edge::Top, Edge::Top, Edge::Left, Edge::Left),
        vec![]
    );
    assert_eq!(
        rotate_to(Edge::Top, Edge::Left, Edge::Right, Edge::Bottom),
        vec![Axis::Diag]
    );
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

    {
        let edge_id_map = tiles
            .iter()
            .flat_map(|tile| tile.edge_pattern.iter().map(move |p| (r10id(*p), tile.id)))
            .collect::<MultiMap<_, _>>();
        println!("edge id map: {:?}", edge_id_map);

        for tile in tiles.iter_mut() {
            for (i, edge_id) in tile.edge_ids.iter().enumerate() {
                let ns = edge_id_map.get_vec(edge_id).unwrap();
                if ns.len() == 2 {
                    tile.neighbors[i] = Some(if tile.id == ns[0] { ns[1] } else { ns[0] });
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
        .filter(|t| t.is_corner())
        .map(|t| t.id)
        .product();

    let corners = tiles.iter().filter(|t| t.is_corner()).collect::<Vec<_>>();
    let outer = tiles.iter().filter(|t| t.is_outer()).collect::<Vec<_>>();

    let edge_len = outer.len() / 4 + 2;
    assert!(edge_len * edge_len == tiles.len());

    println!("res: {}", res);

    // collect known flips by tile.id
    let mut tile_flips = HashMap::<i64, Vec<Axis>>::new();
    // assigned tiles by x,y pos
    let mut assignment = HashMap::new();
    // set of tiles for which assignment / flips have been resolved
    let mut closed = HashSet::new();

    // choose arbitrary corner piece to start
    let upper_left = corners[0];
    println!(
        "neighbors: {:?}",
        &upper_left
            .neighbors
            .iter()
            .map(|n| n.is_none())
            .collect::<Vec<_>>()[0..4]
    );
    // rotate open edges of start tile to top/left
    let open_edges = upper_left
        .neighbors
        .iter()
        .enumerate()
        .filter_map(|(i, n)| {
            if n.is_none() {
                Some(Edge::from_index(i))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    println!("open edges: {:?} {:?}", upper_left, open_edges);
    tile_flips.insert(
        upper_left.id,
        rotate_to(open_edges[0], Edge::Top, open_edges[1], Edge::Left),
    );
    closed.insert(upper_left.id);
    assignment.insert(Vec2(0, 0), upper_left);

    // expand solution along outer edges starting from corner
    for axis in [Axis::X, Axis::Y].iter() {
        let dir = match axis {
            Axis::X => Vec2(1, 0),
            Axis::Y => Vec2(0, 1),
            _ => panic!("unhandled"),
        };
        // direction of the edge we are currently expanding
        let (leading_edge, trailing_edge) = match axis {
            Axis::X => (Edge::Right, Edge::Left),
            Axis::Y => (Edge::Bottom, Edge::Top),
            _ => panic!("unhandled"),
        };
        // direction towards outside ()
        let open_edge = match axis {
            Axis::X => Edge::Top,
            Axis::Y => Edge::Left,
            _ => panic!("unhandled"),
        };
        let mut cur_tile = upper_left;
        for i in 1..(edge_len as i32) {
            // make cur_tile.neighbors point in the correct directions by simulating tile flips
            let neighbors = apply_flips(cur_tile.neighbors, &tile_flips[&cur_tile.id]);

            // next neighbor along leading edge
            let n = neighbors[leading_edge.into_index()].unwrap();

            assert!(!(id_map[&n].is_inner() || closed.contains(&n))); // sanity check: don't move towards inner/closed neighbors

            let last_tile = cur_tile;

            // step to neighbor
            cur_tile = id_map[&n];
            assignment.insert(dir * i, cur_tile);
            closed.insert(cur_tile.id);

            // determine which of our edges should point towards trailing edge (i.e. which leads to last_tile)
            let to_trailing = Edge::from_index(
                cur_tile
                    .neighbors
                    .iter()
                    .position(|n| match n {
                        Some(n) => *n == last_tile.id,
                        None => false,
                    })
                    .unwrap(),
            );

            // determine which of our edges should point towards outside (i.e. the edge without neighbor that is not on the same axis as to_trailing (special case for corners!))
            let to_open = Edge::from_index(
                cur_tile
                    .neighbors
                    .iter()
                    .enumerate()
                    .position(|(i, n)| {
                        n.is_none() && Edge::from_index(i).main_axis() != to_trailing.main_axis()
                    })
                    .unwrap(),
            );

            // resolve to flip sequence
            tile_flips.insert(
                cur_tile.id,
                rotate_to(to_trailing, trailing_edge, to_open, open_edge),
            );

            // println!("{:?} {}", dir * i, cur_tile.id);
            // println!("next: {}", cur_tile.id);
            // break;
        }
    }

    // assign remaining tiles to positions and flips, using the known tile above and left
    for y in 1..(edge_len as i32) {
        for x in 1..(edge_len as i32) {
            let pos = Vec2(x, y);
            let left = pos + Vec2(-1, 0);
            let top = pos + Vec2(0, -1);
            assert!(assignment.contains_key(&left) & assignment.contains_key(&top));

            let left_tile = assignment[&left];
            let top_tile = assignment[&top];

            let left_edges = left_tile.edge_id_set();
            let top_edges = top_tile.edge_id_set();

            // this might be refined since we already know the correct flips of top & left
            let edge_set = left_edges
                .union(&top_edges)
                .cloned()
                .collect::<HashSet<_>>();

            for tile in tiles.iter() {
                // determine non-closed tile that shares two edges with top & left
                if !closed.contains(&tile.id)
                    && edge_set.intersection(&tile.edge_id_set()).count() == 2
                {
                    // determine which edge should point up (i.e. which of its neighbors is the top tile)
                    let to_top = Edge::from_index(
                        tile.neighbors
                            .iter()
                            .position(|n| match n {
                                Some(n) => *n == top_tile.id,
                                None => false,
                            })
                            .unwrap(),
                    );
                    // determine which edge should point left (i.e. which of its neighbors is the left tile)
                    let to_left = Edge::from_index(
                        tile.neighbors
                            .iter()
                            .position(|n| match n {
                                Some(n) => *n == left_tile.id,
                                None => false,
                            })
                            .unwrap(),
                    );
                    // resolve flips
                    let flips = rotate_to(to_top, Edge::Top, to_left, Edge::Left);
                    // println!("next: {:?} {:?} {:?}", pos, tile, flips);
                    tile_flips.insert(tile.id, flips);
                    assignment.insert(pos, tile);
                    closed.insert(tile.id);
                    break;
                }
            }
        }
    }

    // re-create image
    let mut out_image = HashSet::<Vec2>::new();
    // let mut out_image2: Vec<Vec<bool>> = vec![vec![false]];
    for y in 0..(edge_len as i32) {
        for x in 0..(edge_len as i32) {
            let pos = Vec2(x, y);
            let tile = assignment[&pos];
            // println!("assign: {:?} {}", pos, tile.id);
            let image = apply_flips_image(tile.lines.clone(), &tile_flips[&tile.id]);
            for yinner in 1..(image.len() - 1) {
                for xinner in 1..(image[yinner].len() - 1) {
                    if image[yinner][xinner] {
                        out_image.insert(
                            pos * (image.len() - 2) as i32
                                + Vec2(xinner as i32 - 1, yinner as i32 - 1),
                        );
                    }
                }
            }
        }
    }
    print_field(&out_image);

    let field_width = out_image.iter().map(|v| v.x()).max().unwrap() + 1;
    let field_height = out_image.iter().map(|v| v.y()).max().unwrap() + 1;

    // there be monsters...
    let monster = [
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   ",
    ];
    let monster_height = monster.len();
    let monster_width = monster[0].len();

    // collect union of all matched monster pattern
    let mut monster_union = HashSet::new();

    for transformation in 0..8 {
        // try to match all 8 flips / rotations of the monster patter (rather than flipping the image)
        let flip_x = transformation & 0b1 != 0;
        let flip_y = transformation & 0b10 != 0;
        let flip_diag = transformation & 0b100 != 0;
        let mut monster = monster.clone();

        if flip_x {
            monster.reverse();
        }
        let mut monster_pos = Vec::new();
        for (y, line) in monster.iter().enumerate() {
            let line = if flip_y {
                line.chars().rev().collect()
            } else {
                line.to_string()
            };
            for (x, c) in line.chars().enumerate() {
                let pos = if flip_diag {
                    Vec2(y as i32, x as i32)
                } else {
                    Vec2(x as i32, y as i32)
                };
                if c == '#' {
                    monster_pos.push(pos);
                }
            }
        }

        let (monster_width, monster_height) = if flip_diag {
            (monster_height as i32, monster_width as i32)
        } else {
            (monster_width as i32, monster_height as i32)
        };

        for y in 0..=(field_height - monster_height) {
            for x in 0..=(field_width - monster_width) {
                let monster_offset = Vec2(x, y);
                let monster_pos_pattern = monster_pos
                    .iter()
                    .map(|v| *v + monster_offset)
                    .collect::<HashSet<_>>();

                let hit = out_image.intersection(&monster_pos_pattern);
                if hit.count() == monster_pos_pattern.len() {
                    println!("hit: {:?} {:?}", transformation, monster_offset);
                    monster_union.extend(monster_pos_pattern.iter());
                }
            }
        }
    }
    print_field(&monster_union);

    println!(
        "roughness: {}",
        out_image.difference(&monster_union).count()
    );

    // let mut ns = (0..=0b1111111111)
    //     .map(|v| (r10id(v), v))
    //     .collect::<Vec<_>>();

    // ns.sort_by_key(|(k, v)| *k);
    // for (k, v) in ns.iter() {
    //     println!("{:010b} -> {:010b}", v, k);
    // }
}

fn print_field(out_image: &HashSet<Vec2>) {
    // keep up the good tradition to store bitmaps in a HashSet (2d arrays stink...)
    let maxx = out_image.iter().map(|v| v.x()).max().unwrap();
    let maxy = out_image.iter().map(|v| v.y()).max().unwrap();
    for y in 0..=maxy {
        let mut line = String::new();
        for x in 0..=maxx {
            if out_image.contains(&Vec2(x, y)) {
                line.push('#');
            } else {
                line.push('.');
            }
        }
        println!("{}", line);
    }
}
