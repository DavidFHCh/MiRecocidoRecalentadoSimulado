use structs::ciudad::Ciudad;

static NO_EXISTENTE: f64 = 24822373537468.38;//usando el cuadrado de la distancia mas grande de la tabla connections. MAGIC NUMBER.

pub struct Solucion<'a>{
    pub ciudades_solucion: Vec<&'a Ciudad>,
    pub f_obj:f64,
    pub promedio: f64,//Este campo es para optimizar la actualizacion.
    pub sum_peso: f64,//Este campo es para optimizar la actualizacion.
    pub max_dis_castigo:f64,//Este campo es para optimizar la actualizacion.
}


impl<'a> Solucion<'a> {
    pub fn new(v: Vec<&'a Ciudad>) -> Self {
        let mut sol = Solucion {
                ciudades_solucion: v,
                f_obj: 0.0,
                promedio: 0.0,
                max_dis_castigo: 0.0,
            };
        sol.recien_creado();
        sol
    }

    pub fn actualizacion(){
        
    }

    fn recien_creado(&mut self) {
        self.f_promedio_max_1();
        self.f_obj_1();
    }

    fn swap(&mut self, a: usize, b: usize) {
        self.ciudades_solucion.swap(a,b);
    }

    ///Primera funcion para llamar al crear Solucion.
    fn f_promedio_max_1(&mut self) {
        let mut promedio: f64 = 0.0;
        let mut max: f64 = 0.0;
        let len = self.ciudades_solucion.len() as usize;

        for i in 0..len {
            if i < len {
                let ciudad_ac_vec = &self.ciudades_solucion[i].vecinos;
                let id_ciudad_sig = &self.ciudades_solucion[i+1].ciudad_id;
                if ciudad_ac_vec.contains_key(id_ciudad_sig) {
                    let dis = ciudad_ac_vec.get(id_ciudad_sig).unwrap();
                    if *dis > max {
                        max = *dis;
                    }
                    promedio = promedio + *dis;
                }else{
                    max = NO_EXISTENTE;
                }
            }
        }
        self.promedio = promedio/((len+1) as f64);
        self. max_dis_castigo = max;
    }

    ///Priemera vez, segunda funcion a llamar.
    fn f_obj_1(&mut self) {
        let mut sum_peso: f64 = 0.0;
        let len = self.ciudades_solucion.len() as usize;

        for i in 0..len {
            if i < len {
                let ciudad_ac_vec = &self.ciudades_solucion[i].vecinos;
                let id_ciudad_sig = &self.ciudades_solucion[i+1].ciudad_id;
                if ciudad_ac_vec.contains_key(id_ciudad_sig) {
                    let dis = ciudad_ac_vec.get(id_ciudad_sig).unwrap();
                    sum_peso = sum_peso + *dis;
                }else{
                    sum_peso = sum_peso + self.max_dis_castigo;
                }
            }
        }
        self.sum_peso = sum_peso;
        self.f_obj = self.sum_peso/self.promedio;
    }


}
