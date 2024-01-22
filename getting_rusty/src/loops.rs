pub fn run () {
    let mut num = 0;

    // while num < 101 {
    //     if num % 3 == 0 && num % 5 == 0{
    //         println!("FizzBuzz");
    //     } else if num % 5 == 0 {
    //         println!("Buzz");
    //     }
    //     else if num % 3 == 0 {
    //         println!("Fizz");
    //     } else {
    //         println!("{}", num);
    //     }
    //     num += 1;
    // }

//     for i in 1..101 {
//         if i % 3 == 0 && i % 5 == 0{
//             println!("FizzBuzz");
//         } else if i % 5 == 0 {
//             println!("Buzz");
//         } else if i % 3 == 0 {
//             println!("Fizz");
//         } else {
//             println!("{}", i);
//         }
//     }
// }

loop {
    num += 1;
    println!("{}", num);

    if num == 100 {
        break;
    }
}

}