extern crate mysql;

use mysql as ms;

struct Ciudad {
    ciudad_id: i32,
    ciudad_nom:String,
    pais:String,
    poblacion:i64,
    latitud:f64,
    longitud:f64,
}

struct conexion{
    ciudad1: i32,
    ciudad2: i32,
    distancia: f64,
}

fn getCiudades() -> Vec<Ciudad>{
    let pool = ms::Pool::new("mysql://root:password@localhost:3307").unwrap();
}

//read this for reference http://blackbeam.org/doc/mysql/index.html
