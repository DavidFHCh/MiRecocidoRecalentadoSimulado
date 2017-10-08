extern crate rand;

use structs::{ciudad,solucion_lite,solucion,soluciones};
use self::rand::{XorShiftRng, SeedableRng, Rng};
use self::ciudad::Ciudad;
use self::solucion::Solucion;
use self::solucion_lite::SolucionLite;
use self::soluciones::Soluciones;
use std::f64;

static TAMLOTE: usize = 200;
static EPSILON: f64 = 0.04;
static EPSILON_P: f64 = 0.0001;
static EPSILON_T: f64 = 0.0001;
static PHI: f64 = 0.9;
static P: f64 = 0.85;
static N: usize = 200;


pub struct RecocidoSimulado<'a> {
    pub temp: f64,
    pub solucion_act: Solucion<'a>,
    pub solucion_min: Solucion<'a>,
    pub solucion_temp: Solucion<'a>,
    pub rng: XorShiftRng,
    pub sols: Soluciones,
}

impl<'a> RecocidoSimulado<'a> {

    pub fn new(mut s_init: Vec<&'a Ciudad>,seed: [u32;4]) -> Self {
        let mut rng: XorShiftRng = SeedableRng::from_seed(seed);
        rng.shuffle(&mut s_init);
        let s = Solucion::new(s_init);
        let mut rs = RecocidoSimulado {
            temp: 13.0,
            solucion_act: s.clone(),
            solucion_min: s.clone(),
            solucion_temp: s.clone(),
            rng: rng,
            sols: Soluciones::new(),
        };
        rs.temp_inicial();
        rs
    }

    fn calcula_lote(&mut self) -> f64{
        let mut c: usize = 0;
        let mut r: f64 = 0.0;
        let mut a: usize;
        let mut b: usize;
        let mut intentos: usize = 0;

        let len: usize = self.solucion_act.ciudades_solucion.len();

        while c < TAMLOTE {

            let s = self.solucion_act.clone();
            a = self.rng.gen_range(0,len);//Aqui todo se vuelve no determinista.
            b = self.rng.gen_range(0,len);//Aqui todo se vuelve no determinista.
            self.solucion_act.vecino(&a,&b);
            if self.solucion_act.f_obj <= (s.f_obj + self.temp) {
                //println!("E:{}",self.solucion_act.f_obj);
                intentos = 0;
                c = c + 1;
                r = r + self.solucion_act.f_obj;
                self.sols.push(SolucionLite::new(&self.solucion_act));//para el ploteo.
                if self.solucion_act.f_obj < self.solucion_min.f_obj {
                    self.solucion_min = self.solucion_act.clone();//aqui mantenemos la minima.
                }

            } else {
                intentos = intentos + 1;
                self.solucion_act = s;
                if intentos == 2*TAMLOTE {
                    return r/(c as f64)
                }
            }
        }
        r = r/(TAMLOTE as f64);

        r
    }

    pub fn aceptacion_umbrales(&mut self) {
        let mut p: f64 = 0.0;
        let mut q: f64 = f64::INFINITY;
        //let mut intentos = 0;

        while self.temp > EPSILON {
            let mut p_prim = q;
            while p <= p_prim {
                p_prim = p;
                p = self.calcula_lote();
                q = p.clone();
                /*intentos = intentos + 1;
                if intentos > 100 {
                    intentos = 0;
                    break;
                }*/

            }
            self.temp = self.temp * PHI;
            //println!("{}", self.temp);
        }
    }

    fn temp_inicial(&mut self){
        let mut solucion_temp = self.solucion_temp.clone();
        let mut p1 = self.porcent_acep(solucion_temp);
        let mut p = p1.0;
        solucion_temp = p1.1;
        let mut temp_1: f64= 0.0;
        let mut temp_2: f64= 0.0;

        if (P - p).abs() <= EPSILON_P {
            //NADA
        } else if p < P {
            while p < P {
                self.temp = 2.0 * self.temp;
                p1 = self.porcent_acep(solucion_temp);
                p = p1.0;
                solucion_temp = p1.1;
            }
            temp_1 = self.temp/2.0;
            temp_2 = self.temp.clone();
        } else {
            while p > P {
                self.temp = self.temp/2.0;
                p1 = self.porcent_acep(solucion_temp);
                p = p1.0;
                solucion_temp = p1.1;
            }
            temp_1 = self.temp.clone();
            temp_2 = self.temp*2.0;
        }
        self.busqueda_binaria(temp_1, temp_2,solucion_temp);
    }

    fn porcent_acep(&mut self,mut solucion_temp: Solucion<'a>) -> (f64,Solucion<'a>){
        let mut c: f64 = 0.0;
        let mut s = solucion_temp;

        let len: usize = self.solucion_act.ciudades_solucion.len();

        for _i in 1..N {
            let a = self.rng.gen_range(0,len);//Aqui todo se vuelve no determinista.
            let b = self.rng.gen_range(0,len);//Aqui todo se vuelve no determinista.
            let mut s_prim = s.clone();
            s_prim.vecino(&a, &b);
            if s_prim.f_obj <= (s.f_obj + self.temp) {
                c = c + 1.0;
            }
            s = s_prim;
        }
        solucion_temp = s;
        (c/(N as f64),solucion_temp)
    }

    fn busqueda_binaria(&mut self, t1: f64, t2: f64,mut solucion_temp: Solucion<'a>) {
        let tm = (t1 + t2)/2.0;
        if (t2 - t1) < EPSILON_T {
            self.temp = tm;
        } else {
            let p1 = self.porcent_acep(solucion_temp);
            let p = p1.0;
            solucion_temp = p1.1;

            if (P - p).abs() <= EPSILON_P {
                self.temp = tm;
            } else if p > P {
                self.busqueda_binaria(t1,tm,solucion_temp);
            }else {
                self.busqueda_binaria(tm,t2,solucion_temp);
            }
        }
    }


}
