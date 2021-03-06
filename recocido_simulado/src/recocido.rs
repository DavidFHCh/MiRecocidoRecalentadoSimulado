extern crate rand;

use structs::{ciudad,solucion_lite,solucion,soluciones};
use self::rand::{XorShiftRng, SeedableRng, Rng};
use self::ciudad::Ciudad;
use self::solucion::Solucion;
use self::solucion_lite::SolucionLite;
use self::soluciones::Soluciones;
use std::f64;

static TAMLOTE: usize = 3000;
static EPSILON: f64 = 0.000004;
static EPSILON_P: f64 = 0.001;
static EPSILON_T: f64 = 0.001;
static PHI: f64 = 0.99;
static P: f64 = 0.95;
static N: usize = 1000;


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
        let ord = s_init.clone();
        rng.shuffle(&mut s_init);
        let s = Solucion::new(s_init,ord);
        let mut rs = RecocidoSimulado {
            temp: 7.0,
            solucion_act: s.clone(),
            solucion_min: s.clone(),
            solucion_temp: s.clone(),
            rng: rng,
            sols: Soluciones::new(),
        };
        rs.temp_inicial();
        rs
    }

    fn sweep(&mut self){
        let len = self.solucion_act.ciudades_solucion.len();
        for i in 0..len {
            for j in (i+1)..len {
                self.solucion_act.vecino(&i,&j);
                if self.solucion_act.f_obj < self.solucion_min.f_obj {
                    self.solucion_min = self.solucion_act.clone();//aqui mantenemos la minima.
                } else {
                    self.solucion_act.vecino(&j,&i);
                }
            }
        }
    }

    fn calcula_lote(&mut self) -> (f64,bool){
        let mut c: usize = 0;
        let mut r: f64 = 0.0;
        let f;
        let mut a: usize;
        let mut b: usize;
        //let mut c1: usize;
        //let mut d: usize;
        let mut intentos: usize = 0;

        let len: usize = self.solucion_act.ciudades_solucion.len();

        while c < TAMLOTE {

            let s = self.solucion_act.clone();
            a = self.rng.gen_range(0,len);//Aqui todo se vuelve no determinista.
            b = self.rng.gen_range(0,len);//Aqui todo se vuelve no determinista.
            //c1 = self.rng.gen_range(0,len);//Aqui todo se vuelve no determinista.
            //d = self.rng.gen_range(0,len);//Aqui todo se vuelve no determinista.
            self.solucion_act.vecino(&a,&b);
            //self.solucion_act.vecino(&c1,&d);
            if self.solucion_act.f_obj <= (s.f_obj + self.temp) {
                //println!("E:{}",self.solucion_act.f_obj);
                intentos = 0;
                c = c + 1;
                r = r + self.solucion_act.f_obj;
                self.sols.push(SolucionLite::new(&self.solucion_act));//para el ploteo.
                if self.solucion_act.f_obj < self.solucion_min.f_obj {
                    self.solucion_min = self.solucion_act.clone();//aqui mantenemos la minima.
                    self.sweep();
                }

            } else {
                intentos = intentos + 1;
                self.solucion_act = s;
                if intentos == 2*TAMLOTE {
                    return (r/(c as f64),true)
                }
            }
        }
        f = (r/(TAMLOTE as f64), false);

        f
    }

    pub fn aceptacion_umbrales(&mut self) {
        let mut p: f64 = 0.0;
        let mut q: f64 = f64::INFINITY;
        let mut intentos = 0;
        let mut ya_no_sigas = false;
        let mut p1;
        let mut no_se_lleno;

        while self.temp > EPSILON {
            let mut p_prim = q;
            while p <= p_prim {
                p_prim = p;
                p1 = self.calcula_lote();
                p = p1.0;
                no_se_lleno = p1.1;
                q = p.clone();
                intentos = intentos + 1;
                //println!("{}", intentos);
                if intentos > (TAMLOTE*10) || no_se_lleno {
                    intentos = 0;
                    ya_no_sigas = true;
                    break;
                }

            }
            if ya_no_sigas {
                break;
            }
            self.temp = self.temp * PHI;
            //println!("{}", self.temp);
        }
        self.sweep();
    }

    fn temp_inicial(&mut self){
        let mut solucion_temp = self.solucion_temp.clone();
        let mut p1 = self.porcent_acep(solucion_temp);
        let mut p = p1.0;
        solucion_temp = p1.1;
        let temp_1: f64;
        let temp_2: f64;

        if (P - p).abs() <= EPSILON_P {
            return;
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

        for _i in 0..N {
            let a = self.rng.gen_range(0,len);//Aqui todo se vuelve no determinista.
            let b = self.rng.gen_range(0,len);//Aqui todo se vuelve no determinista.
            let mut s_prim = s.clone();
            s_prim.vecino(&a, &b);
            if s_prim.f_obj <= (s.f_obj + self.temp) {
                c = c + 1.0;
            } else {
                s_prim.vecino(&b, &a);
            }
            s = s_prim;
        }
        solucion_temp = s;
        //println!("{}",c/(N as f64) );
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
