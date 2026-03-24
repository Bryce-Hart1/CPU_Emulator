/**
 * @author Designed by Bryce Hart March 22nd, 2026
 * A custom Assembly set language made by me. Translates the language seen in @related ASM_instructions.md
 * 
 * 
 */


#pragma once
#include <iostream>
#include <fstream>
#include <filesystem>
#include <map>
#include <vector>
#include <string>
#include <exception>
#include <bitset>
#include <functional>
#include <array>
#include <optional>
#include <limits>
#include <concepts>
#include <sstream>

namespace fs = std::filesystem;
using i8 = int8_t; //for instruction set
using usi8 = uint8_t;
using _byte = std::bitset<8>; //a bitset<8> namespace

std::map<std::string, i8> Register_Key = {
    {"R0", 0}, {"R1", 1}, {"R2", 2},  {"R3", 3},
    {"R4", 4}, {"R5", 5}, {"R6", 6},  {"R7", 7},
    {"R8", 8}, {"R9", 9}, {"R10", 10},{"R11", 11},
    {"R12", 12},{"R13", 13},{"R14", 14},{"R15", 15}
};

const std::string& asmPostfix = "a"; //choose the type of file that ASM will be read from 
const std::string& binPostfix = ".b"; //choose the type of file that "binary" will write to. include dot for now

const std::string& asmDir = "Asm";
const std::string& binDir = "Bin";
const int Instr_Size = 256;
const int nOfBasics = 4;
const int nOfArith = 8;
const int nOfDataMvm = 6;
const int nOfLoadAndJump = 7;

//for instruction set
std::array<std::string, Instr_Size> Instruction_Set; //set of actual instrucions as they come from strings;
std::map<std::string, std::bitset<8>> Instruction_Key; //actual hashmap of each instruction


void increment(_byte& bits);

i8 convertBitsetToByte(_byte bits);

enum class InstrType { BASIC, ARITH, DATA_MOV, LOAD_JUMP };

inline std::map<std::string, InstrType> Instruction_Types = {
    {"NOPERATION", InstrType::BASIC},  {"HALT",    InstrType::BASIC},
    {"RETURN",     InstrType::BASIC},  {"CLRFLAGS",InstrType::BASIC},

    {"ADD",  InstrType::ARITH}, {"SUB",   InstrType::ARITH},
    {"DIV", InstrType::ARITH}, {"MULTI", InstrType::ARITH},
    {"OR", InstrType::ARITH}, {"AND", InstrType::ARITH},
    {"!OR", InstrType::ARITH}, {"NOT", InstrType::ARITH},

    {"MOVE", InstrType::DATA_MOV}, {"MOVE&CLR", InstrType::DATA_MOV},
    {"LOAD", InstrType::DATA_MOV}, {"STORE", InstrType::DATA_MOV},
    {"PUSH", InstrType::DATA_MOV}, {"POP", InstrType::DATA_MOV},

    {"LOADIMM", InstrType::LOAD_JUMP}, {"JMP", InstrType::LOAD_JUMP},
    {"JMPIF0", InstrType::LOAD_JUMP}, {"JMPIF!0", InstrType::LOAD_JUMP},
    {"CALL", InstrType::LOAD_JUMP}, {"JMPIFCRRY", InstrType::LOAD_JUMP},
    {"JMPIFAULT", InstrType::LOAD_JUMP}
};

//see ASM_Instructions.md for explainations
inline void defineInstructions(std::array<std::string, Instr_Size>& arr){
    arr.at(0) = "NOPERATION";
    arr.at(1) = "HALT";
    arr.at(2) = "RETURN";
    arr.at(3) = "CLRFLAGS";
    //done with basics
    arr.at(4) = "ADD";
    arr.at(5) = "SUB";
    arr.at(6) = "DIV";
    arr.at(7) = "MULTI";
    arr.at(8) = "OR";
    arr.at(9) = "AND";
    arr.at(10) = "!OR";
    arr.at(11) = "NOT";
    //done with arith
    //data movement
    arr.at(12) = "MOVE"; 
    arr.at(13) = "MOVE&CLR";
    arr.at(14) = "LOAD";
    arr.at(15) = "STORE";
    arr.at(16) = "PUSH";
    arr.at(17) = "POP";
    //Load and jumps
    arr.at(18) = "LOADIMM";
    arr.at(19) = "JMP";
    arr.at(20) = "JMPIF0";
    arr.at(21) = "JMPIF!0";
    arr.at(22) = "CALL";
    arr.at(32) = "JMPIFCRRY";
    arr.at(24) = "JMPIFAULT";

}

template <std::integral T>
std::optional<std::bitset<8>> getNumberConversion(T incomingNumber);

std::bitset<8> getNextInstruction(const std::string& instruction);

void defineMap(std::map<std::string, std::bitset<8>>& map);

std::vector<std::string> tokenize(const std::string& line);

void assembleHelper(const std::string& line, std::vector<i8>& binFile);
void assemble(const std::string& filePath);

std::vector<std::string> getAsmFiles(const std::string& asmDirPath, const std::string& binDirPath);

