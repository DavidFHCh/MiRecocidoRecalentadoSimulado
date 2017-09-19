
use std::fmt;

#[derive(Clone)]
pub struct Ciudad {
    pub ciudad_id: i32,
    pub adyacencias: Vec<f64>,
}


impl fmt::Debug for Ciudad {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ciudad{{ id: {}}}", self.ciudad_id)
    }
}
