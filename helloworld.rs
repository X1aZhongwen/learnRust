fn main(){
    println!("Hello World!");
    tabbb();
}
// 缩进是四个空格而不是tab' main函数是特殊函数是程序最先运行的函数
fn tabbb(){
    println!("secod");
}
// rustc 只适合小型程序编程，大型得用Cargo
// cargo new _name