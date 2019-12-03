use crate::task::Task;
use crate::util;

pub struct Task03A {}

pub struct Task03B {}

#[derive(Debug)]
struct Pos {
  x : i32,
  y : i32
}

impl Pos {
  fn dist(&self) -> i32{
    self.x.abs() + self.y.abs()
  }
}

#[derive(Debug)]
struct Segment {
  start_x: i32,
  start_y: i32,
  end_x: i32,
  end_y: i32,
}

impl Segment {
  fn intersect(&self, o :Segment) -> Pos{

  }
}

#[derive(Debug)]
struct Wire {
  segments: Vec<Segment>
}

impl Wire {
  fn intersect(&self, o : Wire) -> Pos{
    let mut best = Pos  {x:1000000,y:1000000};
    for segment in self.segments {
      for o_segment in o.segments {
        let d = segment.intersect(o_segment);
        if d.dist() < best.dist() {
          best = d;
        }
      }
    }
    best
  }

  fn create(&mut self, def: &str) {
    let mut curr_x = 0;
    let mut curr_y = 0;
    let split_defs = def.split(",").collect::<Vec<&str>>();
    let defs = split_defs.iter()
      .map(|d| {
        let op = d.get(0..1).unwrap();
        let length = d.get(1..(d.len())).unwrap();
        return (op, length.parse::<i32>().unwrap());
      });
    for d in defs {

      match d.0 {
        "U" => {
          self.segments.push(Segment {
            start_x: curr_x,
            start_y: curr_y,
            end_x: curr_x,
            end_y: curr_y + d.1,
          });
          curr_y += d.1;
        }
        "D" => {
          self.segments.push(Segment {
            start_x: curr_x,
            start_y: curr_y,
            end_x: curr_x,
            end_y: curr_y - d.1,
          });
          curr_y -= d.1;
        }
        "L" => {
          self.segments.push(Segment {
            start_x: curr_x,
            start_y: curr_y,
            end_x: curr_x - d.1,
            end_y: curr_y,
          });
          curr_x -= d.1;
        }
        "R" => {
          self.segments.push(Segment {
            start_x: curr_x,
            start_y: curr_y,
            end_x: curr_x + d.1,
            end_y: curr_y,
          });
          curr_x += d.1;
        }
        _ => println!("Unknown")
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

    let best = w1.intersect(w2);

    println!("{:?}", best);
  }
}

impl Task for Task03B {
  fn run(&self) {
    unimplemented!()
  }
}
