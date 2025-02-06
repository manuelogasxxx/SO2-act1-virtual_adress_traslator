//coded by Manuel Fernández Mercado FCC BUAP 

/*
Paso 1: obtener los datos del archivo de texto *pendiente*

Paso 2: saber cuantos marcos y páginas hay con su respectivo 
        tamaño en bits, se obtiene el desplazamiento,

Paso 3: El programa lee un número y saca su numéro de pagina

*/



/*
Nota: Todos los tamaños de la memoria están en bytes
*/
use std::process;
use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;

struct Memory{
    physic: u32,
    virt: u32,
    pages: u32,
    marks: u32,
}


//important functions using bit manipulation

fn is_power_of_two(n:u32)->bool{
    return n>0 && (n&(n-1))==0;
}

fn read_numbers_from_file(filename: &str) -> io::Result<Vec<i32>> {
    let path = Path::new(filename);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut numbers = Vec::new();

    for line in reader.lines() {
        let line = line?; // Desempaqueta la línea o retorna un error
        if let Ok(num) = line.trim().parse::<i32>() {
            numbers.push(num);
        }
    }

    Ok(numbers)
}


fn main(){
    //de momento aún no hago las comprobaciones
    /*let mut memory = Memory{
        physic: 262144,
        virtual: 1048576,
        pages: 2048,
        marks: 512,
    }   */
   let filename = "prueba.txt";
   let numbers: Vec<i32> = match read_numbers_from_file(filename) {
        Ok(nums) => nums,
        Err(e) => {
            eprintln!("Error al leer el archivo: {}", e);
            process::exit(1);
        }
    };

    println!("{:?}",numbers);




}