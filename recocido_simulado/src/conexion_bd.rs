extern crate rusqlite;
use std::path::Path;
use self::rusqlite::Connection;
use structs::ciudad::Ciudad;
use structs::conexion::Conexion;

const NUM_CIUDADES: usize = 1093;

/// Regresa un Result que contiene un vector de ciudades o un error de rusqlite.
///
/// # Examples
/// ```
/// extern crate recocido_simulado;
/// use recocido_simulado::conexion_bd::get_ciudades;
/// fn main() {
///     let ciudades = get_ciudades().unwrap();
///     assert_eq!(ciudades.len(),1093);
/// }
/// ```
pub fn get_ciudades<'a>() -> Result<Vec<Ciudad>, rusqlite::Error> {
    let path_db = Path::new("../resources/tsp.db");
    let conexion = Connection::open(path_db).unwrap();
    let mut ciudades = Vec::with_capacity(NUM_CIUDADES);

    let mut consulta2 = conexion.prepare("SELECT * FROM connections").expect("NO SE PUDO COMPLETAR LA CONEXION 2.");

    let con_it = consulta2.query_map(&[], |renglon| {
        Conexion{
            ciudad1: renglon.get(0),
            ciudad2: renglon.get(1),
            distancia: renglon.get(2),
        }
    }).unwrap();

    let mut ceros = Vec::with_capacity(NUM_CIUDADES);
    for _i in 0..NUM_CIUDADES {
        ceros.push(0.0);
    }

    let mut m_adyacencias = Vec::with_capacity(NUM_CIUDADES);

    for _v in 0..NUM_CIUDADES {
            m_adyacencias.push(ceros.clone());
    }

    for arista in con_it {
        let aris = arista.unwrap();
        let id1 = aris.ciudad1;
        let id2 = aris.ciudad2;
        m_adyacencias[id1 as usize][id2 as usize] = aris.distancia.clone();
        m_adyacencias[id2 as usize][id1 as usize] = aris.distancia.clone();
    }

    for i in 0..NUM_CIUDADES {
        ciudades.push(
            Ciudad {
                ciudad_id: i as i32,
                adyacencias: m_adyacencias[i].clone(),
            }
        );
    }

    Ok(ciudades)

}
