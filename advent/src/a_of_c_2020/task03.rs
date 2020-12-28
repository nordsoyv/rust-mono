use crate::task::Task;
use crate::util::read_file;

pub struct Task03A {}
pub struct Task03B {}

impl Task for Task03A {
  fn run(&self) {
    let content = read_file("./res/2020/task03.txt");
    let map = Map::parse(&content);
    let hits = map.get_hits_for_slope(3, 1);
    println!("Hits for slope ({},{}) = {}", 3, 1, hits);
  }
}


impl Task for Task03B {
  fn run(&self) {
    let content = read_file("./res/2020/task03.txt");
    let map = Map::parse(&content);
    let hits1 = map.get_hits_for_slope(1, 1);
    let hits2 = map.get_hits_for_slope(3, 1);
    let hits3 = map.get_hits_for_slope(5, 1);
    let hits4 = map.get_hits_for_slope(7, 1);
    let hits5 = map.get_hits_for_slope(1, 2);
    println!("Product of all hits : {}", hits1 * hits2 * hits3 * hits4 * hits5);
  }
}

struct Map {
  lines: Vec<MapLine>
}

#[derive(Debug)]
struct MapLine {
  line: Vec<u32>,
}

impl MapLine {
  pub fn parse(l: &str) -> MapLine {
    let line = l.chars().map(|c| match c {
      '#' => 1,
      _ => 0
    }).collect::<Vec<u32>>();
    MapLine {
      line
    }
  }

  pub fn get(&self, pos: usize) -> u32 {
    let len = self.line.len();
    let real_pos = pos % len;
    self.line[real_pos]
  }
}

impl Map {
  pub fn parse(lines: &str) -> Map {
    let map_lines = lines
      .lines()
      .map(|l| MapLine::parse(l))
      .collect::<Vec<MapLine>>();
    Map {
      lines: map_lines
    }
  }

  pub fn get_hits_for_slope(&self, x_diff: usize, y_diff: usize) -> u64 {
    let mut x_pos = 0;
    let mut y_pos = 0;
    let mut hits: u64 = 0;
    loop {
      let line = &self.lines[y_pos];
      hits += line.get(x_pos) as u64;
      x_pos += x_diff;
      y_pos += y_diff;
      if y_pos >= self.lines.len() {
        break;
      }
    }
    hits
  }
}

#[test]
fn test_map_line() {
  let line = MapLine::parse("..#.#.#...#");
  assert_eq!(line.line[0], 0);
  assert_eq!(line.line[1], 0);
  assert_eq!(line.line[2], 1);
  assert_eq!(line.line[3], 0);
  assert_eq!(line.line[4], 1);
  assert_eq!(line.line[5], 0);
  assert_eq!(line.line[6], 1);
  assert_eq!(line.line[7], 0);
  assert_eq!(line.line[8], 0);
  assert_eq!(line.line[9], 0);
  assert_eq!(line.line[10], 1);


  assert_eq!(line.get(0), 0);
  assert_eq!(line.get(2), 1);
  assert_eq!(line.get(4), 1);
  assert_eq!(line.get(13), 1);
  assert_eq!(line.get(14), 0);
}

#[test]
fn test_map() {
  let test = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
  let _map = Map::parse(test);
}

#[test]
fn test_map2() {
  let test = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
  let map = Map::parse(test);
  assert_eq!(map.get_hits_for_slope(1, 1), 2);
  assert_eq!(map.get_hits_for_slope(3, 1), 7);
  assert_eq!(map.get_hits_for_slope(5, 1), 3);
  assert_eq!(map.get_hits_for_slope(7, 1), 4);
  assert_eq!(map.get_hits_for_slope(1, 2), 2);
}

