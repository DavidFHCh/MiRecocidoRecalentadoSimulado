use structs::ciudad::Ciudad;
use std::f64;
use std::vec::Vec;

static NO_EXISTENTE: f64 = 5.0;//MAGIC NUMBER.

#[derive(Clone)]
pub struct Solucion<'a>{
    pub ciudades_solucion: Vec<&'a Ciudad>,
    pub f_obj:f64,
    pub promedio: f64,//Este campo es para optimizar la actualizacion.
    pub sum_peso: f64,//Este campo es para optimizar la actualizacion.
    pub max_dis_castigo:f64,//Este campo es para optimizar la actualizacion.
    pub factible: bool,
    pub ordenada : Vec<&'a Ciudad>,
}


impl<'a> Solucion<'a> {

    pub fn new(v: Vec<&'a Ciudad>,v1: Vec<&'a Ciudad>) -> Self {
        let mut sol = Solucion {
                ciudades_solucion: v,
                f_obj: 0.0,
                promedio: 0.0,
                sum_peso: 0.0,
                max_dis_castigo: 0.0,
                factible: false,
                ordenada: v1,
            };
        sol.recien_creado();
        sol
    }

    pub fn vecino(&mut self, a: &usize, b: &usize){
        self.actualizar(a,b);
    }

    fn actualizar(&mut self, a: &usize, b: &usize){
        let len = self.ciudades_solucion.len() as usize;
        let ab_len = *a == (len - 1);
        let bb_len = *b == (len - 1);
        let ab_0 = *a == 0;
        let bb_0 = *b == 0;


        if *a == *b {
            return
        }
        if ab_len {
            self.f_a_eq_len(a);
        } else if ab_0 {
            self.f_a_eq_0(a);
        } else {
            self.f_a_en_medio(a);
        }
        if bb_len {
            self.f_a_eq_len(b);
        } else if bb_0 {
            self.f_a_eq_0(b);
        } else {
            self.f_a_en_medio(b);
        }

        self.swap(a,b);
        //self.actualizar_factibilidad(&len);

        if ab_len {
            self.f_a_eq_len_des(a);
        } else if ab_0 {
            self.f_a_eq_0_des(a);
        } else {
            self.f_a_en_medio_des(a);
        }
        if bb_len {
            self.f_a_eq_len_des(b);
        } else if bb_0 {
            self.f_a_eq_0_des(b);
        } else {
            self.f_a_en_medio_des(b);
        }
        self.f_obj = self.sum_peso/(self.promedio*(len as f64 -1.0));
        //println!("{}", self.promedio);
    }

    pub fn actualizar_factibilidad(&mut self,len: &usize) {//tambien checa factibilodad de la solucion.

        for i in 0..(len-1) {
            let ciudad_ac_vec = &self.ciudades_solucion[i].adyacencias;
            let id_ciudad_sig = &self.ciudades_solucion[i+1].ciudad_id;
            if ciudad_ac_vec[*id_ciudad_sig as usize] == 0.0 {
                self.factible = false;
                return
            }
        }
        self.factible = true;
    }

    fn  f_a_eq_len_des(&mut self,a: &usize) {
        let ciudad_ac_vec = &self.ciudades_solucion[*a].adyacencias;
        let id_ciudad_ant = &self.ciudades_solucion[*a-1].ciudad_id;
        if ciudad_ac_vec[*id_ciudad_ant as usize] > 0.0 {
            let dis = &ciudad_ac_vec[*id_ciudad_ant as usize];
            self.sum_peso = self.sum_peso + dis;
        } else {
            self.sum_peso = self.sum_peso + self.max_dis_castigo;
        }
    }

    fn  f_a_eq_0_des(&mut self,a: &usize) {
        let ciudad_ac_vec = &self.ciudades_solucion[*a].adyacencias;
        let id_ciudad_sig = &self.ciudades_solucion[*a+1].ciudad_id;
        if ciudad_ac_vec[*id_ciudad_sig as usize] > 0.0 {
            let dis = &ciudad_ac_vec[*id_ciudad_sig as usize];
            self.sum_peso = self.sum_peso + dis;
        } else {
            self.sum_peso = self.sum_peso + self.max_dis_castigo;
        }
    }

    fn  f_a_en_medio_des(&mut self,a: &usize) {
        let ciudad_ac_vec = &self.ciudades_solucion[*a].adyacencias;
        let id_ciudad_sig = &self.ciudades_solucion[*a+1].ciudad_id;
        let id_ciudad_ant = &self.ciudades_solucion[*a-1].ciudad_id;
        if ciudad_ac_vec[*id_ciudad_sig as usize] > 0.0 {
            let dis = &ciudad_ac_vec[*id_ciudad_sig as usize];
            self.sum_peso = self.sum_peso + dis;
        } else {
            self.sum_peso = self.sum_peso + self.max_dis_castigo;
        }
        if ciudad_ac_vec[*id_ciudad_ant as usize] > 0.0 {
            let dis = &ciudad_ac_vec[*id_ciudad_ant as usize];
            self.sum_peso = self.sum_peso + dis;
        } else {
            self.sum_peso = self.sum_peso + self.max_dis_castigo;
        }
    }

