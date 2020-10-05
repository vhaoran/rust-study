/*
Result枚举类型
Result<T, E>{
    Ok<T>,
    Err<E>,
}
T：成功时Ok成员中的数据类型
E：失败时Err成员中返回的数据类型
直接处理
对Result值进行模式匹配，分别处理
let f = File::open("hello.txt"); let mut f = match f { Ok(file) => file, Err(error) => panic!("error:{:?}", error), }
使用Result上定义的方法（类似以上）

Result.unwrap()
T = Ok<T>.unwrap()
Err<E>.unwrap()使用默认信息调用panic
Result.expect(&str)
T = Ok<T>.expect(&str)
Err<E>.expect(&str)使用&str调用!panic
Result.unwrap_or_else

Result.unwrap_or_else(|err|{
    clojure...
})
T = Ok<T>.unwrap_or_else()
Err<E>.unwrap_or_else()将E作为闭包参数调用 闭包
Result.is_err()
False = Ok<T>.is_err()
True = Err<E>.is_err()
传播错误（Propagating）
对Result对象进行匹配，提前返回Err<E>，需要注意返回值 类型问题，尤其是在可能存在多处潜在错误需要返回

let f = File:open("hello.txt");
let mut f match f {
    Ok(file) => file,
    Err(error) => return Err(error),
}
?简略写法（效果同上）

let mut f = File::open("hello.txt")?
?会把Err(error)传递给from函数（定义在标准库From trait中），将错误从转换为函数返回值中的类型，潜在 错误类型都实现了from函数
?只能用于返回值为Result类型的函数内，因为其"返回值" 就是Err(E)（如果有）
*/