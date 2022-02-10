//구조체 정의
#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

//메소드 정의
//impl (implementation : 구현)
impl Rectangle {
    fn area(&self) -> u32 {
        //함수 시그니처 내에서 &Rectangle이 사용된 것처럼
        // &self로 써야함.
        // 메소드는 self의 소유권을 가져갈 수도, 여기서처럼 빌릴수도
        //다른 파라미터와 비슷하게 변경가능하도록 빌릴 수도 있음.
        self.length * self.width
    }
}

fn main() {
    let rect1 = Rectangle {
        length: 50,
        width: 30,
    };
    println!(
        "The area of the REctangle is {} square pixel.",
        rect1.area()
    );
}
