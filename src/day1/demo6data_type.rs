pub fn data_type() {
    // 在 Rust 中，每一个值都属于某一个 数据类型（data type），这告诉 Rust 它被指定为何种数据，
    // 以便明确数据处理方式。我们将看到两类数据类型子集：标量（scalar）和复合（compound）。
    // 记住，Rust 是 静态类型（statically typed）语言，也就是说在编译时就必须知道所有变量的类型。
    // 根据值及其使用方式，编译器通常可以推断出我们想要用的类型。当多种类型均有可能时，
    // 例如guess必须标注其变量类型：
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess = {}", guess);

    // Rust 中的整型:
    // 长度	    有符号	无符号
    // 8-bit	i8	    u8
    // 16-bit	i16	    u16
    // 32-bit	i32	    u32
    // 64-bit	i64	    u64
    // 128-bit	i128	u128
    // arch	    isize	usize

    // Rust 也有两个原生的 浮点数（floating-point numbers）类型，它们是带小数点的数字。
    // Rust 的浮点数类型是 f32 和 f64，分别占 32 位和 64 位。默认类型是 f64，
    // 因为在现代 CPU 中，它与 f32 速度几乎一样，不过精度更高。

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("x = {}", x);
    println!("y = {}", y);

    // Rust 中的所有数字类型都支持基本数学运算：加法、减法、乘法、除法和取余。
    // 整数除法会向下舍入到最接近的整数。下面的代码展示了如何在 let 语句中使用它们：

    // 加法
    let sum = 5 + 10;
    println!("sum = {}", sum);
    // 减法
    let difference = 95.5 - 4.3;
    println!("difference = {}", difference);
    // 乘法
    let product = 4 * 30;
    println!("product = {}", product);
    // 除法
    let quotient = 56.7 / 32.2;
    println!("quotient = {}", quotient);
    let floored = 2 / 3; // 结果为 0
    println!("floored = {}", floored);
    // 取余
    let remainder = 43 % 5;
    println!("remainder = {}", remainder);
    // 正如其他大部分编程语言一样，Rust 中的布尔类型有两个可能的值：true 和 false。
    // Rust 中的布尔类型使用 bool 表示。例如：
    let t = true;
    let f: bool = false; // 显式指定类型注解
    println!("t = {}", t);
    println!("f = {}", f);
    // 字符，用单引号标注
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("c = {}", c);
    println!("z = {}", z);
    println!("heart_eyed_cat = {}", heart_eyed_cat);
    // 复合类型（Compound types）可以将多个值组合成一个类型。
    // Rust 有两个原生的复合类型：元组（tuple）和数组（array）。

    // 元组是一个将多个其他类型的值组合进一个复合类型的主要方式。元组长度固定：一旦声明，其长度不会增大或缩小。
    // 我们使用包含在圆括号中的逗号分隔的值列表来创建一个元组。
    // 元组中的每一个位置都有一个类型，而且这些不同值的类型也不必是相同的。这个例子中使用了可选的类型注解：
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // 可运用解构来获取tup的值
    let (a, b, c) = tup;
    println!("a = {}", a);
    println!("b = {}", b);
    println!("c = {}", c);
    //也可以使用.来访问它们
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // 另一个包含多个值的方式是 数组（array）。
    // 与元组不同，数组中的每个元素的类型必须相同。
    // Rust 中的数组与一些其他语言中的数组不同，
    // 因为 Rust 中的数组是固定长度的：一旦声明，它们的长度不能增长或缩小。
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let array: [i32; 5] = [1, 2, 3, 4, 5];
    // 3;5 -> 3,3,3,3,3
    let array2 = [3; 5];

    // 访问
    let first = array2[0];
    println!("first = {}", first);
    let second = array2[1];
    println!("second = {}", second);
}
