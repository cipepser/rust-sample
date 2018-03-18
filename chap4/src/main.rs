fn main() {
    // let x: i32;
    // println!("The value of x is: {}", x); // 初期化してないからエラー
    
    // array
    // let a = [1, 2, 3];
    // let b = &a[1..3]; // slice
    // println!("{}", b[2]);
    
    // if
    // let x = 5;
    // if x == 5 {
    //     println!("x is 5");
    // } else {
    //     println!("x is not 5");
    // }
    //
    // let y = if x == 5 { 10 } else { 15 };
    //
    // println!("{}", y);
    
    // enum
    // go の for i, v := rangeみたいな感じ
    // for (i, j) in (5..10).enumerate() {
    //     println!("i = {} and j = {}", i, j);
    // }
    
    // loop label
    // 'outer: for x in 0..10 {
    //     'inner: for y in 0..10 {
    //         if x % 2 == 0 {continue 'outer; }
    //         if y % 2 == 0 {continue 'inner; }
    //         println!("x: {}, y: {}", x, y);
    //     }
    // }
    
    // ownership
    // VecはCopy traitを実装していないのでuse of move value: `v`でエラー
    // let v = vec![1, 2, 3];
    // let v2 = v;
    // println!("v[0] is: {}", v[0]);
    // println!("v2[0] is: {}", v2[0]);

    // i32はCopy traitを実装しているので怒られない
    let x  = 1;
    let x2 = x;
    println!("{}", x2);
}