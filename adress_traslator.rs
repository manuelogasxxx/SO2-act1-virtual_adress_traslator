//coded by Manuel Fernández Mercado FCC BUAP 
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
    offset_bits: u32,     //necesary bits to represent # page_size
    pages_bits: u32,
    frame_bits: u32,     //necesary bits to represent marks
}

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

fn make_low_mask(shift:u32)->u32{
    if shift>32{
        panic!("Unable to make that mask :(");
    }
    return (1<< shift)-1;
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
        let line = line?; 
        if let Ok(num) = line.trim().parse::<u32>() {
            numbers.push(num);
        }
    }
    Ok(numbers)
}

fn show_vm_info(vm:u32, memory: &Memory, pag_table: &Vec<u32>){
    let vm_low = make_low_mask(memory.offset_bits) & vm; 
    let vm_high = vm>>memory.offset_bits;
    
    println!("\n|---Page info\n");
    println!("Virtual Adress");
    println!("{} , {:0width$b}",vm, vm, width=(memory.offset_bits+memory.pages_bits) as usize);
    println!("Offset");
    println!("{} , {:0width$b}",vm_low, vm_low,width=(memory.offset_bits) as usize );
    println!("# Page");
    println!("{} , {:0width$b}",vm_high, vm_high,width=(memory.pages_bits) as usize );

    //ahora busco la página vm_high y le saco la info
    if (vm_high+3) as usize > pag_table.len(){
        println!("Page does not exist in the pagination table");
    }
    else{
        let aux = pag_table[(3+vm_high) as usize];
        println!("Physical Adress");
        println!("{} , {:b}",aux, aux);
        let phs_low= make_low_mask(memory.frame_bits)& aux;
        let phs_high = aux>>memory.frame_bits;
        println!("Frame");
        println!("{} , {:b}",phs_low, phs_low);
        println!("Control Bits");
        println!("{} , {:05b}",phs_high, phs_high);
        //mando a llamar una función para los datos de los bits de control
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
            eprintln!("Error while reading the archive :8 {}", e);
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