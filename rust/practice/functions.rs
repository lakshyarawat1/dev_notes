fn main() {
    printing_something_here();
    function_with_arguments("John Doe");
}

fn printing_something_here () {
    println!("Function working good !")
}

fn function_with_arguments(name: &str){
    println!("Printing using arguments, name is  {name} ")
}