    ///Funcion para cuando a o b son iguales a len1. Antes de swap
    fn f_a_eq_len(&mut self,a: &usize) {
        let ciudad_ac_vec = &self.ciudades_solucion[*a].adyacencias;
        let id_ciudad_ant = &self.ciudades_solucion[*a-1].ciudad_id;
        if ciudad_ac_vec[*id_ciudad_ant as usize] > 0.0 {
            let dis = &ciudad_ac_vec[*id_ciudad_ant as usize];
            self.sum_peso = self.sum_peso - dis;
        } else {
            self.sum_peso = self.sum_peso - self.max_dis_castigo;
        }
    }

    ///Funcion para cuando a o b son iguales a 0. Antes de swap
    fn f_a_eq_0(&mut self, a: &usize) {
        let ciudad_ac_vec = &self.ciudades_solucion[*a].adyacencias;
        let id_ciudad_sig = &self.ciudades_solucion[*a+1].ciudad_id;
        if ciudad_ac_vec[*id_ciudad_sig as usize] > 0.0 {
            let dis = &ciudad_ac_vec[*id_ciudad_sig as usize];
            self.sum_peso = self.sum_peso - dis;
        } else {
            self.sum_peso = self.sum_peso - self.max_dis_castigo;
        }
    }

    /// funcion para cuando a esta en medio. Antes de swap.
    fn f_a_en_medio(&mut self, a: &usize){
        let ciudad_ac_vec = &self.ciudades_solucion[*a].adyacencias;
        let id_ciudad_sig = &self.ciudades_solucion[*a+1].ciudad_id;
        let id_ciudad_ant = &self.ciudades_solucion[*a-1].ciudad_id;
        if ciudad_ac_vec[*id_ciudad_sig as usize] > 0.0 {
            let dis = &ciudad_ac_vec[*id_ciudad_sig as usize];
            self.sum_peso = self.sum_peso - dis;
        } else {
            self.sum_peso = self.sum_peso - self.max_dis_castigo;
        }
        if ciudad_ac_vec[*id_ciudad_ant as usize] > 0.0 {
            let dis = &ciudad_ac_vec[*id_ciudad_ant as usize];
            self.sum_peso = self.sum_peso - dis;
        } else {
            self.sum_peso = self.sum_peso - self.max_dis_castigo;
        }
    }

    fn recien_creado(&mut self) {
        self.maximo_prom_pesos();
        self.f_obj_1();
    }

    fn swap(&mut self, a: &usize, b: &usize) {
        self.ciudades_solucion.swap(*a,*b);
    }

    fn maximo_prom_pesos(&mut self) {
        let mut max: f64 = 0.0;
        let mut aristas: f64 = 0.0;
        let mut prom: f64 = 0.0;
        let len = self.ordenada.len();
        for i in 0..len {
            for j in (i+1)..len {
                let vec = &self.ordenada[j];
                let vecinos_act_dist = &self.ordenada[i].adyacencias[vec.ciudad_id as usize];
                if *vecinos_act_dist > 0.0 {
                    prom = prom + *vecinos_act_dist;
                    aristas = aristas + 1.0;
                    if *vecinos_act_dist > max {
                        max = *vecinos_act_dist;
                    }
                }
            }
        }
        self.max_dis_castigo = NO_EXISTENTE*max;
        self.promedio = prom/(aristas);
        //println!("{}", self.promedio);
        //println!("{}", aristas);
    }

    ///Priemera vez, segunda funcion a llamar.
    fn f_obj_1(&mut self) {
        let mut sum_peso: f64 = 0.0;
        let len = self.ciudades_solucion.len() as usize;
        //let mut fact = true;

        for i in 0..(len-1) {
            let ciudad_ac_vec = &self.ciudades_solucion[i].adyacencias;
            let id_ciudad_sig = &self.ciudades_solucion[i+1].ciudad_id;
            if ciudad_ac_vec[*id_ciudad_sig as usize] > 0.0 {
                let dis = &ciudad_ac_vec[*id_ciudad_sig as usize];
                sum_peso = sum_peso + dis;
            }else{
                sum_peso = sum_peso + self.max_dis_castigo;
                //fact = false;
            }
        }
        //self.factible = fact;
        self.sum_peso = sum_peso;
        self.f_obj = self.sum_peso/(self.promedio*((len as f64) - 1.0));
    }


}
