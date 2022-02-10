// 슬라이스(slices)
// 소유권을 갖지 않는 또다른 데이터 타입(소유권을 갖지 않는 것 : 참조자 타입)
// 슬라이스는 컬렉션 전체가 아닌 컬렉션의 연속된 일련의 요소를 참조

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    // 문자열을 바이트 배열로 변환
    for (i, &item) in bytes.iter().enumerate() {
        // iter메소드를 이용해서 바이트 배열의 반복자를 생성
        //iter : 컬렉션의 각 요소를 반환하는 함수
        // enumerate : iter의 결과값을 직접 반환하는 대신 이를 감사서 튜플의 일부로 만들어 반환
        //반환된 튜플의 첫번째 요소는 인덱스, 두번째 요소는 요소에 대한 참조값
        if item == b' ' {
            //바이트리터럴 문법
            //공백문자이면 그 문자의 인덱스를 반환한다는 뜻
            return i;
        }
    }
    //문자열이 한 단어로 구성되어있을 때는 문자열의 길이 반환
    return s.len();
}
//슬라이스를 이용한 첫번째 단어 추출 함수
fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}

fn first_word3(s: &str) -> &str {
    //String은 동적 타입
    //str은 리터럴
    //first_word2와 달리 리터럴과 슬라이스 둘 다 사용 가능
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}

fn main() {
    let mut s = String::from("hello World!");

    let word = first_word(&s);

    s.clear();
    //String을 비워서 ''로 만듦
    //word는 여전히 5를 가지고 있지만, 5라는 값을 의미있게 쓸 수 있는 문자열이 존재하지 않음
    //따라서 word는 이제 유효하지 않음.

    //이러한 문제를 해결하기 위해서 스트링슬라이스를 사용
    let s2 = String::from("hello World!");

    let hello = &s[0..5]; //혹은 [..5]
    let world = &s[6..11];
}
