enum Message {
    Quit,
    Move { x: i32, y: i32 }, //익명 구조체
    Write(String),
    ChangeColor(i32, i32, i32),
}

//열거형과 구조체는 impl을 사용해서 메소드를 정의할 수 있다는 공동첨이 있음
impl Message {
    fn call(&self) {}
}
//이러한 열거형은 아래와 같다고 할 수 있음
struct QuitMessage; //유닛구조체
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); //튜플구조체
struct ChangeColorMessage(i32, i32, i32); //튜플 구조체

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
