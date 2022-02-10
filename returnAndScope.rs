fn main() {
    let s1 = gives_ownership();
    //gives-ownership은 반환값을 s1에게 이동
    let s2 = String::from("hello");
    //s2가 스코프 안에 들어옴
    let s3 = takes_and_gives_back(s2);
    //s2는 takes_and_gives_back 안으로 이동하고
    //이 함수가 반환값을 s3로 이동시킴
    println!("{}\ns2는 무효화 됨\n{}", s1,s3);

} //s1, s3만 유효함으로 drop함수가 호출됨

fn gives_ownership() -> String{
    let some_string  = String::from("hello");
    return some_string;
}

fn takes_and_gives_back(a_string:String) ->String{
    return a_string;
}