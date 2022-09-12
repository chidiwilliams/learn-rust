use core::lazy;
use std::{fmt::Debug, fmt::Display};

struct A;

struct S(A);

struct SGen<T>(T);

fn main() {
    let _s = S(A);

    let _char = SGen('a');

    let _t = SGen(A);
    let _i32 = SGen(6);

    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(6));
    generic::<char>(SGen('a'));
    generic(SGen('c'));

    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };
    println!("{} {}", x.value(), y.value());

    let empty = Empty;
    let null = Null;

    empty.double_drop(null);
    // empty;
    // null;

    // let s = SD(vec![1]);

    let rectangle = Rectangle {
        length: 3.0,
        height: 4.0,
    };
    let _triangle = Triangle {
        length: 3.0,
        height: 4.0,
    };

    print_debug(&rectangle);
    println!("Area: {}", rectangle.area());

    // println!("Area: {}", _triangle.area());

    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey = Turkey;

    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
}

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}

struct Val {
    val: f64,
}

impl Val {
    fn value(&self) -> f64 {
        self.val
    }
}

struct GenVal<T> {
    gen_val: T,
}

impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

struct Empty;
struct Null;

trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

impl<T, U> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {}
}

fn printer<T: Display>(t: T) {
    println!("{}", t);
}

struct SD<T: Display>(T);

trait HasArea {
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}

struct Triangle {
    length: f64,
    height: f64,
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}

impl Blue for BlueJay {}

fn red<T: Red>(_: &T) -> &'static str {
    "red"
}

fn blue<T: Blue>(_: &T) -> &str {
    "blue"
}

fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: `{:?}`", t);
    println!("Display: `{}`", t);
}
