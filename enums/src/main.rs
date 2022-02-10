//열거험(enum)
//열거형의 예시
//IP주소는 두 개의 표준이 존재함(버전4, 버전6)
//IP주소는 둘 중하나로 둘 다가 될 수 없음.
//그러나 두 버전 다 IP주소이기 때문에, 동일한 타입으로 처리되는 것이 좋음

enum IpAddrKind {
    V4,
    V6,
} //이 안의 것들을 열거형의 Variants라고함
  //이제 ipAddrKind는 코드 어디에서나 쓸 수 있는 데이터 타입

//구조체와 같이 활용되면 더 자세한 정보를 담을 수 있음
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
//열거형 Variant에 데이터를 직접 넣는 방식을 사용하면
//더 간결하게 동일한 개념을 사용가능
enum IpAddr2 {
    V4(String),
    V6(String),
}

//V4의 경우 4개의 u8타입으로 저장되길 원하면
enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home2 = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback2 = IpAddr2::V6(String::from("::1"));

    let home3 = IpAddr3::V4(127, 0, 0, 1);
    let loopback = IpAddr3::V6(String::from("::1"));
}

//IpAddrKind 타입을 인자로 받는 함수
fn route(ip_type: IpAddrKind) {}
