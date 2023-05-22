#[allow(unused_variables)]
#[allow(dead_code)]
fn main() {
    enum MyEnum {
        FirstVariant(A),
        SecondVariant(B),
        None,
    }
    struct A {
        a: usize,
        b: usize,
        c: [usize; 4],
    }
    struct B {
        n: usize,
        keys: (u8, u8, u8, u8),
    }
    fn test(e: MyEnum) {
        if let MyEnum::None = e {
            panic!("Sa mere");
        }
        match e {
            MyEnum::FirstVariant(A {
                a: 42,
                b: _,
                c: array,
            }) => println!("1: FirstVariant Variant - array is {:?}", array),
            MyEnum::FirstVariant(A {
                c: [1 | 11, 2, 3, 4],
                ..
            }) => println!("2: FirstVariant Variant - Last array is [1 or 11, 2, 3, 4]"),
            MyEnum::FirstVariant(others) => println!("3: FirstVariant Variant, others values"),
            MyEnum::SecondVariant(B {
                keys: (first_key, ..),
                ..
            }) if first_key < 10 => {
                println!("4: SecondVariant Variant, first_key LOW = {}", first_key)
            }
            MyEnum::SecondVariant(B {
                keys: (first_key, ..),
                ..
            }) if first_key >= 10 => {
                println!("5: SecondVariant Variant, first_key HIGH = {}", first_key)
            }
            MyEnum::SecondVariant(_) => panic!("Unexpected !"),
            MyEnum::None => panic!("Woot ?!?"),
        }
    }
    test(MyEnum::FirstVariant(A {
        a: 42,
        b: 11,
        c: [1, 2, 3, 4],
    }));
    test(MyEnum::FirstVariant(A {
        a: 12,
        b: 11,
        c: [11, 2, 3, 4],
    }));
    test(MyEnum::FirstVariant(A {
        a: 12,
        b: 11,
        c: [20, 2, 3, 4],
    }));
    test(MyEnum::SecondVariant(B {
        n: 4,
        keys: (8, 12, 10, 21),
    }));
    test(MyEnum::SecondVariant(B {
        n: 4,
        keys: (18, 12, 10, 21),
    }));
}
