struct User {
    username : String,
    email : String,
    sign_in_count : u64,
    active : bool,
}

fn build_user(username : String, email : String, sign_in_count : u64, active : bool) ->User{
    return User{
        username,
        email,
        sign_in_count,
        active,
    };
}

fn main() {
    let mut user1 = build_user(String::from("Jinsol"), 
        String::from("jsvivical@gmail.com"), 1, true);
    println!("{}\n{}\n{}\n{}",user1.username, user1.email, user1.sign_in_count, user1.active);
    
    //User인스턴스의 email필드 변경하기
    //우선 인스턴스가 mutable해야함
    user1.email = String::from("astrocmder@gmail.com");
    println!("{}\n{}\n{}\n{}",user1.username, user1.email, user1.sign_in_count, user1.active);
    
    //구조체 갱신법을 이용하여 기존 구조체 인스턴스로 새 구조체 인스턴스 생성하기
    let mut user2 = User{
        email:String::from("jsvivical@gmail.com"),
        ..user1
    };
    user2.active = false;
    println!("{}\n{}\n{}\n{}",
    user2.username, user2.email, user2.sign_in_count, user2.active);
}
