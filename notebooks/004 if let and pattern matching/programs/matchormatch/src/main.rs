// #[allow(unused_variables)]
// #[allow(dead_code)]
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
        // Ecrire le code ici
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
