

fn foo (_x: &'static str) -> &'static str {
    "World"

}

fn main() {
    let world = "World";

    println!("Hello World!");
    println!("Hello {}!", world);
    println!("Hello {}!", foo("bar"));
}