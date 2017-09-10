extern crate rand;

use structs::{ciudad,solucion_lite,solucion,soluciones};
use self::rand::{XorShiftRng, SeedableRng, Rng};
use self::ciudad::Ciudad;
use self::solucion::Solucion;
use self::solucion_lite::SolucionLite;
use self::soluciones::Soluciones;
use conexion_bd::get_ciudades;
use std::f64;

static TAMLOTE: usize = 500;
static EPSILON: f64 = 0.01;
static EPSILON_P: f64 = 0.01;
static EPSILON_T: f64 = 0.01;
static PHI: f64 = 0.85;
static P: f64 = 0.85;
static N: usize = 500;


pub struct RecocidoSimulado<'a> {
    pub temp: f64,
    pub solucion_act: Solucion<'a>,
    pub solucion_min: Solucion<'a>,
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
            solucion_act: s.clone(),
            solucion_min: s,
            rng: rng,
            sols: Soluciones::new(),
        }
    }

    fn calcula_lote(&mut self) -> f64{
        let mut c: usize = 0;
        let mut r: f64 = 0.0;
        let mut a: usize;
        let mut b: usize;

        let len: usize = self.solucion_act.ciudades_solucion.len();

        while c < TAMLOTE {
            let s = self.solucion_act.clone();
            a = self.rng.gen_range(0,len);//Aqui todo se vuelve no determinista.
            b = self.rng.gen_range(0,len);//Aqui todo se vuelve no determinista.
            self.solucion_act.vecino(&a,&b);
            if self.solucion_act.f_obj <= (s.f_obj + self.temp) {
                c = c + 1;
                r = r + self.solucion_act.f_obj;
                self.sols.push(SolucionLite::new(&self.solucion_act));//para el ploteo.
                if self.solucion_act.f_obj < self.solucion_min.f_obj {
                    self.solucion_min = self.solucion_act.clone();//aqui mantenemos la minima.
                }

            } else {
                self.solucion_act = s;
            }
        }
        r = r/(TAMLOTE as f64);
        r
    }

    pub fn aceptacion_umbrales(&mut self) {
        let mut p: f64 = 0.0;
        let mut q: f64 = f64::INFINITY;

        while self.temp > EPSILON {
            let mut p_prim = q;
            while p <= p_prim {
                p_prim = p;
                p = self.calcula_lote();
                q = p.clone();
            }
            self.temp = self.temp * PHI;
        }
    }

    fn temp_inicial(&mut self){
        let mut p = self.porcent_acep();
        let mut temp_1: f64= 0.0;
        let mut temp_2: f64= 0.0;

        if (P - p).abs() <= EPSILON_P {
            //NADA
        } else if p < P {
            while p < P {
                self.temp = 2.0 * self.temp;
                p = self.porcent_acep();
            }
            temp_1 = self.temp/2.0;
            temp_2 = self.temp.clone();
        } else {
            while p > P {
                self.temp = self.temp/2.0;
                p = self.porcent_acep();
            }
            temp_1 = self.temp.clone();
            temp_2 = self.temp*2.0;
        }
        self.busqueda_binaria(temp_1, temp_2);
    }

    fn porcent_acep(&mut self) -> f64{
        let mut c: f64 = 0.0;
        let mut s = self.solucion_act.clone();

        let len: usize = self.solucion_act.ciudades_solucion.len();

        for i in 1..N {
            let mut a = self.rng.gen_range(0,len);//Aqui todo se vuelve no determinista.
            let mut b = self.rng.gen_range(0,len);//Aqui todo se vuelve no determinista.
            let mut s_prim = s.clone();
            s_prim.vecino(&a, &b);
            if s_prim.f_obj <= (s.f_obj + self.temp) {
                c = c + 1.0;
            }
            s = s_prim;
        }
        c/(N as f64)
    }

    fn busqueda_binaria(&mut self, t1: f64, t2: f64) {
        let mut tm = (t1 + t2)/2.0;
        if (t2 - t1) < EPSILON_T {
            self.temp = tm;
        } else {
            let mut p = self.porcent_acep();

            if (P - p).abs() <= EPSILON_P {
                self.temp = tm;
            } else if p > P {
                self.busqueda_binaria(t1,tm);
            }else {
                self.busqueda_binaria(tm,t2);
            }
        }
    }


}
