#[enforce_visibility::public]
pub enum Enum {
    Foo,
    Bar { bar: () },
}

#[enforce_visibility::public]
pub struct Struct {
    pub foo: (),
    pub bar: (),
}

#[enforce_visibility::public]
pub struct Unit;

#[enforce_visibility::public]
pub struct Union {
    pub foo: (),
    pub bar: (),
}

#[enforce_visibility::public]
pub struct Tuple(pub (), pub ());

fn main() {}
