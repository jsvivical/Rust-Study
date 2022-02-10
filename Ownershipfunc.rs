fn main(){
    let s = String::from("hello");
    //s가 스코프 안으로 들어옴
    takes_ownership(s); //s의 값이 함수 안으로 이동
    //s가 더이상 유효하지 않음
    let x = 5; 
    //x가 스코프 안으로 들어옴
    makes_copy(x); //x가 함수 안으로 이동했지만,
    //i32는 Copy되므로, x를 계속 사용 가능
    //println!("{}", s); //오류발생
    println!("{}", x);

}

fn takes_ownership(some_string : String) { //some_string이 스코프 안으로 들오옴
    println!("{}", some_string);
}//some_string이 스코프 밖으로 벗어나서 메모리 해제

fn makes_copy(some_integer : i32) {
    println!("{}", some_integer);
} //some_integer는 스코프 밖을 벗어났지만 별 다른 일은 없음