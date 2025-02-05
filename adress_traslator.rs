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


struct Memory{
    physic: u32,
    virtual: u32,
    pages: u32,
    marks: u32,
}

//important functions using bit manipulation

fn is_power_of_two(n:u32)->bool{
    return n>0 && (n&(n-1))==0;
}



fn main(){
    
}