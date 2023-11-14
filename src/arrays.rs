pub fn run () {
    use std::mem::size_of_val;

    let nums = [1, 2, 3, 4, 5];


    println!("{:?}", (nums[0], nums[1], nums[2]));

    println!("{}", size_of_val(&nums));

    let amp: &[i32] = &nums[0..3];

    println!("{:?}", amp);
}