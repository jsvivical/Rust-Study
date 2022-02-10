//구조체
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

//변수명과 구조체의 필드명이 같을 때, 필드 초기화 축약법 이용
fn build_user(email: String, username: String) -> User {
    return User {
        email,
        username, //매개변수랑 이름이 같아 명시하지 않고 초기화 가능
        active: true,
        sign_in_count: 1,
    };
}

fn main() {
    // 구조체 인스턴스 생성
    let user1 = User {
        email: String::from("jsvivical@gmail.com"),
        username: String::from("js"),
        active: true,
        sign_in_count: 1,
    };
    println!(
        "email : {}\nusername: {}\nactive : {}\nsign_in_count : {}",
        user1.email, user1.username, user1.active, user1.sign_in_count
    );
}
