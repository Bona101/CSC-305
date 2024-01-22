pub fn run(){
    let mut numbers: Vec<i32> = vec![1,3];

    numbers.push(9);

    println!("{:?}", numbers);

    for x in numbers.iter_mut() {
        *x *= 2;
        println!("{}", x);
    }

}