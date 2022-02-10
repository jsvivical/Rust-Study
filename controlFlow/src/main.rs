fn main() {
    let number = 3;

    if number > 5 {
        println!("condition was false");
    } else if number == 3 {
        println!("the number is 3");
    } else {
        println!("condition was true");
    }

    //let구문에서 if 사용하기
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is : {}", number);

    let mut i = 0;

    //반복문
    //loop
    loop {
        println!("{}", i);
        i = i + 1;
        if i == 5 {
            break;
        }
    }
    i = 0;
    //while
    while number < 5 {
        println!("{}", i);
        i = i + 1;
    }
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("{}", element);
    }

    for number in (1..5).rev() {
        println!("{}", number);
    }

    for character in "The Twelve days of Christmas".chars() {
        println!("{}", character);
    }
}
