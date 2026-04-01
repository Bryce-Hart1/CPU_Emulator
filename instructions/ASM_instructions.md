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

Adds R2 into R1 and sets R2 to 0

0100 0001 SUB R1 (4 bits ) R2 (4 bits)
-----------------------------------------------


0100 0010 DIV R1 (4 bits ) R2 (4 bits)
-----------------------------------------------


0100 0011 MULTI R1 (4 bits ) R2 (4 bits)
-----------------------------------------------


0100 0100 OR 
-----------------------------------------------


0100 0101 AND 
-----------------------------------------------


0100 0110 !OR (XOR)
-----------------------------------------------


0100 0111 NOT
-----------------------------------------------

```


## Data Movement (2 bytes)
Includes 6 instructions: MOVE, MOVE&CLR, LOAD, STORE, PUSH, POP
```
0100 1000 MOVE
-----------------------------------------------


0100 1001 MOVE&CLR
-----------------------------------------------


0100 1010 LOAD
-----------------------------------------------


0100 1011 STORE
-----------------------------------------------


0100 1100 PUSH
-----------------------------------------------


0100 1101 POP
-----------------------------------------------



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


```