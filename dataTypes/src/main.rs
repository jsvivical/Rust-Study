fn main() {
    //타입은 크게 스칼라와 컴파운드 두 가지임
    //Rust는 타입이 고정된 언어
    //즉, 모든 편수의 타입이 컴파일 시에 반드시 정해져 있어야 함.


    //스칼라 타입들
    //스칼라 : 하나의 값으로 표현되는 타입

    //스칼라 - 정수형
    //signed
    let x : i8 = 1;
    let y : i16 = 1;
    let z : i32 = 1;
    let a : i64 = 1;
    let b : isize = 1; //컴퓨터 환경에 따라 다름
    //unsinged
    let x : u8 = 1;
    let y : u16 = 1;
    let z : u32 = 1;
    let a : u64 = 1;
    let b : usize = 1;
    //일반적으로 가장 빠른 i32를 많이 사용

    //스칼라-부동소수점 타입
    let x : f32 = 2.0;
    let y : f64 = 3.0;
    //기본은 f64dla

    //스칼라-불린타입
    let t : bool = true;
    let f : bool = false;

    //스칼라 - 문자타입
    //문자 타입은 작은 따옴표, 문자열은 큰 따옴표
    let c : char = 'z';
    let d : char = 'd';

    //복합타입, 컴파운드 타입
    //복합타입-튜플
    //다양한 타입의 몇 개의 숫자를 
    //집합시켜 하나의 복합타입으로 만드는 일반적인 방법

    let tup : (i32,f64,u8) = (500, 6.4, 1);
    //튜플의 내용을 언패킹하기(구조해체)
    let (x,y,z) = tup;
    //구조해체의 다른 방법( . 연산자 활용)
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    //복합타입 - 배열
    let a  = [1,2,3,4,5];
    //배열은 고정된 숫자의 요소를 가짐
    //벡터 타입은 표준 라이브러리에서 제공됨

    let months = ["January", "Febrary", "March"];

    //배열 요소에 접근 
    let a = [1,2,3,4,5];
    let first = a[0];
    let second = a[1];








}
