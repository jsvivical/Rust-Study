fn main(){
    let s = String::from("hello");
    let mut str2 = String::from("hello");
    str2.push_str(", World");
    println!("{}", str2);
    

} //여기서 String이 요구한 메모리를 운영체제에게 반납(drop함수)