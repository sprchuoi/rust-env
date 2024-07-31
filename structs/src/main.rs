struct Person{
    name : String,
    age : u8,
}
fn describe(person : &Person){
    println!("{} is {} years old" , person.name ,  person.age);
}
fn main() {
    let mut peter = Person{name : String::from("Peter") , age : 18};
    describe(&peter);
    peter.age = 18;
    describe(&peter);
    let name = String::from("Peter 1");
    let age = 30;
    let peter1 = Person{name , age};
    describe(&peter1);
}
