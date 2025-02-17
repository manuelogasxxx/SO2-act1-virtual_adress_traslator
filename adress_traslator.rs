//coded by Manuel Fernández Mercado FCC BUAP
//Marco Eduardo Baéz Gonzales FCC BUAP
//Eduardo Ulises Estrada Gonzales FCC BUAP
use std::process;
use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;

struct Memory{ //struct to store memory information
    physic: u32,    //Physic memory (bytes)
    vm: u32,      //Virtual memory (bytes)
    page_size: u32, // Page size (bytes)
    pages: u32,     //Total number of pages
    frames: u32,     //Total numbers of marks
    offset_bits: u32,     //necesary bits to represent # page_size
    pages_bits: u32,
    frame_bits: u32,     //necesary bits to represent marks
}
//constructor fot the struct 
fn build_memory(page_size : u32, physic : u32, vm : u32)->Memory{
    let frames = physic / page_size;
    let pages = vm/page_size;
    Memory{
        physic,
        vm,
        page_size,
        pages,
        frames,
        offset_bits: page_size.trailing_zeros(),
        pages_bits: pages.trailing_zeros(),
        frame_bits: frames.trailing_zeros(),
    }
}

//important functions using bit manipulation
fn is_power_of_two(n:u32)->bool{
    return n>0 && (n&(n-1))==0;
}

/*
this function make a shift' 1's consecutive mask 
starting from the lsb 
*/
fn make_low_mask(shift:u32)->u32{
    if shift>32{
        panic!("Unable to make that mask :(");
    }
    return (1<< shift)-1;
}

/*
this function checks if the given memory has the correct values
*/

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

/*
read a file and store each line into an Vec<u32>
*/

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


//shows all the memory info

fn show_vm_info(vm:u32, memory: &Memory, pag_table: &Vec<u32>){
    let vm_low = make_low_mask(memory.offset_bits) & vm; 
    let vm_high = vm>>memory.offset_bits;
    
    println!("\n|---Datos de la pagina\n");
    println!("Dirección virtual");
    println!("{} , {:0width$b}",vm, vm, width=(memory.offset_bits+memory.pages_bits) as usize);
    println!("Desplazamiento");
    println!("{} , {:0width$b}",vm_low, vm_low,width=(memory.offset_bits) as usize );
    println!("No. de Pagina");
    println!("{} , {:0width$b}",vm_high, vm_high,width=(memory.pages_bits) as usize );

    //ahora busco la página vm_high y le saco la info
    if (vm_high+3) as usize > pag_table.len(){
        println!("Page does not exist in the pagination table");
    }
    else{
        let aux = pag_table[(3+vm_high) as usize];
        //let aux1 = aux<<memory.offset_bits;
        //let aux2 = aux1+vm_low;

        let phs_low =make_low_mask(memory.frame_bits) & aux;
        let phs_high = aux>>memory.frame_bits;

        let mut aux1= phs_low<<memory.offset_bits;
        aux1= aux1+vm_low;

        //corroboro las cosillas
        if(pag_table[(3+vm_high) as usize] ==0 || phs_high & 1 ==0 ){
            println!("Fallo de página");
        }
        else{
        println!("Dirección Física");
        println!("{} , {:0width$b}",aux1, aux1, width=(memory.frame_bits + memory.offset_bits) as usize);
        println!("Marco de página");
        println!("{} , {:0width$b}",phs_low, phs_low, width=(memory.frame_bits) as usize);
        println!("Bits de control");
        println!("{} , {:0width$b}",phs_high, phs_high, width=5);
        bit_control_info(phs_high);
        }

        
    }
}

fn bit_control_info(aux:u32){
    let mut mask:u32=16;
    if mask & aux ==0{
        //5to bit
        print!("Caché inhabilitado-");
    }
    else{
        print!("Caché habilitado-");
    }
    mask>>=1;
    if mask & aux ==0{
        //4to bit
        print!("No referida-");
    }
    else{
        print!("Referida-")
    }
    mask>>=1;
    if mask & aux ==0{
        print!("No modificada-");
    }
    else{
        print!("Modificada-");
    }
    mask>>=1;
    if mask & aux ==0{
        print!("Permiso de lectura/escritura-");
    }
    else{
        print!("Permiso de solo lectura-");
    }
    mask>>=1;
    if mask & aux ==0{
        println!("Ausente");
    }
    else{
        println!("Presente");
    }
}

fn show_memory_info(m: &Memory){
    println!("|---NOW we are working with (Bytes)->");
    println!("Physical Memory: {}  | bits: {}",m.physic,m.frame_bits + m.offset_bits);
    println!("Virtual  Memory: {}  | bits: {}",m.vm,m.pages_bits+ m.offset_bits);
    println!("Page size: {}  | bits: {}",m.page_size, m.offset_bits);
    println!("|---SO there are ->");
    println!("# Frames: {} | bits: {}",m.frames,m.frame_bits );
    println!("# Pages: {}  | bits: {}",m.pages, m.pages_bits);
}

fn main(){
   let filename = "pruebita.txt";
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
    
    let mut memory = build_memory(numbers[0],numbers[1],numbers[2]);
    show_memory_info(&memory);
    
    loop {
        println!("\nIngrese el valor decimal de una dirección virtual o (salir)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        if input == "salir" {
            break;
        }
        match input.parse::<u32>() { 
            Ok(choice) => show_vm_info(choice, &memory,&numbers),
            Err(e) => println!("Error: {}", e), 
        }
    }
}