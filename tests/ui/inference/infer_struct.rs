// run-pass

mod stf {}

struct Foo {
    _a: usize,
    _b: isize,
}

fn foo(_: Foo) {}

fn main() {
    foo(.{_a:10,_b:-10})
}
