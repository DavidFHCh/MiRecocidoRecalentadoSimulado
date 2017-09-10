use structs::ciudad::Ciudad;

static NO_EXISTENTE: f64 = 24822373537468.38;//usando el cuadrado de la distancia mas grande de la tabla connections. MAGIC NUMBER.

#[derive(Clone)]
pub struct Solucion<'a>{
    pub ciudades_solucion: Vec<&'a Ciudad>,
    pub f_obj:f64,
    pub sum_dist_exist: f64,//Este campo es para optimizar la actualizacion.
    pub promedio: f64,//Este campo es para optimizar la actualizacion.
    pub sum_peso: f64,//Este campo es para optimizar la actualizacion.
    pub max_dis_castigo:f64,//Este campo es para optimizar la actualizacion.
    pub factible: bool,
}


impl<'a> Solucion<'a> {

    pub fn new(v: Vec<&'a Ciudad>) -> Self {
        let mut sol = Solucion {
                ciudades_solucion: v,
                f_obj: 0.0,
                sum_dist_exist: 0.0,
                promedio: 0.0,
                sum_peso: 0.0,
                max_dis_castigo: 0.0,
                factible: false,
            };
        sol.recien_creado();
        sol
    }

    pub fn actualizacion(&mut self, a: &usize, b: &usize){
        self.actualizar(a,b);
    }

    fn actualizar(&mut self, a: &usize, b: &usize){
        let len = self.ciudades_solucion.len() as usize;
        let ab_len = self.f_a_eq_len(a,&len);
        let bb_len = self.f_a_eq_len(b,&len);
        let ab_0 = self.f_a_eq_0(a);
        let bb_0 = self.f_a_eq_0(b);
        if ab_len {
            if !bb_0 {
                self.f_a_en_medio(b);
            }
        } else if bb_len {
            if !ab_0 {
                self.f_a_en_medio(a);
            }
        } else {
            self.f_a_en_medio(a);
            self.f_a_en_medio(b);
        }

        self.swap(a,b);
        self.actualizar_maximo_factibilidad(&len);

        if ab_len {
            self.f_a_eq_len_des(a);
            if bb_0 {
                self.f_a_eq_0_des(b);
            } else {
                self.f_a_en_medio_des(b);
            }
        } else if bb_len {
            self.f_a_eq_len_des(b);
            if ab_0 {
                self.f_a_eq_0_des(a);
            } else {
                self.f_a_en_medio_des(a);
            }
        } else {
            self.f_a_en_medio_des(a);
            self.f_a_en_medio_des(b);
        }
        self.promedio = self.sum_dist_exist/(len as f64);
        self.f_obj = self.sum_peso/self.promedio;
    }

    fn actualizar_maximo_factibilidad(&mut self,len: &usize) {//tambien checa factibilodad de la solucion.
        let mut max: f64 = 0.0;

        for i in 0..((*len)-1) {
            let ciudad_ac_vec = &self.ciudades_solucion[i].vecinos;
            let id_ciudad_sig = &self.ciudades_solucion[i+1].ciudad_id;
            if ciudad_ac_vec.contains_key(id_ciudad_sig) {
                let dis = ciudad_ac_vec.get(id_ciudad_sig).unwrap();
                if *dis > max {
                    max = *dis;
                }
            }else{
                self.max_dis_castigo = NO_EXISTENTE;
                self.factible = false;
                return
            }
        }
        self.max_dis_castigo = max;
        self.factible = true;
    }

    fn  f_a_eq_len_des(&mut self,a: &usize) {
        let ciudad_ac_vec = &self.ciudades_solucion[*a].vecinos;
        let id_ciudad_ant = &self.ciudades_solucion[*a-1].ciudad_id;
        if ciudad_ac_vec.contains_key(id_ciudad_ant) {
            let dis = ciudad_ac_vec.get(id_ciudad_ant).unwrap();
            self.sum_dist_exist = self.sum_dist_exist + *dis;
            self.sum_peso = self.sum_peso + *dis;
        } else {
            self.sum_peso = self.sum_peso + self.max_dis_castigo;
        }
    }

    fn  f_a_eq_0_des(&mut self,a: &usize) {
        let ciudad_ac_vec = &self.ciudades_solucion[*a].vecinos;
        let id_ciudad_sig = &self.ciudades_solucion[*a+1].ciudad_id;
        if ciudad_ac_vec.contains_key(id_ciudad_sig) {
            let dis = ciudad_ac_vec.get(id_ciudad_sig).unwrap();
            self.sum_dist_exist = self.sum_dist_exist + *dis;
            self.sum_peso = self.sum_peso + *dis;
        } else {
            self.sum_peso = self.sum_peso + self.max_dis_castigo;
        }
    }

