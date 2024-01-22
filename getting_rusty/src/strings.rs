// pub fn run () -> String {
//     let message: String = String::from("9y");
//     message
// }

// pub fn run2 () -> String {
//     let msg: String = String::from("ok");
//     return msg;
// }


pub fn run () {
    let mut hello = String::from(" h ell o");

    hello.push('u');

    hello.push_str("we ");

    println!("{}", hello.contains("we"));

    for a in hello.split_whitespace() {
        print!("{}", a);
    }

    assert_eq!(2, 1+1);

    print!("{}{}", hello.len(), hello);

    print!("p");

    let mut s = String::with_capacity(10);
    s.push('a');
    s.push_str("adka");

    println!("{}", s);
}