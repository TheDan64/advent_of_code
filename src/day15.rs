use std::collections::{HashMap, HashSet};
use std::fmt::{self, Display, Formatter};
use std::mem::replace;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Tile {
    Elf { hp: u8 },
    Goblin { hp: u8 },
    Open,
    Wall,
}

impl Tile {
    fn as_char(&self) -> char {
        match self {
            Tile::Elf { .. } => 'E',
            Tile::Goblin { .. } => 'G',
            Tile::Open => '.',
            Tile::Wall => '#',
        }
    }

    fn take_damage(&mut self, damage: u8) -> bool {
        let hp = match self {
            Tile::Elf { ref mut hp } => hp,
            Tile::Goblin { ref mut hp } => hp,
            _ => panic!("Attacking non sentient creature"),
        };
        // let is_elf = match self {
        //     Tile::Elf { .. } => true,
        //     _ => false,
        // };

        // println!("{} takes {} damage to {} hp", if is_elf { "Elf" } else { "Goblin" }, damage, hp);

        // Take damage or dies
        if *hp > damage {
            *hp -= damage;

            false
        } else {
            *self = Tile::Open;

            true
        }
    }

    fn is_npc(&self) -> bool {
        match self {
            Tile::Elf { .. } => true,
            Tile::Goblin { .. } => true,
            _ => false,
        }
    }

    fn hp(&self) -> u8 {
        match self {
            Tile::Elf { hp } => *hp,
            Tile::Goblin { hp } => *hp,
            _ => panic!("Tried getting hp on non npc tile"),
        }
    }

    #[cfg(test)]
    fn is_elf(&self) -> bool {
        match self {
            Tile::Elf { .. } => true,
            _ => false,
        }
    }

    #[cfg(test)]
    fn is_goblin(&self) -> bool {
        match self {
            Tile::Goblin { .. } => true,
            _ => false,
        }
    }
}

fn manhattan_dist(index1: usize, index2: usize, map_width: usize) -> u64 {
    let x1 = index1 % map_width;
    let x2 = index2 % map_width;
    let y1 = index1 / map_width;
    let y2 = index2 / map_width;

    let dist = (x1 as isize - x2 as isize).abs() + (y1 as isize - y2 as isize).abs();

    dist as u64
}

// For moves?
// enum FourSlotVec<T> {
//     None,
//     One(T),
//     Two(T, T),
//     Three(T, T, T),
//     Four(T, T, T, T),
// }

#[derive(Debug, Copy, Clone)]
struct AStarPath {
    next_move: usize,
    last_move: Option<usize>,
    len: u16,
}

#[derive(Clone)]
pub struct Map {
    elves: HashSet<usize>,
    goblins: HashSet<usize>,
    tiles: Vec<Tile>,
    total_turns: u8,
    width: usize,
}

impl Map {
    fn elves_won(&self) -> bool {
        self.goblins.is_empty()
    }

    fn count_elves(&self) -> usize {
        self.elves.len()
    }

    fn sum_total_hp(&self) -> u64 {
        let mut total_hp = 0;

        for elf_index in &self.elves {
            match self.tiles[*elf_index] {
                Tile::Elf { hp } => total_hp += hp as u64,
                e => unreachable!("Found {:?} but expected Elf", e),
            }
        }

        for goblin_index in &self.goblins {
            match self.tiles[*goblin_index] {
                Tile::Goblin { hp } => total_hp += hp as u64,
                e => unreachable!("Found {:?} but expected Goblin", e),
            }
        }

        total_hp
    }

