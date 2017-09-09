extern crate rusqlite;
use std::path::Path;
use self::rusqlite::Connection;
use std::collections::HashMap;
use structs::ciudad::Ciudad;
use structs::conexion::Conexion;

const NUM_CIUDADES: usize = 1098;

/// Regresa un Result que contiene un vector de ciudades o un error de rusqlite.
///
/// # Examples
/// ```
/// extern crate recocido_simulado;
/// use recocido_simulado::conexion_bd::get_ciudades;
/// fn main() {
///     let ciudades = get_ciudades().unwrap();
///     assert_eq!(ciudades.len(),1099);
/// }
/// ```
pub fn get_ciudades() -> Result<Vec<Ciudad>, rusqlite::Error> {
    let path_db = Path::new("/home/david/Documents/HOC/Proyecto1/MiRecocidoRecalentadoSimulado/resources/world.db");
    let conexion = Connection::open(path_db).unwrap();
    let mut ciudades = Vec::with_capacity(NUM_CIUDADES);

    let mut consulta = conexion.prepare("SELECT * FROM cities;").expect("NO SE PUDO COMPLETAR LA CONEXION.");

    let mut consulta2 = conexion.prepare("SELECT * FROM connections").expect("NO SE PUDO COMPLETAR LA CONEXION 2.");

    ciudades.push(
        Ciudad {
            ciudad_id: 0,
            ciudad_nom: "Aldeeran".to_string(),
            pais: "Desconocido".to_string(),
            poblacion: 0,
            latitud: 0.0,
            longitud: 0.0,
            vecinos: HashMap::new(),
        }
    );

    let c_it = consulta.query_map(&[], |renglon| {
        Ciudad {
            ciudad_id: renglon.get(0),
            ciudad_nom: renglon.get(1),
            pais: renglon.get(2),
            poblacion: renglon.get(3),
            latitud: renglon.get(4),
            longitud: renglon.get(5),
            vecinos: HashMap::new(),
        }
    }).unwrap();

    let con_it = consulta2.query_map(&[], |renglon| {
        Conexion{
            ciudad1: renglon.get(0),
            ciudad2: renglon.get(1),
            distancia: renglon.get(2),
        }
    }).unwrap();

    for ciudad in c_it {
        let ciudad_1 = ciudad.unwrap();
        ciudades.push(ciudad_1);
    }

    for arista in con_it {
        let aris = arista.unwrap();
        let id1 = aris.ciudad1;
        let id2 = aris.ciudad2;
        let dis = aris.distancia;
        ciudades[id1 as usize].vecinos.insert(id2,dis);
        ciudades[id2 as usize].vecinos.insert(id1,dis);
    }

    Ok(ciudades)

}
