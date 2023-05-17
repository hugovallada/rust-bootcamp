fn main() {
    // boolean
    let b1 = true;

    //unsigned integer - positive always
    let i1: u8 = 1;
    let i2: u16 = 1;
    let i3: u32 = 1;
    let i4: u64 = 1;
    let i5: u128 = 1;

    // signed integer - positive or negative
    let i6: i8 = -1;
    let i7: i16 = -1;
    let i8: i32 = -1;
    let i9: i64 = -1;
    let i10: i128 = -1;

    // float
    let f1: f32 = 1.0;
    let f2: f64 = 1.0;

    // platform specific integer
    let p1: usize = 1;
    let p2: isize = 1;

    // characters, &str and String
    let c1: char = 'a';
    let s1: &str = "hello";
    let s2: String = String::from("hello");

    // array

    let a1: [i32; 5] = [1, 2, 3, 4, 5];
    let i1 = a1[4];

    // tuple
    let t1 = (1, 2, 3);
    let t1 = (5, 5.0, "5");
    let s1 = t1.2;
    let (i1, f1, s1) = t1;

    // unit -> Tupla vazia
    let unit = ();

    // Type aliasing
    type age = u8;
    let a1: age = 57;
}
