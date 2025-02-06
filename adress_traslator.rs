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
    physic: u32,    //Physic memory (bytes)
    vm: u32,      //Virtual memory (bytes)
    page_size: u32, // Page size (bytes)
    pages: u32,     //Total number of pages
    frames: u32,     //Total numbers of marks
    page_bits: u32,     //necesary bits to represent # page_size
    frame_bits: u32,     //necesary bits to represent marks
}

//constructor of "Memory"


//no se verifican diviciones entre 0
fn build_memory(page_size : u32, vm : u32, physic : u32)->Memory{
    let frames = physic / page_size;
    Memory{
        physic,
        vm,
        page_size,
        pages: vm/page_size,
        frames,
        page_bits: page_size.leading_zeros(),
        frame_bits: frames.leading_zeros(),
    }
}


//important functions using bit manipulation

fn is_power_of_two(n:u32)->bool{
    return n>0 && (n&(n-1))==0;
}

fn memory_check(v: &Vec<u32>)->bool{
    if !is_power_of_two(v[0]) || !is_power_of_two(v[1]) || !is_power_of_two(v[2]){
        println!("You are not using power of two memory sizes ;(");
        return false;
    }
    if v[0]>v[1] || v[0]>v[2]{
        println!("Page size enormously big ;c");
        return false;
    }
    if v[0]*v[1]*v[2] ==0{
        println!("You are using zero(s) as memory sizes :z")
    }
    return true;
}

fn read_numbers_from_file(filename: &str) -> io::Result<Vec<u32>> {
    let path = Path::new(filename);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut numbers = Vec::new();
    for line in reader.lines() {
        let line = line?; // Desempaqueta la línea o retorna un error
        if let Ok(num) = line.trim().parse::<u32>() {
            numbers.push(num);
        }
    }
    Ok(numbers)
}


fn main(){
   let filename = "pruebita.txt";
   //let numbers = read_numbers_from_file(filename).expect("Error al leer el archivo");
   let numbers: Vec<u32> = match read_numbers_from_file(filename) {
        Ok(nums) => nums,
        Err(e) => {
            eprintln!("Error al leer el archivo: {}", e);
            process::exit(1);
        }
    };

    if !memory_check(&numbers) {
        return;
    }

    //del índice 3 en adelante ya es la tabla de páginas

    //función para contruir la memoria
    println!("{:?}",numbers);
    let mut memory = build_memory(numbers[0],numbers[1],numbers[2]);
    //en este punto ya todo está comprobado y listo

    




}


//por el momento no va a manejar todos los errores para irnos más facil xd, cuando termine ya le pongo las comprobaciones


//ya leo del archivo 


//definir máximos para números de páginas, marcos 