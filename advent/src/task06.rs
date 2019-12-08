use crate::task::Task;
use crate::util::read_file;

pub struct Task06A {}

pub struct Task06B {}

impl Task for Task06A {
  fn run(&self) {
    let input = read_file("./res/task06.txt");
      let map = OrbitMap::new(input);
      let orbits = map.count_orbits();
      println!("Number of orbits: {}", orbits);
  }
}

impl Task for Task06B {
  fn run(&self) {
    unimplemented!()
  }
}

#[derive(Debug, Clone)]
struct Orbit {
  parent_index: i32,
  parent: String,
  name: String,
}

#[derive(Debug, Clone)]
struct OrbitMap {
  orbits: Vec<Orbit>,
}

impl OrbitMap {
  fn new(s: String) -> OrbitMap {
    let lines = s.split("\n").collect::<Vec<&str>>();
    let org_orbits: Vec<Orbit> = lines.iter().map(|l| {
      let parts = l.split(")").collect::<Vec<&str>>();

      Orbit {
        name: parts[1].to_string(),
        parent: parts[0].to_string(),
        parent_index: -2,
      }
    }).collect::<Vec<Orbit>>();

    let mut orbits = org_orbits.clone();

    for o in &mut orbits {
      let parent_index = find_index_for_parent(&org_orbits, &o.parent);
      o.parent_index = parent_index;
    }

    OrbitMap {
      orbits
    }
  }

  fn count_orbits(&self) -> i32 {
    let mut total_orbits = 0;
    for orbit in &self.orbits {
      total_orbits += self.find_orbits_count(orbit);
    }

    total_orbits
  }

  fn find_orbits_count(&self, orbit: &Orbit) -> i32 {
    if orbit.parent_index == -1 {
      return 1;
    }
    return 1+ self.find_orbits_count(&self.orbits[orbit.parent_index as usize]);
  }
}


fn find_index_for_parent(orbits: &Vec<Orbit>, parent: &str) -> i32 {
  let mut index = 0;

  for orbit in orbits {
    if orbit.name == parent {
      return index;
    }
      index +=1;
  }
  return -1;
}


#[test]
fn test_01() {
  let def = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L".to_string();

  let o = OrbitMap::new(def);
    let total_orbits = o.count_orbits();
    assert_eq!(total_orbits, 42);
  //println!("{:?}", o);
  //println!("{}", o.count_orbits());
}