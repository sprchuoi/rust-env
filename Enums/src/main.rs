struct Foo {
    x : (u32 , u32),
    y : u32,
}
fn main() {
    let foo = Foo{x:(1,2), y :3};
    match foo {
        Foo{x : (1 , b) , y }    => println!("x.0 = 1 , b = {b} , y = {y}", ),
        Foo{y:2 , x: i}         => println!("y =2 , x = {i:?} " ),
        Foo{y , .. }            => println!("y = {y }, Other feild is ignore" ),
    }

}
