// lifetime
// struct Foo<'a> {
//     x: &'a i32,
// }
//
// impl<'a> Foo<'a> {
//     fn x(&self) -> &'a i32 { self.x }
// }

// call method
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

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
    // let x  = 1;
    // let x2 = x;
    // println!("{}", x2);
    
    // borrowing
    // let a = vec![1, 2, 3];
    // let b = vec![2, 3, 1];
    //
    // let ans = dot(&a, &b);
    // println!("{}", ans);
    //
    // fn dot(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    //     let mut d: i32 = 0;
    //     for i in 0..v1.len() {
    //         d += v1[i] * v2[i];
    //     }
    //     d
    // }
    
    
    // borrowing2
    // let mut v = vec![1, 2, 3]; // mutは必要ないけど、pushで変更したい想定でmutにする
    //
    // for i in &v {
    //     println!("{}", i);
    //     // v.push(34) // loopが借用するので変更できない
    // }
    
    // scope
    // let y: &i32;
    // {
    //     let x = 5;
    //     y = &x; // xはこの{}内でしか生きていないので、その外で定義されるyに貸そうとすると怒られる。
    // }
    //
    // println!("{}", y);
    
    // let y: &i32;
    // let x = 5;
    // y = &x; // リソースは宣言された順と逆順(x, yの順)で開放されるのでyのほうがxより長生き。なので怒られる。
    // println!("{}", y);
    
    // lifetime
    // let y = &5;
    // let f = Foo { x: y };
    //
    // println!("{}", f.x());
    
    // update
    // struct Point3d {
    //     x: i32,
    //     y: i32,
    //     z: i32,
    // }
    //
    // let mut point = Point3d { x: 10, y: 10, z: 10 };
    // let mut q = Point3d { y: 1, .. point };
    //
    // println!("{}", q.x);
    // println!("{}", q.y);
    // println!("{}", q.z);
    //
    // println!("{}", point.x);
    // println!("{}", point.y);
    // println!("{}", point.z);
    
    // struct
    // struct Inches(i32);
    // let length = Inches(10);
    //
    // let Inches(integer_length) = length;
    // println!("length is {} inches", integer_length);
    
    // pattern
    // let x = 'x';
    // let c = 'c';
    //
    // match c {
    //     x => println!("x: {} c: {}", x, c),
    // }
    //
    // println!("x: {}", x)
    
    // struct Point {
    //     x: i32,
    //     y: i32,
    // }
    //
    // let origin = Point { x: 0, y: 0 };
    //
    // match origin {
    //     Point { y, .. } => println!("x is {}, y is {}", origin.x, y),
    // }
    
    // enum OptionalTuple {
    //     Value(i32, i32, i32),
    //     Missing,
    // }
    //
    // let x = OptionalTuple::Value(5, -2, 3);
    //
    // match x {
    //     OptionalTuple::Value(..) => println!("Got a tuple!"),
    //     OptionalTuple::Missing => println!("No such luck."),
    // }
    
    // // let x = 5;
    // let mut x = 5;
    //
    // match x {
    //     // ref r => println!("Got a refrence to {}", r),
    //     ref mut mr => println!("Got a mutable refrence to {}", mr),
    // }
    
    // #[derive(Debug)]
    // struct Person {
    //     name: Option<String>,
    // }
    //
    // let name = "Steve".to_string();
    // let mut x: Option<Person> = Some(Person { name: Some(name) });
    // match x {
    //     Some(Person { name: ref a @ Some(_), .. }) => println!("{:?}", a),
    //     _ => {}
    // }
    
    // let x = 2;
    //
    // match x {
    //     e @ 1 ... 5 | e @ 8 ... 10 => println!("got a range element {}", e),
    //     _ => println!("anything"),
    // }

    // enum OptionalInt {
    //     Value(i32),
    //     Missing,
    // }
    //
    // let x = OptionalInt::Value(6);
    //
    // match x {
    //     OptionalInt::Value(i) if i > 5 => println!("Got an int bigger than five!"),
    //     OptionalInt::Value(..) => println!("Got an int!"),
    //     OptionalInt::Missing => println!("No such luck."),
    // }

    let c = Circle { x: 0.0, y: 0.0, radius: 2.0 };
    println!("{}", c.area());

}