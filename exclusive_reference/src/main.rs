fn main() {
    let mut a = (1,2);
    let x_coord = &mut a.0;
    *x_coord = 20;
    println!("a = {a:?}");
}
