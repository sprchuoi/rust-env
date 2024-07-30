// block and scope
fn main(){
    let y = 3 ; 
    let z = {
        let x = 10; 
        println!("y = {}",y );
        x-y
    };
    println!{"z = {}" , z};
}