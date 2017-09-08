extern crate rusqlite;
use std::path::Path;
use self::rusqlite::Connection;
use std::collections::HashMap;
use structs::ciudad::Ciudad;
use structs::conexion::Conexion;

const NUM_CIUDADES: i64 = 1097;


fn getCiudades() -> Result<Vec<Ciudad>, rusqlite::Error> {
    let path_db = Path::new("../../resources/world.db");
    let conexion = sqlite::open(path_db);
    let mut ciudades = Vec::with_capacity(NUM_CIUDADES);

    let mut consulta = conexion.prepare("SELECT * FROM cities;").expect("NO SE PUDO COMPLETAR LA CONEXION.")

}

//read this for reference http://blackbeam.org/doc/mysql/index.html
