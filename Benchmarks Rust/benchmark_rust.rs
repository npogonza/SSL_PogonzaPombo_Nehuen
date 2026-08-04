use std::time::Instant;             // Libreria estandar para medir el tiempo

// Defino algoritmo Bubble Sort

fn bubble_sort(datos: &mut Vec<i32>) {
    let n = datos.len();                    // Obtengo la cantidad de elementos del array a ordenar 

    for i in 0..n {                         // Recorro el array completo
        for j in 0..(n - 1 - i) {           // En cada pasada el ultimo elemento ya esta ordenado, se va achicando el rango
            if datos[j] > datos[j + 1] {    // Comparo dos elementos consecutivos del array
                datos.swap(j, j + 1);       // Si estan en el orden incorrecto, los intercambio
            }
        }
    }
}

fn main() {

let mut datos: Vec<i32> = (0..5000).rev().collect();     // Hacemos un array de 5000 numeros desordenados, del 4999 al 0

println!("Primeros 10 antes: {:?}", &datos[..10]);      // Muestra como se encontraba el array al principio

let inicio = Instant::now();        // Arrancamos a medir el tiempo

bubble_sort(&mut datos);            // Algoritmo bubble sort

let duracion = inicio.elapsed();    // Finalizamos la medicion

println!("Primeros 10 despues: {:?}", &datos[..10]);    // Muestra como se encuentra el array al final

println!("{:?}", duracion);         // Muestra las mediciones de tiempo del algoritmo

}