fn main() {
    let _x:i32 = 42;
    let y:i32 = 43;
    let pair:(char, i32) = ('a', 14); //pair.0, pair.1

    println!("Hello world {} !",y);
    println!("Pair.0={}, Pair.1={}", pair.0, pair.1);

    let x = vec![1,1].iter().map(|m| m + 5).fold(0, |x,y| x + y);

    println!("{}",x);

    match y {
        1 => println!("it's 1"),
        2 => println!("it's 2"),
        42 => println!("it's 42"),
        _ => println!("Other")
    }

    for text in (1..200).step_by(2) {
        print!("{}", text)
    } 

}