    fn get_moves(&self, index: usize, open_only: bool) -> Vec<usize> {
        let mut moves = vec![];

        // Up, not in top row
        if index / self.width > 0 {
            let tile_index = index - self.width;
            let tile = self.tiles[tile_index];

            match tile {
                Tile::Open => moves.push(tile_index),
                _ if !open_only => moves.push(tile_index),
                _ => {},
            }
        }

        // Left, not in leftmost column
        if index % self.width > 0 {
            let tile_index = index - 1;
            let tile = self.tiles[tile_index];

            match tile {
                Tile::Open => moves.push(tile_index),
                _ if !open_only => moves.push(tile_index),
                _ => {},
            }
        }

        // Right, not in rightmost column
        if index % self.width < self.width - 1 {
            let tile_index = index + 1;
            let tile = self.tiles[tile_index];

            match tile {
                Tile::Open => moves.push(tile_index),
                _ if !open_only => moves.push(tile_index),
                _ => {},
            }
        }

        // Down, not in bottom row
        if index < self.tiles.len() - self.width {
            let tile_index = index + self.width;
            let tile = self.tiles[tile_index];

            match tile {
                Tile::Open => moves.push(tile_index),
                _ if !open_only => moves.push(tile_index),
                _ => {},
            }
        }

        // Can stay in place, last choice? Maybe should be first?
        // moves.push(index);
        moves
    }

    /// Pick the enemy with lowest hp in reading order in reading order
    fn get_nearby_enemy(&self, index: usize, is_elf: bool) -> Option<usize> {
        let mut enemy = None;

        // Up, not in top row
        if index / self.width > 0 {
            let tile_index = index - self.width;
            let tile = self.tiles[tile_index];

            match tile {
                Tile::Goblin { hp } if is_elf => {
                    match enemy {
                        None => enemy = Some((tile_index, hp)),
                        Some((_, enemy_hp)) => {
                            if hp < enemy_hp {
                                enemy = Some((tile_index, hp));
                            }
                        }
                    }
                },
                Tile::Elf { hp } if !is_elf => {
                    match enemy {
                        None => enemy = Some((tile_index, hp)),
                        Some((_, enemy_hp)) => {
                            if hp < enemy_hp {
                                enemy = Some((tile_index, hp));
                            }
                        }
                    }
                },
                _ => {}
            }
        }

        // Left, not in leftmost column
        if index % self.width > 0 {
            let tile_index = index - 1;
            let tile = self.tiles[tile_index];

            match tile {
                Tile::Goblin { hp } if is_elf => {
                   match enemy {
                        None => enemy = Some((tile_index, hp)),
                        Some((_, enemy_hp)) => {
                            if hp < enemy_hp {
                                enemy = Some((tile_index, hp));
                            }
                        }
                    }

                },
                Tile::Elf { hp } if !is_elf => {
                   match enemy {
                        None => enemy = Some((tile_index, hp)),
                        Some((_, enemy_hp)) => {
                            if hp < enemy_hp {
                                enemy = Some((tile_index, hp));
                            }
                        }
                    }

                },
                _ => {}
            }
        }

        // Right, not in rightmost column
        if index % self.width < self.width - 1 {
            let tile_index = index + 1;
            let tile = self.tiles[tile_index];

            match tile {
                Tile::Goblin { hp } if is_elf => {
                   match enemy {
                        None => enemy = Some((tile_index, hp)),
                        Some((_, enemy_hp)) => {
                            if hp < enemy_hp {
                                enemy = Some((tile_index, hp));
                            }
                        }
                    }

                },
                Tile::Elf { hp } if !is_elf => {
                   match enemy {
                        None => enemy = Some((tile_index, hp)),
                        Some((_, enemy_hp)) => {
                            if hp < enemy_hp {
                                enemy = Some((tile_index, hp));
                            }
                        }
                    }

                },
                _ => {}
            }
        }

        // Down, not in bottom row
        if index < self.tiles.len() - self.width {
            let tile_index = index + self.width;
            let tile = self.tiles[tile_index];

            match tile {
                Tile::Goblin { hp } if is_elf => {
                   match enemy {
                        None => enemy = Some((tile_index, hp)),
                        Some((_, enemy_hp)) => {
                            if hp < enemy_hp {
                                enemy = Some((tile_index, hp));
                            }
                        }
                    }

                },
                Tile::Elf { hp } if !is_elf => {
                   match enemy {
                        None => enemy = Some((tile_index, hp)),
                        Some((_, enemy_hp)) => {
                            if hp < enemy_hp {
                                enemy = Some((tile_index, hp));
                            }
                        }
                    }

                },
                _ => {}
            }
        }

        enemy.map(|(i, _)| i)
    }