    fn  f_a_en_medio_des(&mut self,a: &usize) {
        let ciudad_ac_vec = &self.ciudades_solucion[*a].vecinos;
        let id_ciudad_sig = &self.ciudades_solucion[*a+1].ciudad_id;
        let id_ciudad_ant = &self.ciudades_solucion[*a-1].ciudad_id;
        if ciudad_ac_vec.contains_key(id_ciudad_sig) {
            let dis = ciudad_ac_vec.get(id_ciudad_sig).unwrap();
            self.sum_dist_exist = self.sum_dist_exist + *dis;
            self.sum_peso = self.sum_peso + *dis;
        } else {
            self.sum_peso = self.sum_peso + self.max_dis_castigo;
        }
        if ciudad_ac_vec.contains_key(id_ciudad_ant) {
            let dis = ciudad_ac_vec.get(id_ciudad_ant).unwrap();
            self.sum_dist_exist = self.sum_dist_exist + *dis;
            self.sum_peso = self.sum_peso + *dis;
        } else {
            self.sum_peso = self.sum_peso + self.max_dis_castigo;
        }
    }

    ///Funcion para cuando a o b son iguales a len1. Antes de swap
    fn f_a_eq_len(&mut self,a: &usize, len: &usize) -> bool{
        if *a == ((*len)-1) {
            let ciudad_ac_vec = &self.ciudades_solucion[*a].vecinos;
            let id_ciudad_ant = &self.ciudades_solucion[*a-1].ciudad_id;
            if ciudad_ac_vec.contains_key(id_ciudad_ant) {
                let dis = ciudad_ac_vec.get(id_ciudad_ant).unwrap();
                self.sum_dist_exist = self.sum_dist_exist - *dis;
                self.sum_peso = self.sum_peso - *dis;
            } else {
                self.sum_peso = self.sum_peso - self.max_dis_castigo;
            }
            return true
        }
        return false
    }

    ///Funcion para cuando a o b son iguales a 0. Antes de swap
    fn f_a_eq_0(&mut self, a: &usize) -> bool{
        if *a == 0 {
            let ciudad_ac_vec = &self.ciudades_solucion[*a].vecinos;
            let id_ciudad_sig = &self.ciudades_solucion[*a+1].ciudad_id;
            if ciudad_ac_vec.contains_key(id_ciudad_sig) {
                let dis = ciudad_ac_vec.get(id_ciudad_sig).unwrap();
                self.sum_dist_exist = self.sum_dist_exist - *dis;
                self.sum_peso = self.sum_peso - *dis;
            } else {
                self.sum_peso = self.sum_peso - self.max_dis_castigo;
            }
            return true
        }
        return false
    }

    /// funcion para cuando a esta en medio. Antes de swap.
    fn f_a_en_medio(&mut self, a: &usize){
        let ciudad_ac_vec = &self.ciudades_solucion[*a].vecinos;
        let id_ciudad_sig = &self.ciudades_solucion[*a+1].ciudad_id;
        let id_ciudad_ant = &self.ciudades_solucion[*a-1].ciudad_id;
        if ciudad_ac_vec.contains_key(id_ciudad_sig) {
            let dis = ciudad_ac_vec.get(id_ciudad_sig).unwrap();
            self.sum_dist_exist = self.sum_dist_exist - *dis;
            self.sum_peso = self.sum_peso - *dis;
        } else {
            self.sum_peso = self.sum_peso - self.max_dis_castigo;
        }
        if ciudad_ac_vec.contains_key(id_ciudad_ant) {
            let dis = ciudad_ac_vec.get(id_ciudad_ant).unwrap();
            self.sum_dist_exist = self.sum_dist_exist - *dis;
            self.sum_peso = self.sum_peso - *dis;
        } else {
            self.sum_peso = self.sum_peso - self.max_dis_castigo;
        }
    }

    fn recien_creado(&mut self) {
        self.f_promedio_max_1();
        self.f_obj_1();
    }

    fn swap(&mut self, a: &usize, b: &usize) {
        self.ciudades_solucion.swap(*a,*b);
    }

    ///Primera funcion para llamar al crear Solucion.
    fn f_promedio_max_1(&mut self) {
        let mut promedio: f64 = 0.0;
        let mut max: f64 = 0.0;
        let mut fact: bool = true;


        let len = self.ciudades_solucion.len() as usize;

        for i in 0..(len-1) {
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
                fact = false;
            }
        }
        self.sum_dist_exist = promedio;
        self.promedio = self.sum_dist_exist/(len as f64);
        self. max_dis_castigo = max;
        self.factible = fact;
    }

    ///Priemera vez, segunda funcion a llamar.
    fn f_obj_1(&mut self) {
        let mut sum_peso: f64 = 0.0;
        let len = self.ciudades_solucion.len() as usize;

        for i in 0..(len-1) {
            let ciudad_ac_vec = &self.ciudades_solucion[i].vecinos;
            let id_ciudad_sig = &self.ciudades_solucion[i+1].ciudad_id;
            if ciudad_ac_vec.contains_key(id_ciudad_sig) {
                let dis = ciudad_ac_vec.get(id_ciudad_sig).unwrap();
                sum_peso = sum_peso + *dis;
            }else{
                sum_peso = sum_peso + self.max_dis_castigo;
            }
        }
        self.sum_peso = sum_peso;
        self.f_obj = self.sum_peso/self.promedio;
    }


}
