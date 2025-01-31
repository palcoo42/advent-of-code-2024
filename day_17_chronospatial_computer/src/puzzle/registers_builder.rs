use super::registers::Registers;

#[derive(Default)]
pub struct RegistersBuilder {
    a: usize,
    b: usize,
    c: usize,
}

impl RegistersBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn a(self, a: usize) -> Self {
        Self { a, ..self }
    }

    pub fn b(self, b: usize) -> Self {
        Self { b, ..self }
    }

    pub fn c(self, c: usize) -> Self {
        Self { c, ..self }
    }

    pub fn build(self) -> Registers {
        Registers {
            a: self.a,
            b: self.b,
            c: self.c,
        }
    }
}
