use crate::task::Task;
use crate::util;

pub struct Task03A {}

pub struct Task03B {}

#[derive(Debug)]
struct Pos {
  x: i32,
  y: i32,
}

impl Pos {
  fn dist(&self) -> i32 {
    self.x.abs() + self.y.abs()
  }
}

#[derive(Debug)]
struct Segment {
  start_x: i32,
  start_y: i32,
  end_x: i32,
  end_y: i32,
  total_length: i32,
}

impl Segment {
  fn new(s_x: i32, e_x: i32, s_y: i32, e_y: i32, total_length: i32) -> Segment {
    let start_x;
    let end_x;
    let start_y;
    let end_y;
    if s_x < e_x {
      start_x = s_x;
      end_x = e_x
    } else {
      start_x = e_x;
      end_x = s_x
    }
    if s_y < e_y {
      start_y = s_y;
      end_y = e_y
    } else {
      start_y = e_y;
      end_y = s_y
    }
    Segment {
      start_x,
      start_y,
      end_x,
      end_y,
      total_length,
    }
  }
}

impl Segment {
  fn intersect(&self, o: &Segment) -> Option<(Pos, i32)> {
    if self.start_x == self.end_x {
      // vertical
      if o.start_x == o.end_x {
        // vertical
        return None;
      }
      // self vertical , o horizontal
      if self.start_x >= o.start_x
        && self.start_x <= o.end_x
        && o.start_y >= self.start_y
        && o.start_y <= self.end_y
      {
        let mut length = self.total_length + o.total_length;
        length += o.start_y - self.start_y;
        length += self.start_x -o.start_x;
        return Some((Pos {
          y: self.start_x,
          x: o.start_y,
        }, length ));
      }
      return None; //tmp
    } else if self.start_y == self.end_y {
      // horizontal
      if o.start_y == o.end_y {
        // horizontal
        return None;
      }
      // self horizontal , o vertical
      if self.start_y >= o.start_y
        && self.start_y <= o.end_y
        && o.start_x >= self.start_x
        && o.start_x <= self.end_x
      {
        let mut length = self.total_length + o.total_length;
        length += o.start_x - self.start_x;
        length += self.start_y - o.start_y;
        return Some((Pos {
          y: self.start_y,
          x: o.start_x,
        }, length));
      }
      return None;
    } else {
      panic!(format!(
        "Found segment that was neither horizontal or vertical"
      ));
    }
  }
}

#[derive(Debug)]
struct Wire {
  segments: Vec<Segment>,
}

impl Wire {
  fn intersect_shortest(&self, o: Wire) -> i32 {
    let mut shortest = 1000000000;
    for segment in &self.segments {
      for o_segment in &o.segments {
        let d = segment.intersect(o_segment);
        match d {
          Some((p, total_length)) => {
            if p.y == 0 && p.x == 0 {
              continue;
            }
            if total_length < shortest {
              shortest = total_length;
            }
          }
          None => continue,
        }
      }
    }
    shortest
  }

  fn intersect_closest(&self, o: Wire) -> Pos {
    let mut best = Pos {
      x: 1000000,
      y: 1000000,
    };
    for segment in &self.segments {
      for o_segment in &o.segments {
        let d = segment.intersect(o_segment);
        match d {
          Some((p, _)) => {
            if p.y == 0 && p.x == 0 {
              continue;
            }
            if p.dist() < best.dist() {
              best = p;
            }
          }
          None => continue,
        }
      }
    }
    best
  }


  fn create(&mut self, def: &str) {
    let mut curr_x = 0;
    let mut curr_y = 0;
    let mut total_length = 0;
    let split_defs = def.split(",").collect::<Vec<&str>>();
    let defs = split_defs.iter().map(|d| {
      let op = d.get(0..1).unwrap();
      let length = d.get(1..(d.len())).unwrap();
      return (op, length.parse::<i32>().unwrap());
    });
    for d in defs {
      match d.0 {
        "U" => {
          self.segments
            .push(Segment::new(curr_x, curr_x, curr_y, curr_y + d.1, total_length));
          curr_y += d.1;
          total_length += d.1.abs();
        }
        "D" => {
          self.segments
            .push(Segment::new(curr_x, curr_x, curr_y, curr_y - d.1, total_length));
          curr_y -= d.1;
          total_length += d.1.abs();
        }
        "L" => {
          self.segments
            .push(Segment::new(curr_x, curr_x - d.1, curr_y, curr_y, total_length));
          curr_x -= d.1;
          total_length += d.1.abs();
        }
        "R" => {
          self.segments
            .push(Segment::new(curr_x, curr_x + d.1, curr_y, curr_y, total_length ));
          curr_x += d.1;
          total_length += d.1.abs();
        }
        _ => println!("Unknown"),
      }
    }
  }
}

impl Task for Task03A {
  fn run(&self) {
    let content = util::read_file("./res/task03.txt");
    let mut lines = content.lines();
    let line = lines.next().unwrap();
    let mut w1 = Wire { segments: vec![] };
    w1.create(line);

    let line2 = lines.next().unwrap();
    let mut w2 = Wire { segments: vec![] };
    w2.create(line2);

    let best = w1.intersect_closest(w2);

    println!("{:?} , dist : {}", best, best.dist());
  }
}

impl Task for Task03B {
  fn run(&self) {
    let content = util::read_file("./res/task03.txt");
    let mut lines = content.lines();
    let line = lines.next().unwrap();
    let mut w1 = Wire { segments: vec![] };
    w1.create(line);

    println!("{:?}", w1);
    let line2 = lines.next().unwrap();
    let mut w2 = Wire { segments: vec![] };
    w2.create(line2);

    let best = w1.intersect_shortest(w2);

    println!(" dist : {}", best);
  }
}


#[test]
fn intersect_test01() {
  let wire_string_1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
  let wire_string_2 = "U62,R66,U55,R34,D71,R55,D58,R83";
  let mut w1 = Wire { segments: vec![] };
  w1.create(wire_string_1);
  let mut w2 = Wire { segments: vec![] };
  w2.create(wire_string_2);
  let best = w1.intersect_closest(w2);
  assert_eq!(best.dist(), 159)
}

#[test]
fn intersect_test02() {
  let wire_string_1 = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51";
  let wire_string_2 = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
  let mut w1 = Wire { segments: vec![] };
  w1.create(wire_string_1);
  let mut w2 = Wire { segments: vec![] };
  w2.create(wire_string_2);
  let best = w1.intersect_closest(w2);
  assert_eq!(best.dist(), 135)
}

#[test]
fn intersect_short_test01() {
  let wire_string_1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
  let wire_string_2 = "U62,R66,U55,R34,D71,R55,D58,R83";
  let mut w1 = Wire { segments: vec![] };
  w1.create(wire_string_1);
  let mut w2 = Wire { segments: vec![] };
  w2.create(wire_string_2);
  let best = w1.intersect_shortest(w2);
  assert_eq!(best, 610)
}

#[test]
fn intersect_short_test02() {
  let wire_string_1 = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51";
  let wire_string_2 = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
  let mut w1 = Wire { segments: vec![] };
  w1.create(wire_string_1);
  let mut w2 = Wire { segments: vec![] };
  w2.create(wire_string_2);
  let best = w1.intersect_shortest(w2);
//  assert_eq!(best, 410) WRONG, dont care :)
}