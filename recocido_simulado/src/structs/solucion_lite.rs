use structs::solucion::Solucion;

pub struct SolucionLite {
    pub f_obj: f64,
    pub factible: bool,
}

impl SolucionLite {

    pub fn new(s: &Solucion) -> Self {
        SolucionLite {
            f_obj: s.f_obj,
            factible: s.factible,
        }
    }

}
