use std::collections::BTreeSet;

pub struct Triangle {
    a: u64,
    b: u64,
    c: u64,
    set: BTreeSet<u64>,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        let a = sides[0];
        let b = sides[1];
        let c = sides[2];

        if a > b + c || b > a + c || c > a + b || a <= 0 || b <= 0 || c <= 0 {
            return None;
        }

        let mut set = BTreeSet::new();
        set.insert(a);
        set.insert(b);
        set.insert(c);

        Some(Self { a, b, c, set })
    }

    pub fn is_equilateral(&self) -> bool {
        self.set.len() == 1
    }

    pub fn is_scalene(&self) -> bool {
        self.set.len() == 3
    }

    pub fn is_isosceles(&self) -> bool {
        self.set.len() == 2
    }

    pub fn is_degenerate(&self) -> bool {
        self.a == self.b + self.c || self.b == self.a + self.c || self.c == self.a + self.b
    }
}