    // FIXME
    fn get_shortest_path(&self, index: usize, index2: usize) -> AStarPath {
        let mut open = Vec::new();
        let mut closed = Vec::new();
        let mut f_vals: HashMap<usize, u64> = HashMap::new();
        let mut g_vals: HashMap<usize, u64> = HashMap::new();
        let mut h_vals: HashMap<usize, u64> = HashMap::new();
        let mut parents: HashMap<usize, usize> = HashMap::new();
        // let start_index = index;

        assert_ne!(self.tiles[index2], Tile::Wall);
        assert_ne!(self.tiles[index2], Tile::Open);

        f_vals.insert(index, 0);
        g_vals.insert(index, 0);
        h_vals.insert(index, 0);
        open.push(index);

        // println!("searching from {}", index);

        'w: while !open.is_empty() {
            // A-B) Find node with least f-val and remove from open list
            // let item = open.iter().min_by(|lhs, rhs| {
            //     let lhs = f_vals.get(*lhs).unwrap();
            //     let rhs = f_vals.get(*rhs).unwrap();

            //     lhs.cmp(rhs)
            // }).unwrap();
            // let pos = open.iter().position(|i| i == item).unwrap();
            // println!("open: {:?}", open);
            let index = open.remove(0);

            // C) Generate successors and set their parents to index
            let successors = self.get_moves(index, false);

            // Push onto closed list
            closed.push(index);

            // println!("index: {} successors: {:?}", index, successors);

            // D)
            for successor_index in &successors {
                // println!("looking at {}", successor_index);

                // I)
                // assert_eq!(self.tiles[*successor_index], Tile::Open);
                // println!("{} ({:?}) vs {} ({:?})", successor_index, self.tiles[*successor_index], index2, self.tiles[index2]); idx 33
                if *successor_index == index2 {
                    // println!("==> Found index2 {} for index {}", index2, start_index);
                    parents.insert(*successor_index, index);

                    break 'w;
                }

                // We only allow looking at non npc to find end position
                if self.tiles[*successor_index] != Tile::Open {
                    g_vals.insert(*successor_index, u64::max_value() - 10);
                    h_vals.insert(*successor_index, u64::max_value() - 10);
                    f_vals.insert(*successor_index, u64::max_value() - 10);
                    // closed.push(*successor_index);
                    continue;
                }

                let is_in_closed = closed.iter().find(|idx| *idx == successor_index).is_some();

                if is_in_closed {
                    // println!("continuing: closed: {:?}", closed);
                    continue;
                }

                let is_in_open = open.iter().find(|idx| *idx == successor_index).is_some();

                let new_g = *g_vals.get(&index).unwrap_or(&0) + 1;
                let new_h = manhattan_dist(index2, *successor_index, self.width);
                let new_f = new_g + new_h;

                // println!("{} || {} < {}", !is_in_open, new_f, *f_vals.get(successor_index).unwrap_or(&u64::max_value()));
                if !is_in_open || new_f < *f_vals.get(successor_index).unwrap_or(&u64::max_value()) {
                    // This will likely produce duplicates. HashSet maybe better?
                    open.push(*successor_index);

                    f_vals.insert(*successor_index, new_f);
                    g_vals.insert(*successor_index, new_g);
                    h_vals.insert(*successor_index, new_h);
                    parents.insert(*successor_index, index);
                }
            }
        }

        let mut move_index = index2;
        let mut len = 0;
        let last_move = parents.get(&index2).map(|m| *m);
        // println!("parent of {} is {:?}", index2, parents.get(&index2));

        while let Some(&idx) = parents.get(&move_index) {
            move_index = idx;
            len += 1;
        }

        // println!("h_vals: {:?}", h_vals);

        // println!("open: {:?}", open);
        // println!("closed: {:?}", closed);
        // println!("parents: {:?}", parents);

        // If the len is 0 we were unable to figure out how to get there.
        // This usually means we're blocked in, so we should say it's
        // "impossible" (highest distance possible) This would probably be
        // more accurate to return a None
        if index != index2 && len == 0 {
            // println!("{} -> {} len 0", index, index2);
            len = u16::max_value();
            // assert_ne!(len, 0, "{} -> {}", index, index2);
        }

