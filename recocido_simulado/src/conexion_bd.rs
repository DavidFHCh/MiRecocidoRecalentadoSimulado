extern crate rusqlite;
use std::path::Path;
use self::rusqlite::Connection;
use std::collections::HashMap;
use structs::ciudad::Ciudad;
use structs::conexion::Conexion;

const NUM_CIUDADES: usize = 1097;

/// Regresa un Result que contiene un vector de ciudades o un error de rusqlite.
///
/// # Examples
/// ```
/// extern crate recocido_simulado;
/// use recocido_simulado::conexion_bd::get_ciudades;
/// fn main() {
///     let ciudades = get_ciudades().unwrap();
///     assert_eq!(ciudades.len(),1098);
/// }
/// ```
pub fn get_ciudades() -> Result<Vec<Ciudad>, rusqlite::Error> {
    let path_db = Path::new("/home/david/Documents/HOC/Proyecto1/MiRecocidoRecalentadoSimulado/resources/world.db");
    let conexion = Connection::open(path_db).unwrap();
    let mut ciudades = Vec::with_capacity(NUM_CIUDADES);

    let mut consulta = conexion.prepare("SELECT * FROM cities;").expect("NO SE PUDO COMPLETAR LA CONEXION.");

    let c_it = consulta.query_map(&[], |renglon| {
        Ciudad {
            ciudad_id: renglon.get(0),
            ciudad_nom: renglon.get(1),
            pais: renglon.get(2),
            poblacion: renglon.get(3),
            latitud: renglon.get(4),
            longitud: renglon.get(5),
        }
    }).unwrap();

    

    for ciudad in c_it {
        let ciudad_1 = ciudad.unwrap();
        ciudades.push(ciudad_1);
    }

    Ok(ciudades)

}

//read this for reference http://blackbeam.org/doc/mysql/index.html
