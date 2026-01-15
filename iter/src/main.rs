/// We want to test implementing the iter trait for a struct
pub struct MetaVec {
    meta: u8,
    curr: u8,
}

impl Iterator for MetaVec {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr <= self.meta {
            let buf = self.curr;
            self.curr += 1;
            return Some(buf);
        }
        None
    }
}
fn main() {
    let m = MetaVec { meta: 10, curr: 0 };

    for i in m.into_iter() {
        println!("i: {i}");
    }

    println!("Hello, world!");
}