        // println!("next {}, last {:?}, len {}", index, last_move, len);

        AStarPath {
            next_move: index,
            last_move,
            len,
        }
    }

    fn execute_round(&mut self, elf_damage: u8) -> bool {
        let mut already_moved_npcs = HashSet::new();

        for i in 0..self.tiles.len() {
            let tile = self.tiles[i];
            let is_elf = match tile {
                Tile::Elf { .. } => true,
                Tile::Goblin { .. } => false,
                _ => continue,
            };

            if already_moved_npcs.contains(&i) {
                continue;
            }

            // println!("[Turn Start] {} at {}", if is_elf { "Elf" } else { "Goblin" }, i);

            // Action: Attack if enemy in range
            if let Some(enemy_index) = self.get_nearby_enemy(i, is_elf) {
                // println!("[Attack] {} -> {}", i, enemy_index);
                let enemy = &mut self.tiles[enemy_index];
                let damage_dealt = if is_elf {
                    elf_damage
                } else {
                    3
                };

                let is_dead = enemy.take_damage(damage_dealt);

                if is_dead {
                    let enemies = if is_elf {
                        &mut self.goblins
                    } else {
                        &mut self.elves
                    };

                    enemies.remove(&enemy_index);
                }

                already_moved_npcs.insert(i);

                continue;
            }

            // Action: Move
            let possible_moves = self.get_moves(i, true);

            // println!("npc idx {} possible moves: {:?}", i, possible_moves);

            let enemies = if is_elf {
                &self.goblins
            } else {
                &self.elves
            };

            if enemies.is_empty() {
                return false;
            }

            let mut path: Option<AStarPath> = None;

            for possible_move in possible_moves {
                assert_eq!(self.tiles[possible_move], Tile::Open, "Possible move is not open");

                for enemy_pos in enemies {
                    // println!("pre get path: {}, {}", possible_move, enemy_pos);
                    let new_path = self.get_shortest_path(possible_move, *enemy_pos);

                    // println!("potential path: {:?} current path: {:?}", new_path, path);

                    match path {
                        None => {
                            // Max length represents unreachable dest. This should ideally be
                            // an optional, but I'll leave that to a future cleanup.
                            if new_path.len != u16::max_value() {
                                path = Some(new_path);
                            }
                        },
                        Some(ref p) => {
                            let new_path_last_move = new_path.last_move.unwrap_or(usize::max_value());
                            let p_last_move = p.last_move.unwrap_or(usize::max_value());

                            // println!("new_path.len {} <= p.len {} && new_path.last_move {} < p.last_move {}", new_path.len, p.len, new_path_last_move, p_last_move);

                            if new_path.len < p.len {
                                path = Some(new_path);
                            } else if new_path.len == p.len && new_path_last_move < p_last_move {
                                path = Some(new_path);
                            }
                        },
                    }

                    // println!("decided path: {:?}", path);
                }
            }

            let mut i = i;

            if let Some(path) = path {
                let friends = if is_elf {
                    &mut self.elves
                } else {
                    &mut self.goblins
                };

                let tile = replace(&mut self.tiles[i], Tile::Open);

                friends.remove(&i);
                friends.insert(path.next_move);

                assert_eq!(self.tiles[path.next_move], Tile::Open);

                self.tiles[path.next_move] = tile;

                i = path.next_move;

                already_moved_npcs.insert(i);
                // println!("Final move: {:?}", path);
            }

            // Action: Attack if enemy in range
            if let Some(enemy_index) = self.get_nearby_enemy(i, is_elf) {
                // println!("[Attack2] {} -> {}", i, enemy_index);
                let enemy = &mut self.tiles[enemy_index];
                let is_dead = enemy.take_damage(3);

                if is_dead {
                    let enemies = if is_elf {
                        &mut self.goblins
                    } else {
                        &mut self.elves
                    };

                    enemies.remove(&enemy_index);
                }

                already_moved_npcs.insert(i);
            }
        }


        self.total_turns += 1;

        true
    }

    #[cfg(test)]
    fn print_all_npcs(&self) {
        for (i, tile) in self.tiles.iter().enumerate() {
            if tile.is_npc() {
                println!("{} at {}", if tile.is_elf() { "Elf" } else { "Goblin" }, i);
            }
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut string = String::with_capacity(self.tiles.len() + self.tiles.len() / self.width);

        // println!("string cap: {}", string.capacity());

        for (i, tile) in self.tiles.iter().enumerate() {
            string.push(tile.as_char());

            if i % self.width == self.width - 1 {
                string.push('\n')
            }
        }

        // println!("end cap: {}", string.capacity());

        f.write_str(&string)
    }
}


