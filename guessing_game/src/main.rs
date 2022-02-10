use std::io;
//러스트는 모든 프로그램의 스코프에 prelude내의 타입들을 가져옴
//원하는 타입이 prelude 타입 내에 없다면 use문은 활용해서 명시적으로 가져옴.
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the Number!");
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("The secret Number is : {}", secret_number);

    println!("Please input your guess");
    //느낌표가 붇은 것을 러스트 매크로라고 불림.
    //느낌표가 없으면 함수라고 부름
    loop{
        let mut guess = String::new();
        //let은 변수를 선언하는 키워드
        //mut은 가변변수를 만드는 키워드 
        //러스트에서는 변수는 기본적으로 불변임

        io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
        //표준 입력을 통해 들어온 문자열을 guess에 저장
        //expect는 입력 오류가 발생하면 메시지를 출력하는 함수 예상
        //문제가 발생했을 때 프로그램이 멈추길 바란다면 expect메소드
        //expect()메소드는 io::Result타입의 내장메소드임
        let guess: u32 = guess.trim().parse()
        .expect("Please type a number");
        //같은 이름의 변수 guess를 shadow를 허락
        //trim()메소드는 양 옆의 공백, 개행문자 등을 없앰.
        //parse()는 문자열을 숫자형으로 파싱

        println!("you guessed : {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You Win");
                break;
            }
        }
    }
}