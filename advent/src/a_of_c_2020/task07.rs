use std::collections::HashSet;

use crate::task::Task;
use crate::util::read_file;

pub struct Task07A {}

pub struct Task07B {}

impl Task for Task07A {
  fn run(&self) {
    let input = read_file("./res/2020/task07.txt");
    let rules = parse_rules(&input);
    let parents = check_rules_for_parents(&rules, "shiny gold");
    println!("Number of allowed parents: {}", parents);
  }
}

impl Task for Task07B {
  fn run(&self) {
    let input = read_file("./res/2020/task07.txt");
    let rules = parse_rules(&input);
    let count = check_rules_for_children(&rules, "shiny gold");
    println!("Number of bags inside: {}", count);
  }
}

#[derive(Debug)]
struct Rule {
  id: usize,
  name: String,
  children: Vec<Child>,
  parent: Vec<usize>,
}

#[derive(Debug)]
struct Bag {
  name: String,
  children: Vec<Child>,
}

#[derive(Debug, Clone)]
struct Child {
  count: usize,
  name: String,
}

fn parse_rules(input: &str) -> Vec<Rule> {
  let mut rules: Vec<Rule> = vec![];
  for line in input.lines() {
    let bag = parse_line(line);
    let rule_id = get_rule_id_for_name(&mut rules, &bag.name);
    update_rule(rule_id, &mut rules, &bag.children);
    for child in bag.children {
      let child_rule_id = get_rule_id_for_name(&mut rules, &child.name);
      let child_rule = rules.get_mut(child_rule_id).unwrap();
      child_rule.parent.push(rule_id);
    }
  }
  rules
}

fn update_rule(rule_id: usize, rules: &mut Vec<Rule>, children: &Vec<Child>) {
  let rule = rules.get_mut(rule_id).unwrap();
  if rule.children.len() != children.len() {
    rule.children.clear();
    for child in children {
      rule.children.push(child.clone());
    }
  }
}

fn get_rule_id(rules: &Vec<Rule>, name: &str) -> Option<usize> {
  for rule in rules {
    if rule.name == name {
      return Some(rule.id);
    }
  }
  None
}

fn get_rule_id_for_name(rules: &mut Vec<Rule>, name: &str) -> usize {
  match get_rule_id(rules, name) {
    Some(id) => id,
    None => {
      let rule = Rule {
        name: name.to_owned(),
        parent: vec![],
        id: rules.len(),
        children: vec![],
      };
      rules.push(rule);
      rules.len() - 1
    }
  }
}

fn parse_line(input: &str) -> Bag {
  let items = input.split(" ").collect::<Vec<&str>>();
  let main_bag = items[0].to_owned() + " " + items[1];
  let mut children = vec![];

  let mut i = 4;
  while i < items.len() {
    let num = items[i];
    if num == "no" {
      break;
    }
    let count = num.parse::<usize>().unwrap();
    i += 1;
    let name = items[i].to_owned() + " " + items[i + 1];
    i += 3;
    children.push(Child { count, name });
  }
  Bag {
    name: main_bag,
    children,
  }
}

fn check_rules_for_children(rules: &Vec<Rule>, name: &str) -> usize {
  let start_rule_id = get_rule_id(rules, name).unwrap();
  let start_rule = &rules[start_rule_id];
  check_rules_for_children_inner(start_rule, rules)
}

fn check_rules_for_children_inner(rule: &Rule, rules: &Vec<Rule>) -> usize {
  let mut count = 0;
  for child in &rule.children {
    count += child.count;
    let child_rule_id = get_rule_id(&rules, &child.name).unwrap();
    count += child.count * check_rules_for_children_inner(&rules[child_rule_id], rules);
  }
  count
}

fn check_rules_for_parents(rules: &Vec<Rule>, name: &str) -> usize {
  let mut found_colors = HashSet::new();
  let start_rule_id = get_rule_id(rules, name).unwrap();
  let start_rule = &rules[start_rule_id];
  for parent in &start_rule.parent {
    found_colors.insert(*parent);
  }

  loop {
    let start_num = found_colors.len();
    let new_parents = find_new_parents(rules, &found_colors);
    for new_parent in new_parents {
      found_colors.insert(new_parent);
    }
    let end_num = found_colors.len();
    if end_num == start_num {
      break;
    }
  }
  found_colors.len()
}

fn find_new_parents(rules: &Vec<Rule>, current_parents: &HashSet<usize>) -> HashSet<usize> {
  let mut result = HashSet::new();
  for current_parent in current_parents.iter() {
    let current_parent_rule = &rules[*current_parent];
    for parent in &current_parent_rule.parent {
      result.insert(*parent);
    }
  }
  result
}

#[cfg(test)]
mod test {
  use crate::a_of_c_2020::task07::{check_rules_for_children, check_rules_for_parents, parse_line, parse_rules};

  #[test]
  fn test_parse() {
    let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
    let bag = parse_line(input);
    assert_eq!(bag.name, "light red");
    assert_eq!(bag.children.len(), 2);
    assert_eq!(bag.children[0].name, "bright white");
    assert_eq!(bag.children[0].count, 1);
    assert_eq!(bag.children[1].name, "muted yellow");
    assert_eq!(bag.children[1].count, 2);
  }

  #[test]
  fn test_parse2() {
    let input = "bright white bags contain 1 shiny gold bag.";
    let bag = parse_line(input);
    assert_eq!(bag.name, "bright white");
    assert_eq!(bag.children.len(), 1);
    assert_eq!(bag.children[0].name, "shiny gold");
    assert_eq!(bag.children[0].count, 1);
  }

  #[test]
  fn test_parse3() {
    let input = "faded blue bags contain no other bags.";
    let bag = parse_line(input);
    assert_eq!(bag.name, "faded blue");
    assert_eq!(bag.children.len(), 0);
  }

  #[test]
  fn parse_rules_test() {
    let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
    let r = parse_rules(input);
    assert_eq!(r.len(), 9);
    dbg!(&r);
    assert_eq!(r[2].parent.len(), 2)
  }

  #[test]
  fn check_rules_for_parents_test() {
    let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    let rules = parse_rules(input);
    let num_colors = check_rules_for_parents(&rules, "shiny gold");
    assert_eq!(num_colors, 4);
  }

  #[test]
  fn check_rules_for_children_test() {
    let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    let rules = parse_rules(input);
    let num_colors = check_rules_for_children(&rules, "shiny gold");
    assert_eq!(num_colors, 32);
  }
}