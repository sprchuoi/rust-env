struct Point(i32 , i32);
struct PoundofForces(f64);
struct Newton(f64);
fn compute_thruster_force() -> PoundofForces{
    todo!("Ask a rocket scentist at NASA");
}
fn set_thruster_force()(force : &Newton){
    
}
fn main() {
    let p = Point(17 , 32);
    println!("{} {}" , p.0 , p.1);
    let force = compute_thruster_force(); 
        
}
