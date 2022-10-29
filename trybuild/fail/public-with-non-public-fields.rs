#[enforce_visibility::public]
pub struct Struct {
    pub foo: (),
    bar: (),
}

#[enforce_visibility::public]
pub union Union {
    pub foo: (),
    bar: (),
}

#[enforce_visibility::public]
pub struct Tuple(pub (), ());

fn main() {}
