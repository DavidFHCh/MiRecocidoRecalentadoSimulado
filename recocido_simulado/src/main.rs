extern crate recocido_simulado;
extern crate config;
extern crate time;

use recocido_simulado as rs;
use rs::recocido::RecocidoSimulado;
use rs::conexion_bd::get_ciudades;
use rs::structs::ciudad::Ciudad;
use config::{Config, File, FileFormat, Value};
use time::{PreciseTime};
use std::env;
use std::fs::File as FsFile;
use std::io::prelude::*;
use std::path::Path;
use std::error::Error;


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
    let ciudades = get_ciudades().unwrap();
    c.merge(File::new("Ajustes", FileFormat::Toml).required(true)).expect("NO HAY ARCHIVO DE CONFIGURACION 'Ajustes.toml'");
    let semillas: Vec<u32> = to_u32_vec(c.get_array("semillas").expect("No hay lista de semillas declarada en Ajustes.toml"));
    let conj_ciudades = to_usize_vec(c.get_array("ciudad_ids").expect("No hay lista de ids de ciudades declarada en Ajustes.toml"));
    let print = c.get_bool("print").expect("No hay opcion print declarada en Ajustes.toml");

    let file_name: String = match env::args().nth(1) {
        Some(name) => name,
        None       => if print {panic!("No hay archivo de salida.")} else {"".to_string()}
    };
    let conj_ciudades_ref = v_usize_to_ciudades(conj_ciudades,&ciudades);
    let exec_start = PreciseTime::now();
    for semilla in semillas {
        let now = PreciseTime::now();
        let mut c_ciudades = conj_ciudades_ref.clone();
        let mut recocido = RecocidoSimulado::new(
            c_ciudades,
            [semilla,semilla*2,semilla*3,semilla*5]
        );
        recocido.aceptacion_umbrales();
        let sol_min = recocido.solucion_min;
        if sol_min.factible {
            factibles = factibles + 1;
            println!("-------------------------");
            println!("semilla = {}",semilla);
            print!("[");
            for ciudad in sol_min.ciudades_solucion {
                print!("{}, ",ciudad.ciudad_id);
            }
            println!("]")
        }
        println!("Tiempo de ejecucion: {}", now.to(PreciseTime::now()));
        println!("-------------------------");
    }
    println!("Tiempo de ejucucion total: {}", exec_start.to(PreciseTime::now()));
    println!("numero de soluciones factibles: {}",factibles);
}
