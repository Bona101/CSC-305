pub fn run() {
    greet("a", "n");
    let water = add(2, 7);

    let boo = 9;

    println!("{}", water);

    let mini_func = |a: i32, b: i32| a + b + boo;

    println!("5 + 4 = {}", mini_func(5, 4));
}

fn greet(s: &str, t: &str){
    println!("{} {} ok", s, t);
}

fn add (n: i32, m: i32) -> i32 {
    n + m
}