extern crate recocido_simulado;
extern crate config;
extern crate time;
extern crate gnuplot;

use gnuplot::{Figure, Caption, Color};
use recocido_simulado as rs;
use rs::recocido::RecocidoSimulado;
use rs::conexion_bd::get_ciudades;
use rs::structs::ciudad::Ciudad;
use config::{Config, File, FileFormat, Value};
use time::{PreciseTime};
use std::f64::INFINITY;
//use std::env;
//use std::fs::File as FsFile;
//use std::io::prelude::*;
//use std::path::Path;
//use std::error::Error;


fn to_usize_vec(values: Vec<Value>) -> Vec<usize> {
    let mut v = Vec::with_capacity(values.len());
    for vs in values.clone() {
        v.push(vs.into_int().expect("Error convirtiendo a i64") as usize);
    }
    v
}

fn to_u32_vec(values: Vec<Value>) -> Vec<u32> {
    let mut v = Vec::with_capacity(values.len());
    for vs in values.clone() {
        v.push(vs.into_int().expect("Error convirtiendo a i64") as u32);
    }
    v
}

fn v_usize_to_ciudades(ciudades_id: Vec<usize>,ciudades: &Vec<Ciudad>) -> Vec<&Ciudad>{
    let mut v = Vec::new();
    for id in ciudades_id {
        let ciudad = &ciudades[id];
        v.push(ciudad);
    }
    v
}


fn main() {
    let mut c = Config::new();
    let mut factibles: usize = 0;
    let mut totales:usize = 0;
    let mut f_min:f64 = INFINITY;
    let mut sem_min = 0;
    let ciudades = get_ciudades().unwrap();
    c.merge(File::new("Ajustes", FileFormat::Toml).required(true)).expect("NO HAY ARCHIVO DE CONFIGURACION 'Ajustes.toml'");
    let semillas: Vec<u32> = to_u32_vec(c.get_array("semillas").expect("No hay lista de semillas declarada en Ajustes.toml"));
    let conj_ciudades = to_usize_vec(c.get_array("ciudad_ids").expect("No hay lista de ids de ciudades declarada en Ajustes.toml"));
    let print = c.get_bool("print").expect("No hay opcion print declarada en Ajustes.toml");

    let conj_ciudades_ref = v_usize_to_ciudades(conj_ciudades,&ciudades);
    let exec_start = PreciseTime::now();
    for semilla in semillas {
        totales = totales + 1;
        let now = PreciseTime::now();
        let c_ciudades = conj_ciudades_ref.clone();
        let mut recocido = RecocidoSimulado::new(
            c_ciudades,
            [semilla,semilla*2,semilla*3,semilla*5]
        );
        recocido.aceptacion_umbrales();
        let mut sol_min = recocido.solucion_min;
        let len = sol_min.ciudades_solucion.len();
        sol_min.actualizar_factibilidad(&len);
        if print /*&& sol_min.factible*/ {
            let mut x = Vec::new();
            let mut y = Vec::new();
            let mut i = 0.0;
            for sol in recocido.sols.soluciones {
                y.push(sol.f_obj);
                x.push(i);
                i = i + 1.0;
            }
            let mut fg = Figure::new();
            let str1 = format!("A line,{},{}",semilla,sol_min.factible).to_owned();
            let str1_sliced: &str = &str1[..];
            fg.axes2d().lines(&x, &y, &[Caption(str1_sliced), Color("blue")]);
            fg.show();
        }


        if sol_min.factible {
            if sol_min.f_obj < f_min {
                f_min = sol_min.f_obj.clone();
                sem_min = semilla.clone();
            }
            factibles = factibles + 1;
            println!("-------------------------");
            println!("semilla = {}",semilla);
            println!("funcion de costo : {}", sol_min.f_obj);
            print!("[");
            for ciudad in sol_min.ciudades_solucion {
                print!("{}, ",ciudad.ciudad_id);
            }
            println!("]");
        }
        println!("Tiempo de ejecucion: {}", now.to(PreciseTime::now()));
        println!("-------------------------");
    }
    println!("Tiempo de ejucucion total: {}", exec_start.to(PreciseTime::now()));
    println!("numero de soluciones factibles: {}",factibles);
    println!("numero de semillas totales: {}",totales);
    println!("Semilla: {}, f_obj_min: {}",sem_min,f_min);
}
