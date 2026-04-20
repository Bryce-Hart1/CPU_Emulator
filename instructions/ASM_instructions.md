# About these instructions
These instructions were designed the way *I wanted them* so if you ask yourself "Why is this instruction so long?
thats why. 
I designed these with a beginner in mind. I want to be able to show anyone this instruction set and them to be able to remember without a table, to the best of my ability. 

## my simple rules when designing this set
1. No instruction goes over 10 characters 
2. All instructions must be ascii characters
3. Must be as clear as possible
4. Instruction set must be flexible to be able to do any instruction that a modern CPU can carry out.

# opcode rule
```
top 2 bytes
00 1 byte instruction
01 2 byte instruction
10 3 byte instructions
11 6 byte instructions (Not yet implemented)

```
# Registers 
See ```Registers.md```
All 4 bits

# Instructions

## Basic (1 byte)
Includes 4 instructions: NOPERATION, HALT, RETURN, CLRFLAGS
```
0
0000 0000 NOPERATION
-----------------------------------------------

No operation

0000 0001 HALT
-----------------------------------------------

Stop CPU

0000 0010 RETURN
-----------------------------------------------

Return from subroutine pop Program counter off stack

0000 0011 CLRFLAGS
-----------------------------------------------

Clear all flags
```
## Arithmetic (2 bytes)
Includes 8 instructions: ADD, SUB, DIV, MULTI, OR, AND, !OR, NOT
```
0100 0000 ADD R1 (4 bits) R2 (4 bits)
-----------------------------------------------

Adds R2 into R1 and leaves r2 unchanged.


0100 0001 SUB R1 (4 bits ) R2 (4 bits)
-----------------------------------------------
Subtracts r2 from r1, and stores the result in r1, leaving 
r2 unchanged

0100 0010 DIV R1 (4 bits ) R2 (4 bits)
-----------------------------------------------
Divides r1 by r1, and stores the result in r1, leaving 
r2 unchanged


0100 0011 MULTI R1 (4 bits ) R2 (4 bits)
-----------------------------------------------
multiplies r1 by r2, and stores the result in r1, leaving
r2 unchanged


0100 0100 OR 
-----------------------------------------------
inclusive or of 2 operations
0 + 0 = 0
0 + 1 = 1
1 + 1 = 1
leaves value in reg2.

0100 0101 AND 
-----------------------------------------------
must have both bits flipped to have a 1
0 + 0 = 0
0 + 1 = 0
1 + 1 = 1

0100 0110 !OR (XOR) (reg1 (4 bytes)) (reg2 (4 bytes))
-----------------------------------------------
exclusive or must be different bits to have a one
Stored in r1 and does not clear r2
0 + 0 = 0
0 + 1 = 1
1 + 1 = 0

0100 0111 NOT (reg1 4 bytes) (4 byte buffer)
-----------------------------------------------
first 4 bytes contains a register, the last 4 can be discarded as a buffer
reverses bits in register 1.
0 = 1
1 = 0

```


## Data Movement (2 bytes)
Includes 4 instructions: MOVE, MOVE&CLR, LOAD, STORE, PUSH, POP
```
0100 1000 MOVE (reg1 (4 bytes)) (reg2 (4 bytes))
-----------------------------------------------
moves reg1 TO reg2. does not clear reg1.


0100 1001 MOVE&CLR (reg1 (4 bytes)) (reg2 (4 bytes))
-----------------------------------------------
moves reg1 TO reg2. does not clear reg1.


0100 1010 PUSH
-----------------------------------------------


0100 1011 POP
-----------------------------------------------



```

