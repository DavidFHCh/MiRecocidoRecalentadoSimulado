extern crate rand;

use structs::{ciudad,solucion_lite,solucion,soluciones};
use self::rand::{XorShiftRng, SeedableRng, Rng};
use self::ciudad::Ciudad;
use self::solucion::Solucion;
use self::solucion_lite::SolucionLite;
use self::soluciones::Soluciones;
use conexion_bd::get_ciudades;

static TAMLOTE: usize = 500;

pub struct RecocidoSimulado<'a> {
    pub temp: f64,
    pub solucion_act: Solucion<'a>,
    pub rng: XorShiftRng,
    pub sols: Soluciones,
}

impl<'a> RecocidoSimulado<'a> {

    pub fn new(mut s_init: Vec<&'a Ciudad>,seed: [u32;4]) -> Self {
        let mut rng: XorShiftRng = SeedableRng::from_seed(seed);
        rng.shuffle(&mut s_init);
        let mut s = Solucion::new(s_init);
        RecocidoSimulado {
            temp: 0.0,
            solucion_act: s,
            rng: rng,
            sols: Soluciones::new(),
        }
    }

    pub fn calcula_lote<T: Rng>(&mut self, rng: &mut T) -> (f64, &Solucion){
        let mut c: usize = 0;
        let mut r: f64 = 0.0;
        let mut a: usize = 0;
        let mut b: usize = 0;

        let len: usize = self.solucion_act.ciudades_solucion.len();

        while c < TAMLOTE {
            let s = self.solucion_act.clone();
            a = rng.gen_range(0,len);//Aqui todo se vuelve no determinista.
            b = rng.gen_range(0,len);//Aqui todo se vuelve no determinista.
            self.solucion_act.vecino(&a,&b);
            if self.solucion_act.f_obj <= (s.f_obj + self.temp) {
                c = c + 1;
                r = r + self.solucion_act.f_obj;
                self.sols.push(SolucionLite::new(&self.solucion_act));//para el ploteo.
            } else {
                self.solucion_act = s;
            }
        }
        r = r/(TAMLOTE as f64);
        (r,&self.solucion_act)
    }


}
