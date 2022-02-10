fn main() {
    println!("Hello, World!");
    another_function();
    another_function2(30);
    another_function3(32, 15);
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is {}", y);
}

fn another_function() {
    println!("Another function.");
}

fn another_function2(x: i32) {
    println!("The value of x is {}", x);
}

fn another_function3(x: i32, y: i32) {
    println!("The value of x is {}, {}", x, y);
}
fn plus_one(x: i32) -> i32 {
    //세미콜론을 넣으면 구문이 되어 반환하지 못하게 됨.
    return x + 1;
    //C:\Users\jsviv\OneDrive\바탕 화면\vendor\git-for-windows\usr\share\vim\vim82\autoload\rustfmt.vim
}
