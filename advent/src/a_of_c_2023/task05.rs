use std::{ops::Range, usize::MAX};
use rayon::prelude::*;
use crate::{task::Task, util::read_file};

pub struct Task05A {}
pub struct Task05B {}

impl Task for Task05A {
  fn run(&self) {
    let input = read_file("./res/2023/task05a.txt");
    let almanac = Almanac::parse_taska(input);
    let min = almanac
      .seeds
      .iter()
      .map(|seed_id| almanac.map_seed_to_location(*seed_id))
      .collect::<Vec<usize>>()
      .into_iter()
      .min()
      .unwrap();
    println!("The result is {}", min);
  }
}

impl Task for Task05B {
  fn run(&self) {
    let input = read_file("./res/2023/task05a.txt");
    let almanac = Almanac::parse_taskb(input);
    let min = almanac
      .seed_ranges
      .par_iter()
      .map(|range| {
        let mut min = MAX;
        for seed_id in range.clone() {
            let loc = almanac.map_seed_to_location(seed_id);
            if loc < min {
                min = loc
            }
        }   
        min
      })
      .collect::<Vec<usize>>()
      .into_iter()
      .min()
      .unwrap();
    println!("The result is {}", min);

  }
}

#[derive(Debug)]
struct Almanac {
  seeds: Vec<usize>,
  seed_ranges: Vec<Range<usize>>,
  id_maps: Vec<IdMap>,
}

impl Almanac {
  fn map_seed_to_location(&self, seed_id: usize) -> usize {
    let mut mapped_id = seed_id;
    for current_map in &self.id_maps {
      mapped_id = current_map.map_id(mapped_id);
    }
    return mapped_id;
  }

    

  fn parse_taska(input: String) -> Almanac {
    let mut current_map = IdMap::new();
    let mut alamanac = Almanac {
      id_maps: vec![],
      seeds: vec![],
      seed_ranges: vec![],
    };
    for line in input.lines() {
      if line.starts_with("seeds:") {
        alamanac.seeds = line.split(":").collect::<Vec<&str>>()[1]
          .split(" ")
          .collect::<Vec<&str>>()
          .into_iter()
          .map(|p| p.trim())
          .filter(|p| p.len() > 0)
          .map(|p| p.parse::<usize>().unwrap())
          .collect::<Vec<usize>>();
        continue;
      }
      if line.is_empty() {
        alamanac.id_maps.push(current_map);
        current_map = IdMap::new();
        continue;
      }
      if line.contains(":") {
        current_map.name = line.split(" ").collect::<Vec<&str>>()[0].to_string();
        continue;
      }
      current_map.add_range(line);
    }
    alamanac.id_maps.push(current_map);
    alamanac.id_maps.remove(0);
    alamanac
  }

  fn parse_taskb(input: String) -> Almanac {
    let mut current_map = IdMap::new();
    let mut alamanac = Almanac {
      id_maps: vec![],
      seeds: vec![],
      seed_ranges: vec![],
    };
    for line in input.lines() {
      if line.starts_with("seeds:") {
        let seed_ids = line.split(":").collect::<Vec<&str>>()[1]
          .split(" ")
          .collect::<Vec<&str>>()
          .into_iter()
          .map(|p| p.trim())
          .filter(|p| p.len() > 0)
          .map(|p| p.parse::<usize>().unwrap())
          .collect::<Vec<usize>>();
        for index in 0..seed_ids.len() {
          if index % 2 != 0 {
            continue;
          }
          let start = seed_ids[index];
          let span = seed_ids[index+1];
          alamanac.seed_ranges.push(start..start+span);
        }
        continue;
      }
      if line.is_empty() {
        alamanac.id_maps.push(current_map);
        current_map = IdMap::new();
        continue;
      }
      if line.contains(":") {
        current_map.name = line.split(" ").collect::<Vec<&str>>()[0].to_string();
        continue;
      }
      current_map.add_range(line);
    }
    alamanac.id_maps.push(current_map);
    alamanac.id_maps.remove(0);
    alamanac
  }
}

#[derive(Debug)]
struct IdMap {
  name: String,
  source_ranges: Vec<Range<usize>>,
  dest_ranges: Vec<Range<usize>>,
}

impl IdMap {
  fn new() -> IdMap {
    IdMap {
      name: String::new(),
      source_ranges: vec![],
      dest_ranges: vec![],
    }
  }
  fn add_range(&mut self, input: &str) {
    let parts = input
      .split(" ")
      .collect::<Vec<&str>>()
      .into_iter()
      .map(|p| p.trim())
      .map(|p| p.parse::<usize>().unwrap())
      .collect::<Vec<usize>>();
    let dest_start = parts[0];
    let source_start = parts[1];
    let span = parts[2];
    self.dest_ranges.push(dest_start..dest_start + span);
    self.source_ranges.push(source_start..source_start + span);
  }

  fn map_id(&self, id_to_map: usize) -> usize {
    for (index, source_range) in self.source_ranges.iter().enumerate() {
      if source_range.contains(&id_to_map) {
        let dest_range = &self.dest_ranges[index];
        let offset = id_to_map - source_range.start;
        return dest_range.start + offset;
      }
    }
    id_to_map
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn task_a_example() {
    let input = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    let almanac = Almanac::parse_taska(input.to_string());
    //dbg!(&almanac);
    assert_eq!(81, almanac.id_maps[0].map_id(79));
    assert_eq!(14, almanac.id_maps[0].map_id(14));
    assert_eq!(57, almanac.id_maps[0].map_id(55));
    assert_eq!(13, almanac.id_maps[0].map_id(13));

    assert_eq!(82, almanac.map_seed_to_location(79));

    assert_eq!(43, almanac.map_seed_to_location(14));
    assert_eq!(86, almanac.map_seed_to_location(55));
    assert_eq!(35, almanac.map_seed_to_location(13));

    let min = almanac
      .seeds
      .iter()
      .map(|seed_id| almanac.map_seed_to_location(*seed_id))
      .collect::<Vec<usize>>()
      .into_iter()
      .min()
      .unwrap();
    assert_eq!(35, min);
  }

  #[test]
  fn task_b_example() {
    let input = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    let almanac = Almanac::parse_taskb(input.to_string());
    let min = almanac
      .seed_ranges
      .iter()
      .map(|range| {
        let mut min = MAX;
        for seed_id in range.clone() {
            let loc = almanac.map_seed_to_location(seed_id);
            if loc < min {
                min = loc
            }
        }   
        min
      })
      .collect::<Vec<usize>>()
      .into_iter()
      .min()
      .unwrap();
    println!("The result is {}", min);

  }


}
