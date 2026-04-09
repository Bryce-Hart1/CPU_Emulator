# About these instructions
These instructions were designed the way *I wanted them* so if you ask yourself "Why is this instruction so long?
thats why. 
I designed these with a beginner in mind. I want to be able to show anyone this instruction set and them to be able to
remember without a table. 

## my simple rules when designing this set
1. No instruction goes over 10 characters 
2. All instructions must be ascii characters
3. Must be as clear as possible

# optcode rule
```
top 2 bytes
00 1 byte instruction
01 2 byte instruction
10 3 byte instructions

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

## Interupts (2 bytes)
```
0100 1100 INT (interupt)
-----------------------------------------------
Cuts to Interupt table In the bios. Keep in mind the CPU will only handle 
the interupt byte.
When CPU sees ->INT<- itll pass to the bios.
Next Byte:
possible second Byte:
INT 0A //writes to screen
INT 0B //takes from keyboard
INT 0C //takes from Disk

INT 0A
looks at R1, R2 for what to do next.
R1 : Char to write (takes mod of 255)
R2 : 
(top 3 blank bytes in reg ) _ _ _ _ _ _ _ _ 
                            ^       ^
                        Foreground Background
16 options for each
This number will also be modded by 255 to prevent errors


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
Unconditionally move the program counter to a new address

1000 0010 JMPIF0
-----------------------------------------------
Checks Zero flag in flags register 

1000 0011 JMPIF!0
-----------------------------------------------


1000 0100 CALL
-----------------------------------------------


1000 0101 JMPIFCRRY
-----------------------------------------------


1000 0110 JMPIFAULT
-----------------------------------------------

1000 0111 LOAD
-----------------------------------------------


1000 1000 STORE (reg) (padding) (8 bit addr (ram))
-----------------------------------------------
Stores requested value from register into ram at a 8 bit address



```
