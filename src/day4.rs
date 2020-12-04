#[derive(Debug, Default)]
pub struct Passport {
    /// (Birth Year)
    byr: Option<String>,
    /// (Issue Year)
    iyr: Option<String>,
    /// (Expiration Year)
    eyr: Option<String>,
    /// (Height)
    hgt: Option<String>,
    /// (Hair Color)
    hcl: Option<String>,
    /// (Eye Color)
    ecl: Option<String>,
    /// (Passport ID)
    pid: Option<String>,
    /// (Country ID)
    cid: Option<String>,
}

impl Passport {
    fn is_valid(&self) -> bool {
        self.byr.is_some() && self.iyr.is_some() && self.eyr.is_some() && self.hgt.is_some() &&
        self.hcl.is_some() && self.ecl.is_some() && self.pid.is_some()
    }

    fn is_valid_ext(&self) -> bool {
        if !self.is_valid() {
            return false;
        }

        let byr = self.byr.as_ref().unwrap().parse::<u16>().unwrap();

        if byr < 1920 || byr > 2002 {
            return false;
        }

        let iyr = self.iyr.as_ref().unwrap().parse::<u16>().unwrap();

        if iyr < 2010 || iyr > 2020 {
            return false;
        }

        let eyr = self.eyr.as_ref().unwrap().parse::<u16>().unwrap();

        if eyr < 2020 || eyr > 2030 {
            return false;
        }

        let hgt = self.hgt.as_ref().unwrap();

        if !hgt.ends_with("in") && !hgt.ends_with("cm") {
            return false;
        }

        let hgt_num = hgt[..hgt.len()-2].parse::<u16>().unwrap();

        if hgt.ends_with("in") && (hgt_num < 59 || hgt_num > 76) {
            return false;
        }

        if hgt.ends_with("cm") && (hgt_num < 150 || hgt_num > 193) {
            return false;
        }

        let hcl = self.hcl.as_ref().unwrap();

        if hcl.len() != 7 || !hcl.starts_with('#') || u32::from_str_radix(&hcl[1..], 16).is_err() {
            return false;
        }

        let ecl = self.ecl.as_ref().unwrap();

        if !matches!(&**ecl, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth") {
            return false;
        }

        let pid = self.pid.as_ref().unwrap();

        if pid.len() != 9 || pid.parse::<u32>().is_err() {
            return false;
        }

        true
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Passport> {
    let lines = input.split('\n');

    let mut passports = Vec::new();
    let mut current_passport = None;

    for line in lines {
        if line.is_empty() {
            passports.push(current_passport.take().unwrap());
            continue;
        }

        let mut passport = current_passport.take().unwrap_or_else(Passport::default);

        let key_val_pairs = line.split(' ');

        for key_val in key_val_pairs {
            let mut split = key_val.split(':');
            let key = split.next().unwrap();
            let val = Some(split.next().unwrap().to_string());

            match key {
                "byr" => passport.byr = val,
                "iyr" => passport.iyr = val,
                "eyr" => passport.eyr = val,
                "hgt" => passport.hgt = val,
                "hcl" => passport.hcl = val,
                "ecl" => passport.ecl = val,
                "pid" => passport.pid = val,
                "cid" => passport.cid = val,
                _ => unreachable!(),
            }
        }

        current_passport = Some(passport);
    }

    if let Some(passport) = current_passport.take() {
        passports.push(passport);
    }

    passports
}

#[aoc(day4, part1)]
pub fn part1(passports: &[Passport]) -> usize {
    passports.iter().filter(|p| p.is_valid()).count()
}

#[aoc(day4, part2)]
pub fn part2(passports: &[Passport]) -> usize {
    passports.iter().filter(|p| p.is_valid_ext()).count()
}