## Interrupts (2 bytes)
```
0100 1100 INT (interrupt)
-----------------------------------------------
Cuts to Interrupt table In the bios. Keep in mind the CPU will only handle 
the interrupt byte. Keep in mind these need called AFTER you have registers loaded, and 
the registers that you are loading are specific to the Instruction, but should be in order, 
example r1, r2 ... 
When CPU sees ->INT<- it'll pass to the bios.
Next Byte:
possible second Byte:
INT 0A //writes to screen
INT 0B //takes from keyboard
INT 0C //move user cursor 
INT 0D //takes from Disk
```
#### INT 0A
```
looks at R1, R2 for what to do next.
R1 : Char to write (takes mod of 255)
R2 : 
(top 3 blank bytes in reg ) _ _ _ _ _ _ _ _ 
                            ^       ^
                        Foreground Background
16 options for each
This number will also be modded by 255 to prevent errors

```
#### INT 0B 
```
Colors whole screen based on whats found in R1

R1 : (top 3 blank) _ _ _ _ _ _ _ _
                           ^
                           Background 

This overrides anything previously drawn, does not reset cursor

16 options for each.
```
### colors for bios
#### black 
```
00H
```
#### blue
```
01H
```
#### green
```
02H
```
#### bright blue (cyan-ish)
```
03H
```
#### red
```
04H
```
#### purple
```
05H
```
#### orange (brown-ish alt)
```
06H
```
#### white (light gray)
```
07H
```
#### grey
```
08H
```
#### bright blue
```
09H
```
#### bright green
```
0AH
```
#### bright cyan
```
0BH
```
#### bright red
```
0CH
```
#### bright purple
```
0DH
```
#### yellow
```
0EH
```
#### bright white
```
0FH
```

### INT 0C
```
```

### INT 0D 
```
Allows for a Interrupt to read from disk.
Takes a few arguments: 
Looks at R1, R2, and R3 for what to do next.
R1 How many bytes to read (no actual platter, so pick as many bytes as you want!)
R2 location to read from this is 0 based.
*If you want cell 18 for example:
LOADIMM R2 11 (in hex) 
R3 where to write in RAM
Same as Disk, also 0 based, but keep in mind there is only 256 locations, 
compared to the disks 2^16 (65536) locations.
*For both, if you write out of bounds, this will cause undefined behavior
```
### INT 0E
```
write to disk. Same rules apply from reading.
R1 How many bytes to transfer from RAM to disk
R2 location of RAM to read from 
R3 where to write on disk
```



## Load and Jumps (3 bytes)
Includes 7 instructions: LOADIMM, JMP, JMPIF0, JMPIF!0, CALL, JMPIFCRRY, JMPIFAULT
```
1000 0000 LOADIMM R1 (4 bits ) (12 bit addr)
-----------------------------------------------
same as LDI, loads an 12 bit address into R1. This is changed from the legacy design,
as there is no padding.

1000 0001 JMP 
-----------------------------------------------
Unconditionally move the program counter to a new address,
defined as a label.

1000 0010 JMPIF0
-----------------------------------------------
Checks Zero flag in flags register,
If it is 0, jumps to a label_

1000 0011 JMPIF!0
-----------------------------------------------
Checks Zero flag in flags register,
If it is not 0, jumps to a label_


1000 0100 CALL
-----------------------------------------------
Currently not implemented in this version of the assembler

1000 0101 JMPIFCRRY
-----------------------------------------------
If carry flag is set, jump to label_


1000 0110 JMPIFAULT
-----------------------------------------------
Currently not implemented

1000 0111 LOAD
-----------------------------------------------


1000 1000 STORE (reg) (padding) (8 bit addr (ram))
-----------------------------------------------
Stores requested value from register into ram at a 8 bit address
```

## Long Instructions (not yet implemented) (6 bytes)


# ASM rules
In every doc you see here, I may refer to the assembly files as em (Emulator Machine code) or assembly, or asm. These all mean the same thing.
## labels
labels are indicated in lines that begin with 
``` 
label_
```
these labels can be used to jump to a byte offset in the binary. *However,* if a program is longer then the max offset available in the fixed bit, instruction jumps is only 2^12-1, (4095), or FFF. Any offset larger then this pre designed size will throw an error