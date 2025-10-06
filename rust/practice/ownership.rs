fn main(){
    let s = String::from("Hello, Rust!");

    ownership_example(s);

}

fn ownership_example(s: String){
    println!("Inside ownership_example");
}