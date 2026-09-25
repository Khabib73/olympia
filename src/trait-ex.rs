trait Greet {
    fn greet(&self) -> String;
}

struct Cat;

impl Greet for Cat {
    fn greet(&self) -> String {
        "meow".into()
    }
}

struct Dog;

impl Greet for Dog {
    fn greet(&self) -> String {
        "woof".into()
    }
}

fn say_hello(animal: impl Greet) {
    println!("{}", animal.greet());
}


fn main() {
    let cat = Cat;
    let dog = Dog;
    say_hello(cat);
    say_hello(dog);
}
