use std::time::Instant;                 // Librería estándar para medir el tiempo

// Defino algoritmo Búsqueda Binaria
fn busqueda_binaria(datos: &Vec<i32>, objetivo: i32) -> Option<usize> {

    let mut izquierda = 0;                      // Primer índice del array
    let mut derecha = datos.len() - 1;          // Último índice del array

    while izquierda <= derecha {                                // Continúo buscando mientras el rango sea válido

        let medio = izquierda + (derecha - izquierda) / 2;      // Calculo la posición del elemento central
        if datos[medio] == objetivo {                           // Si encontré el elemento, devuelvo su posición
            return Some(medio);
        }
        if datos[medio] < objetivo {                            // Si el objetivo es mayor, busco en la mitad derecha
            izquierda = medio + 1;
        }
        else {                                                  // Si el objetivo es menor, busco en la mitad izquierda
            if medio == 0 {                                     // Evito un underflow cuando medio vale 0
                break;
            }
            derecha = medio - 1;
        }
    }
    None                                                            // Si el elemento no existe, devuelvo None

}

fn main() {

    // Hacemos un array ordenado de 5000 números (0 al 4999)
    let datos: Vec<i32> = (0..5000).collect();
    // Número que queremos buscar
    let objetivo = 4321;
    // Arrancamos a medir el tiempo
    let inicio = Instant::now();
    // Ejecutamos la búsqueda muchas veces para obtener una medición más precisa y que los numeros no sean muy bajos
    for _ in 0..100_000 {
        busqueda_binaria(&datos, objetivo);
    }
    // Finalizamos la medición
    let duracion = inicio.elapsed();
    // Ejecutamos una vez más para mostrar el resultado encontrado
    match busqueda_binaria(&datos, objetivo) {
        Some(posicion) => println!("Elemento encontrado en la posición {}", posicion),
        None => println!("Elemento no encontrado"),
    }

    // Muestra las mediciones de tiempo del algoritmo
    println!("{:?}", duracion);
}

