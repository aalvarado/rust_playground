struct Person {
    name: String,
    age: int
}

fn main() {
    println!("hello?");
    let p = Person { name: "Foo".to_string(), age: 20 };
    println!("{:s}", p.name);
    println!("{:d}", p.age);
}
