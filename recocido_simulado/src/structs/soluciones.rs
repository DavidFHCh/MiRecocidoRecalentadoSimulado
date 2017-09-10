use structs::solucion_lite::SolucionLite;

pub struct Soluciones {
    pub soluciones : Vec<SolucionLite>,
}

impl Soluciones {

    pub fn new() -> Self {
        Soluciones {
            soluciones: Vec::new(),
        }
    }

    pub fn push(&mut self, s: SolucionLite) {
        self.soluciones.push(s);
    }
}