#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Map {
    let mut elves = HashSet::new();
    let mut goblins = HashSet::new();
    let width = input.chars().position(|c| c == '\n').unwrap();
    let tiles = input.chars().filter(|&c| c != '\n').enumerate().map(|(i, ch)| {
        match ch {
            '#' => Tile::Wall,
            'G' => {
                goblins.insert(i);

                Tile::Goblin { hp: 200 }
            },
            'E' => {
                elves.insert(i);

                Tile::Elf { hp: 200 }
            },
            '.' => Tile::Open,
            _ => unreachable!("Here"),
        }
    }).collect();

    Map {
        elves,
        goblins,
        tiles,
        total_turns: 0,
        width,
    }
}

#[aoc(day15, part1, Chars)]
pub fn part1_chars(map: &Map) -> u64 {
    let mut map = map.clone();

    println!("Starting map:\n{}", map);
    println!("Starting Elves: {:?}", map.elves);
    println!("Starting Goblins: {:?}", map.goblins);

    while map.execute_round(3) {
        // println!("turn {}:\n{}", map.total_turns, map);
    }

    println!("end map:\n{}", map);
    println!("end hps: {:?}", map.tiles.iter().filter(|tile| tile.is_npc()).map(|tile| tile.hp()).collect::<Vec<_>>());
    println!("end full turn: {}", map.total_turns);

    println!("Outcome: {} x {}", map.total_turns, map.sum_total_hp());

    map.total_turns as u64 * map.sum_total_hp()
}

#[aoc(day15, part2, Chars)]
pub fn part2_chars(map: &Map) -> u64 {
    let starting_elves = map.count_elves();

    for i in 4.. {
        let mut map = map.clone();

        while map.execute_round(i) {}

        if map.elves_won() && map.count_elves() == starting_elves {
            println!("{}", map);
            return map.total_turns as u64 * map.sum_total_hp();
        }
    }

    unreachable!("End of part2")
}

// https://www.reddit.com/r/adventofcode/comments/a70ohq/day_15_need_assistance_on_part_1/ec09zmg
// #[test]
// fn test_reddit1() {
//     let input = "##########################
// #######..#...###.........#
// #######......#G##GE.....##
// #######......#...E......##
// #######....E.........E#.##
// #####....................#
// #############............#
// ##########################";
//     let mut map = input_generator(input);

//     while map.execute_round() {}

//     println!("{}", map);

//     let elf1 = map.tiles[map.width + 18]; // 35
//     let elf2 = map.tiles[77];
//     let elf3 = map.tiles[79];
//     let elf4 = map.tiles[100];

//     assert_eq!(map.total_turns, 40);

//     map.print_all_npcs();

//     assert!(elf1.is_elf(), "{:?}", elf1);
//     assert_eq!(elf1.hp(), 200);

//     assert!(elf2.is_elf(), "{:?}", elf2);
//     assert_eq!(elf2.hp(), 200);

//     assert!(elf3.is_elf(), "{:?}", elf3);
//     assert_eq!(elf3.hp(), 83);

//     assert!(elf4.is_elf(), "{:?}", elf4);
//     assert_eq!(elf4.hp(), 128);

// }

#[test]
fn test_reddit2() {
    let input = "#######
#.E..G#
#.#####
#G#####
#######";
    let mut map = input_generator(input);

    map.execute_round(3);

    let elf1 = map.tiles[10];
    let goblin1 = map.tiles[11];
    let goblin2 = map.tiles[15];

    assert!(elf1.is_elf(), "{:?}", elf1);
    assert_eq!(elf1.hp(), 197);

    assert!(goblin1.is_goblin(), "{:?}", goblin1);
    assert_eq!(goblin1.hp(), 200);

    assert!(goblin2.is_goblin(), "{:?}", goblin2);
    assert_eq!(goblin2.hp(), 200);
}

