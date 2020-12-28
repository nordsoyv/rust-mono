use crate::task::Task;
use crate::util::read_file;

pub struct Task04A {}

pub struct Task04B {}

impl Task for Task04A {
  fn run(&self) {
    let content = read_file("./res/2020/task04.txt");
    let sum = parse_passports(&content)
      .iter()
      .map(|p| match p.is_valid_simple() {
        true => 1,
        false => 0
      }).fold(0, |acc, n| acc + n);
    println!("Valid passports {}", sum);
  }
}

impl Task for Task04B {
  fn run(&self) {
    let content = read_file("./res/2020/task04.txt");
    let sum = parse_passports(&content)
      .iter()
      .map(|p| match p.is_valid() {
        true => 1,
        false => 0
      }).fold(0, |acc, n| acc + n);
    println!("Valid passports {}", sum);
  }
}

#[derive(Debug)]
struct Passport {
  byr: Option<String>,
  iyr: Option<String>,
  eyr: Option<String>,
  hgt: Option<String>,
  hcl: Option<String>,
  ecl: Option<String>,
  pid: Option<String>,
  cid: Option<String>,
}

impl Passport {
  pub fn new() -> Passport {
    Passport {
      byr: None,
      cid: None,
      ecl: None,
      eyr: None,
      hcl: None,
      hgt: None,
      iyr: None,
      pid: None,
    }
  }

  pub fn is_valid(&self) -> bool {
    self.check_pid()
      && self.check_ecl()
      && self.check_eyr()
      && self.check_byr()
      && self.check_hcl()
      && self.check_hgt()
      && self.check_iyr()
  }

  pub fn is_valid_simple(&self) -> bool {
    if self.ecl.is_none() {
      return false;
    }
    if self.pid.is_none() {
      return false;
    }
    if self.iyr.is_none() {
      return false;
    }
    if self.hgt.is_none() {
      return false;
    }
    if self.hcl.is_none() {
      return false;
    }
    if self.eyr.is_none() {
      return false;
    }
    if self.byr.is_none() {
      return false;
    }
    return true;
  }


  fn check_ecl(&self) -> bool {
    match &self.ecl {
      Some(val) => match val.as_str() {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
        _ => false
      },
      None => false
    }
  }

  fn check_byr(&self) -> bool {
    self.check_year(&self.byr, 1920, 2002)
  }

  fn check_iyr(&self) -> bool {
    self.check_year(&self.iyr, 2010, 2020)
  }

  fn check_eyr(&self) -> bool {
    self.check_year(&self.eyr, 2020, 2030)
  }

  fn check_pid(&self) -> bool {
    match &self.pid {
      Some(val) => {
        if val.len() != 9 {
          return false;
        }
        return val.chars().all(|c| c.is_digit(10));
      }
      None => false
    }
  }

  fn check_hcl(&self) -> bool {
    match &self.hcl {
      None => false,
      Some(val) => {
        if val.len() != 7 {
          return false;
        }
        if val.chars().nth(0).unwrap() != '#' {
          return false;
        }
        return val.chars().skip(1).all(|c| c.is_digit(16));
      }
    }
  }

  fn check_hgt(&self) -> bool {
    return match &self.hgt {
      None => false,
      Some(val) => {
        let height = val
          .chars()
          .take_while(|c| c.is_digit(10))
          .collect::<String>()
          .parse::<i32>()
          .unwrap();

        if val.ends_with("cm") {
          if height > 149 && height < 194 {
            return true;
          }
        }
        if val.ends_with("in") {
          if height > 58 && height < 77 {
            return true;
          }
        }
        false
      }
    };
  }

  fn check_year(&self, val: &Option<String>, min_valid: i32, max_valid: i32) -> bool {
    return match val {
      Some(val) => {
        if val.len() != 4 {
          return false;
        }
        let result = val.parse::<i32>();
        if result.is_err() {
          false
        } else {
          let year = result.unwrap();
          if year < min_valid || year > max_valid {
            return false;
          }
          true
        }
      }
      None => false
    };
  }
}


fn parse_passports(batch: &str) -> Vec<Passport> {
  let mut result = vec![];
  let mut current_passport = Passport::new();
  let lines = batch.lines();
  for line in lines {
    if line == "" {
      result.push(current_passport);
      current_passport = Passport::new();
      continue;
    }
    let line_parts = line.split(" ").collect::<Vec<&str>>();
    for line_part in line_parts {
      let field = line_part.split(':').collect::<Vec<&str>>();
      match field[0] {
        "ecl" => current_passport.ecl = Some(field[1].to_string()),
        "pid" => current_passport.pid = Some(field[1].to_string()),
        "eyr" => current_passport.eyr = Some(field[1].to_string()),
        "hcl" => current_passport.hcl = Some(field[1].to_string()),
        "byr" => current_passport.byr = Some(field[1].to_string()),
        "iyr" => current_passport.iyr = Some(field[1].to_string()),
        "cid" => current_passport.cid = Some(field[1].to_string()),
        "hgt" => current_passport.hgt = Some(field[1].to_string()),
        _ => panic!("Unknown passport field found {}", field[1])
      }
    }
  }
  result.push(current_passport);
  result
}

#[cfg(test)]
mod test {
  use crate::a_of_c_2020::task04::parse_passports;

  #[test]
  fn test_parse_passport() {
    let passport = parse_passports(TEST_DATA);
    for pass in passport {
      dbg!(pass.is_valid_simple());
    }
  }

  #[test]
  fn test_valid_data() {
    let passport = parse_passports(TEST_DATA2);
    for pass in passport {
      assert_eq!(pass.is_valid(), false);
    }

    let passport = parse_passports(TEST_DATA3);
    for pass in passport {
      assert_eq!(pass.is_valid(), true);
    }
  }


  static TEST_DATA: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";


  static TEST_DATA2: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

  static TEST_DATA3: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
}

