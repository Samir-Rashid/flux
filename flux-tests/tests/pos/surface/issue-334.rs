#![feature(register_tool)]
#![register_tool(flux)]

#[flux::refined_by(n: int)]
struct S {
    #[flux::field(i32[@n])]
    x: i32,
}

impl S {
    fn take_shr(&self) -> bool {
        unimplemented!()
    }

    #[flux::sig(fn(self: &strg S[@n]) ensures self: S)]
    fn take_strg(&mut self) {
        unimplemented!()
    }
}

#[flux::sig(fn(s: &strg S[@n]) ensures s: S)]
fn test(s: &mut S) {
    if s.take_shr() {
        s.take_strg();
    }
    let x = s.x;
}