#[test]
fn test_reddit3() {
    let input = "########
#..E..G#
#G######
########";
    let mut map = input_generator(input);

    map.execute_round(3);

    let elf1 = map.tiles[10];
    let goblin1 = map.tiles[9];
    let goblin2 = map.tiles[13];

    assert!(elf1.is_elf(), "{:?}", elf1);
    assert_eq!(elf1.hp(), 197);

    assert!(goblin1.is_goblin(), "{:?}", goblin1);
    assert_eq!(goblin1.hp(), 200);

    assert!(goblin2.is_goblin(), "{:?}", goblin2);
    assert_eq!(goblin2.hp(), 200);
}

#[test]
fn test_reddit4() {
    let input = "####
#GG#
#.E#
####";
    let mut map = input_generator(input);

    map.execute_round(3);

    let elf1 = map.tiles[10];
    let goblin1 = map.tiles[6];
    let goblin2 = map.tiles[9];

    assert!(elf1.is_elf(), "{:?}", elf1);
    assert_eq!(elf1.hp(), 194, "{:?}", elf1);

    assert!(goblin1.is_goblin(), "{:?}", goblin1);
    assert_eq!(goblin1.hp(), 197, "{:?}", goblin1);

    assert!(goblin2.is_goblin(), "{:?}", goblin2);
    assert_eq!(goblin2.hp(), 200, "{:?}", goblin2);
}

#[test]
fn test_reddit5() {
    let input = "#####
#..E#
#...#
#G..#
#####";
    let mut map = input_generator(input);

    map.execute_round(3);

    let elf1 = map.tiles[7];
    let goblin1 = map.tiles[11];

    assert!(elf1.is_elf(), "{:?}", elf1);
    assert_eq!(elf1.hp(), 200, "{:?}", elf1);

    assert!(goblin1.is_goblin(), "{:?}", goblin1);
    assert_eq!(goblin1.hp(), 200, "{:?}", goblin1);
}

#[test]
fn test_reddit6() {
    let input = "#####
##E##
#EGE#
##G##
#####";
    let mut map = input_generator(input);

    while map.execute_round(3) {}

    let elf1 = map.tiles[7];
    let elf2 = map.tiles[12];
    let elf3 = map.tiles[13];

    assert!(elf1.is_elf(), "{:?}", elf1);
    assert_eq!(elf1.hp(), 134, "{:?}", elf1);

    assert!(elf2.is_elf(), "{:?}", elf2);
    assert_eq!(elf2.hp(), 2, "{:?}", elf2);

    assert!(elf3.is_elf(), "{:?}", elf3);
    assert_eq!(elf3.hp(), 200, "{:?}", elf3);
}

#[test]
fn test_aoc() {
    let input = "#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";
    let mut map = input_generator(input);

    while map.execute_round(3) {}

    map.print_all_npcs();

    let goblin1 = map.tiles[8];
    let goblin2 = map.tiles[16];
    let goblin3 = map.tiles[26];
    let goblin4 = map.tiles[40];

    assert!(goblin1.is_goblin(), "{:?}", goblin1);
    assert_eq!(goblin1.hp(), 200, "{:?}", goblin1);

    assert!(goblin2.is_goblin(), "{:?}", goblin2);
    assert_eq!(goblin2.hp(), 131, "{:?}", goblin2);

    assert!(goblin3.is_goblin(), "{:?}", goblin3);
    assert_eq!(goblin3.hp(), 59, "{:?}", goblin3);

    assert!(goblin4.is_goblin(), "{:?}", goblin4);
    assert_eq!(goblin4.hp(), 200, "{:?}", goblin4);

    assert_eq!(map.total_turns, 47);
}

