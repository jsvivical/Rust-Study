fn main() {
    let x = 5;
    //기본 변수는 불변성이기 때문에 값의 변경이 불가
    println!("The value of x is : {}", x);

    //x = 6;
    //불변성이기 때문에 에러발생
    //println!("The value of x is : {}", x);
    
    let mut y = 5;
    println!("The value of x is {} ",y);
    y = 6;
    println!("The value of x is : {}", y);
    
}
