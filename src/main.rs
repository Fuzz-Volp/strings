#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    // String Slices 
    let cat = "Fluffy";
    let cat: &'static str = "Fluffy";
    println!("{}", cat);
    // string slices are immutable

    // String Objects 
    let dog = String::new();
    let mut dog = String::from("Remi");
    println!("{}", dog);

    // Format Macro
    let owner = format!("Hi I'm {} the owner of {}", "Fuzzy", dog);
    println!("{}", owner);

    // len "Length"
    println!("{}", dog.len());

    // push & push_str Functions
    dog.push(' ');
    println!("{}", dog);
    dog.push_str("the dog");
    println!("{}", dog);

    // replace
    let new_dog = dog.replace("the", "is my");
    println!("{}", new_dog);

}
