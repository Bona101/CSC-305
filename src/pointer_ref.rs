pub fn run (){
    let vec1 = vec![1, 2, 3];
    // let vec2 = vec1.clone();
    let vec2 = &vec1;

    println!("{:?} {:?}", &vec1, vec2);
}