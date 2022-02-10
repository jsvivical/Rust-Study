fn main() {
    let x = 5;
    //x는 불변성임

    let x = x + 1;
    //x의 값에 x + 1의 값을 대입하는 것이 아니라
    //새로 생성한 x변수가 이전의 x값을 shadowing함

    let x = x * 2;
    println!("The value of x is : {}", x);

    //mut와 shadowing의 차이는 let키워드를 다시 사용하여 효과적으로 새로운
    //값을 할당하고, 값의 유형을 변경하면서
    //동일 이름을 사용할 수 있음.__rust_force_expr!

    let spaces = "    ";
    let spaces = spaces.len();
    println!("number of spaces is {}", spaces);

    let mut spaces2 = "    ";
    //spaces = spaces.len();
    //가변변수이기 때문에 같은 유형의 값이 아니면 할당할 수 없음
}
