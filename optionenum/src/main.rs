//Option 열거형
//흔하게 사용되고 유용함.
//값이 있을 수도 있고 없을 수도 있는 경우. ->버그방지에 유용
//러스트에서는 null특성이 존재하지 않음.
//Option열거형은 값의 존재, 부재의 개념을 표현하는 열거형임

// enum Option<T> {
//     Some(T),
//     None,
// }
//Option<t>열거형은 기본적으로 포함이기 때문에 명시적으로
//가져오지않아도 사용할 수 있음.
//Option<T>의 variants도 Option::을 붙이지 않고 바로 사용가능

fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;
    //None을 사용하면 Option<T>가 어떤 타입을 가질지 러스트에게 알려줄 필요가 있음.
}
