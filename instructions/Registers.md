Registers are 32 bits (4 bytes) and cannot be accessed currently by a u32 number, as per
assemblers current spec, this will change soon. The largest number that can be directly put into a register by a single operation:
```
LOADIMM FFF R1 (load max pos number, 4096)

```


# R0 
always 0
# R1 - R11 
General purpose registers 
# R12 
Frame Pointer
# R13 
Stack pointer
Stack has a decending pattern, and the stack pointer keeps track of the top (decending part) of the stack
starts at the top of RAM (which is FF, or u8 max.) Is popped off top by the return instruction
# R14
Link Registor
# P15 
Program counter