#[xlint::public]
pub enum Enum {
    Foo,
    Bar { bar: () },
}

#[xlint::public]
pub struct Struct {
    pub foo: (),
    pub bar: (),
}

#[xlint::public]
pub struct Unit;

#[xlint::public]
pub struct Union {
    pub foo: (),
    pub bar: (),
}

#[xlint::public]
pub struct Tuple(pub (), pub ());

fn main() {}
