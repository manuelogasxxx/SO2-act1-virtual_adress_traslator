# Virtual Adress Traslator

This project is the 2nd practice of the curse Operating Systems II, by itselft it is an introduction to the topic "pagination"

## Objetives
Emulate the way an Operating System makes the traslate from an virtual adress to a physical adress, trying to use the less operations and using bit manipulations.

## Before Start 
In order to run the project you only need the Rust compiler installed in your computer. The code was only tested on a Windows machine, take that in count if you want to run it on Linux 
It is important to introduce some value concepts for this implementation.

**Memory form:**
We can think the memory as an two individual sectors (HIGH-LOW), it is important beacause each part of the physical and virtual adress has his own meaning.

**VM adress** 
-> HIGH: correspond to the OFFSET.
->  LOW: correspond to the "page number". 

**Physical Adress**
-> HIGH: represent the control bits
->  LOW: represent the offset

**Control bits**
This part has a default size of 5 bits. Let's numerate each bit from the most significant to the less significant. (add here the anotations aboout the control bits)


**How the program runs? (a resume way)


NOTE: there is no default size of the entire adress, it depends of the physical, virtual and page memory sizes 
