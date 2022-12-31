#[xlint::public]
pub struct Struct {
    pub foo: (),
    bar: (),
}

#[xlint::public]
pub union Union {
    pub foo: (),
    bar: (),
}

#[xlint::public]
pub struct Tuple(pub (), ());

fn main() {}
