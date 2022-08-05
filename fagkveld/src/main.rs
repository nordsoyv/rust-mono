// fn main() {
//   let nums = vec![1, 2, 3];
//   let mut double = vec![];
//   let mut triple = vec![];
//   for num in nums {
//     double.push(num * 2);
//   }
//   println!("{:?}", double);
//   for num in nums {
//     triple.push(num * 3);
//   }
//   println!("{:?}", triple);
// }

trait Animal {
  fn speak(&self);
}

struct Dog {}
impl Animal for Dog {
  fn speak(&self) {
    println!("Woof")
  }
}

struct Cat {}
impl Animal for Cat {
  fn speak(&self) {
    println!("Meow")
  }
}

// will generate a concrete implementation of this function for each type that it is called with
fn animal_say_generic<T: Animal>(animal: T) {
  animal.speak();
}

// will use dynamic dispatch and a virtual table to find the correct function to call at runtime
fn animal_say_dyn(animal: Box<dyn Animal>) {
  animal.speak();
}

fn main() {
  let dog = Dog {};
  animal_say_generic(dog);
  let cat = Cat {};
  animal_say_generic(cat);

  let dog_dyn = Box::new(Dog {});
  let cat_dyn = Box::new(Cat {});
  animal_say_dyn(dog_dyn);
  animal_say_dyn(cat_dyn);
}
