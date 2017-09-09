
use std::collections::HashMap;
use std::fmt;

#[derive(Clone)]
pub struct Ciudad {
    pub ciudad_id: i32,
    pub ciudad_nom: String,
    pub pais: String,
    pub poblacion: i64,
    pub latitud: f64,
    pub longitud: f64,
    pub vecinos: HashMap<i32,f64>,
}


impl fmt::Debug for Ciudad {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ciudad{{ id: {}, nombre: {}, pais: {} }}",self.ciudad_id, self.ciudad_nom,self.pais)
    }
}
