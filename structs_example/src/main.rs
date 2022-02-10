//구조체를 활용한 예제 프로그램
//넓이 구하기
#[derive(Debug)]
//러스트는 디버깅 정보를 출력하는 기능을 포함하지만
//사용자가 정의한 구조체에 대하여 해당 기능을 활성화하도록
//사전 동의를 해주어야 함.
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let length1 = 50;
    let width1 = 30;

    println!(
        "The area of the rectangle is {} square pixels.",
        area1(length1, width1)
    );

    //튜플을 이용한 리팩터링
    let rect1 = (50, 30);

    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect1)
    );

    //구조체를 이용한 리팩터링
    let rect2 = Rectangle {
        length: 50,
        width: 30,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect2)
    );

    //파생트레잇으로 유용한 기능 추가하기
    //println!()에서 {}안에 :?명시자를 넣는 것은 println!에게 Debug라
    //불리우는 출력 포맷을 사용하고 싶다고 말하는 것
    println!("rect2 is {:?}", rect2);
    //혹은
    println!("rect2 is {:#?}", rect2);
    //아래 함수에서는 요소별로 다른 줄에 표시됨
}

fn area1(length: u32, width: u32) -> u32 {
    return length * width;
}

fn area2(dimensions: (u32, u32)) -> u32 {
    return dimensions.0 * dimensions.1;
}

fn area3(rectangle: &Rectangle) -> u32 {
    return rectangle.length * rectangle.width;
}
