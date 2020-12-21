use crate::task::Task;
use crate::util::read_file;

pub struct Task06A {}

pub struct Task06B {}

impl Task for Task06A {
  fn run(&self) {
    let input = read_file("./res/2019/task06.txt");
    let map = OrbitMap::new(input);
    let orbits = map.count_orbits();
    println!("Number of orbits: {}", orbits);
  }
}

impl Task for Task06B {
  fn run(&self) {
    let input = read_file("./res/2019/task06.txt");
    let map = OrbitMap::new(input);
    let you_index = map.find_orbit_index_by_name("YOU".to_string());
    let san_index = map.find_orbit_index_by_name("SAN".to_string());
    let mut you_parents = map.find_parents_index_for_orbit_index(you_index);
    let mut san_parents = map.find_parents_index_for_orbit_index(san_index);

    you_parents.reverse();
    san_parents.reverse();
    let (you_index, san_index) = find_common_parent(&you_parents, &san_parents);
    println!("Orbit changed required : {}", you_index+san_index);
  }
}

fn find_common_parent(you_parents: &Vec<usize>, san_parents: &Vec<usize>) -> (usize, usize) {
  for you_index in 0..you_parents.len() {
    for san_index in 0..san_parents.len() {
      let you_parent = you_parents[you_index];
      let san_parent = san_parents[san_index];
      if you_parent == san_parent {
        return (you_index, san_index);
      }
    }
  }
  return (0, 0);
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
    return 1 + self.find_orbits_count(&self.orbits[orbit.parent_index as usize]);
  }

  fn find_orbit_index_by_name(&self, name: String) -> usize {
    for (index, orbit) in self.orbits.iter().enumerate() {
      if orbit.name == name {
        return index;
      }
    }
    0
  }

  fn find_parents_index_for_orbit_index(&self, orbit_index: usize) -> Vec<usize> {
    let orbit = &self.orbits[orbit_index];
    if orbit.parent_index == -1 {
      return vec![];
    }
    let mut parents = self.find_parents_index_for_orbit_index(orbit.parent_index as usize);
    parents.push(orbit.parent_index as usize);
    return parents;
  }
}


fn find_index_for_parent(orbits: &Vec<Orbit>, parent: &str) -> i32 {
  let mut index = 0;
  for orbit in orbits {
    if orbit.name == parent {
      return index;
    }
    index += 1;
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
}

#[test]
fn test_02() {
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
K)L
K)YOU
I)SAN".to_string();

  let map = OrbitMap::new(def);

  let you_index = map.find_orbit_index_by_name("YOU".to_string());
  let san_index = map.find_orbit_index_by_name("SAN".to_string());
  let mut you_parents = map.find_parents_index_for_orbit_index(you_index);
  let mut san_parents = map.find_parents_index_for_orbit_index(san_index);

  you_parents.reverse();
  san_parents.reverse();
  let (you_index, san_index) = find_common_parent(&you_parents, &san_parents);

  assert_eq!(you_index+san_index,4)

}