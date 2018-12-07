use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref FORMAT: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
}

const Y_HEIGHT: usize = 1000;

#[derive(Debug)]
pub struct Claim {
    id: u16,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
}

impl Claim {
    fn draw_on(&self, fabric: &mut [Option<FabricType>]) {
        let x2 = self.x + self.width;
        let y2 = self.y + self.height;

        for x in self.x..x2 {
            for y in self.y..y2 {
                let tile = &mut fabric[y as usize * Y_HEIGHT + x as usize];
                let tile_taken = tile.take();

                *tile = match tile_taken {
                    None => Some(FabricType::Claimed { id: self.id }),
                    Some(FabricType::Claimed { id }) =>
                        Some(FabricType::ClaimedMultiple { ids: vec![id, self.id] }),
                    Some(FabricType::ClaimedMultiple { mut ids }) => {
                        ids.push(self.id);

                        Some(FabricType::ClaimedMultiple { ids })
                    }
                }
            }
        }
    }

    fn has_overlap(&self, fabric: &[Option<FabricType>]) -> bool {
        let x2 = self.x + self.width;
        let y2 = self.y + self.height;

        for x in self.x..x2 {
            for y in self.y..y2 {
                let tile = &fabric[y as usize * Y_HEIGHT + x as usize];

                if let Some(FabricType::ClaimedMultiple { .. }) = tile {
                    return true;
                }
            }
        }

        false
    }
}

// This is sort of like a SmallVec
#[derive(Clone, Debug)]
enum FabricType {
    Claimed { id: u16, },
    ClaimedMultiple { ids: Vec<u16>, },
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Claim> {
    FORMAT.captures_iter(input).map(|cap| {
        Claim {
            id: cap[1].parse::<u16>().unwrap(),
            x: cap[2].parse::<u16>().unwrap(),
            y: cap[3].parse::<u16>().unwrap(),
            width: cap[4].parse::<u16>().unwrap(),
            height: cap[5].parse::<u16>().unwrap(),
        }
    }).collect()
}

#[aoc(day3, part1, Chars)]
pub fn part1_chars(claims: &[Claim]) -> usize {
    let mut fabric: Vec<Option<FabricType>> = vec![None; 1_000_000];

    for claim in claims {
        claim.draw_on(&mut fabric);
    }

    fabric.iter().filter(|ft| {
        match ft {
            Some(FabricType::ClaimedMultiple { .. }) => true,
            _ => false,
        }
    }).count()
}

#[aoc(day3, part2, Chars)]
pub fn part2_chars(claims: &[Claim]) -> u16 {
    let mut fabric: Vec<Option<FabricType>> = vec![None; 1_000_000];

    for claim in claims {
        claim.draw_on(&mut fabric);
    }

    for claim in claims {
        if !claim.has_overlap(&fabric) {
            return claim.id;
        }
    }

    unreachable!()
}
