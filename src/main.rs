fn main() {
    // If odd: x * 3 + 1, always: x / 2
    let mut x: i32 = get_valid_input();
    while x != 1 {
        if x % 2 != 0 {
            x = x * 3 + 1;
            println!("{}", x);
        }
        x /= 2;
        println!("{}", x);
    }
}
fn get_valid_input() -> i32 {
    use std::io::stdin;
    println!("Input a number");
    let mut input: String = String::new();
    stdin().read_line(&mut input).expect("Taking input failed");
    match input.replace("\n", "").parse::<i32>() {
        Ok(x) => x,
        Err(e) => {
            println!("Error parsing input: {}", e);
            // This could theoretically lead to a stack overflow, but realistically it is unlikely
            get_valid_input()
        }
    }
}