#[test]
fn test_aoc_p2() {
    let input = "#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";
    let mut map = input_generator(input);

    while map.execute_round(15) {}

    map.print_all_npcs();

    let elf1 = map.tiles[10];
    let elf2 = map.tiles[18];

    assert!(elf1.is_elf(), "{:?}", elf1);
    assert_eq!(elf1.hp(), 158, "{:?}", elf1);

    assert!(elf2.is_elf(), "{:?}", elf2);
    assert_eq!(elf2.hp(), 14, "{:?}", elf2);

    assert_eq!(map.total_turns, 29);
    assert_eq!(map.sum_total_hp(), 172);
}

// #[test]
// fn test_aoc2() {
//     let input = "#######
// #G..#E#
// #E#E.E#
// #G.##.#
// #...#E#
// #...E.#
// #######";
//     let mut map = input_generator(input);

//     while map.execute_round() {}

//     map.print_all_npcs();

//     let elf1 = map.tiles[12];
//     let elf2 = map.tiles[15];
//     let elf3 = map.tiles[23];
//     let elf4 = map.tiles[29];
//     let elf5 = map.tiles[33];

//     assert!(elf1.is_elf(), "{:?}", elf1);
//     assert_eq!(elf1.hp(), 200, "{:?}", elf1);

//     assert!(elf2.is_elf(), "{:?}", elf2);
//     assert_eq!(elf2.hp(), 197, "{:?}", elf2);

//     assert!(elf3.is_elf(), "{:?}", elf3);
//     assert_eq!(elf3.hp(), 185, "{:?}", elf3);

//     assert!(elf4.is_elf(), "{:?}", elf4);
//     assert_eq!(elf4.hp(), 200, "{:?}", elf4);

//     assert!(elf5.is_elf(), "{:?}", elf5);
//     assert_eq!(elf5.hp(), 200, "{:?}", elf5);

//     assert_eq!(map.total_turns, 37);
// }

#[test]
fn test_reddit_p2() {
    let input = "################################
#...############################
###G.###########################
##.....#########################
#......#########################
##G...G.########################
#G.....G########################
###...G#########################
###....#########################
######.G.#######################
#######....#####################
###..#.....GG...G.E...##########
##........G...#####...##.#######
#.G..........#######...#..######
#...####G...#########......#####
#..G##.#..G.#########.......####
#...##....E.#########...E.....##
#...##......#########G......####
#...........#########.......####
#............#######...........#
#.....E..G...E#####E...........#
#.G...........G.............E###
#...............E#####.#..######
#..#..G...........####...#######
#..#..............######.#######
####.#...E.......###############
########..##...#################
##...##..###..##################
#.......########################
##...E..########################
###......#######################
################################";
    let mut map = input_generator(input);

    while map.execute_round(25) {
        println!("Turn {}:\n{}", map.total_turns, map);

        if map.total_turns == 20 {
            break;
        }
    }

    let elf1 = map.tiles[7];
    let elf2 = map.tiles[12];
    let elf3 = map.tiles[13];

    assert!(elf1.is_elf(), "{:?}", elf1);
    assert_eq!(elf1.hp(), 134, "{:?}", elf1);

    assert!(elf2.is_elf(), "{:?}", elf2);
    assert_eq!(elf2.hp(), 2, "{:?}", elf2);

    assert!(elf3.is_elf(), "{:?}", elf3);
    assert_eq!(elf3.hp(), 200, "{:?}", elf3);
}

#[test]
fn test_damage() {
    let input = "\
####
##E#
#GG#
####";
    let mut map = input_generator(input);

    while map.execute_round(3) {
        println!("Turn {}:\n{}", map.total_turns, map);

        // if map.total_turns == 20 {
        //     break;
        // }
    }

    assert_eq!(map.total_turns, 71);

    println!("End: {}", map);

    let goblin1 = map.tiles[9];
    let elf2 = map.tiles[12];
    let elf3 = map.tiles[13];

    assert!(goblin1.is_goblin(), "{:?}", goblin1);
    assert_eq!(goblin1.hp(), 3, "{:?}", goblin1);

    assert!(elf2.is_elf(), "{:?}", elf2);
    assert_eq!(elf2.hp(), 2, "{:?}", elf2);

    assert!(elf3.is_elf(), "{:?}", elf3);
    assert_eq!(elf3.hp(), 200, "{:?}", elf3);
}
