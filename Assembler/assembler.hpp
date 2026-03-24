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
using _bytestr = std::array<char, 8>;

inline std::map<std::string, i8> Register_Key = {
    {"R0", 0}, {"R1", 1}, {"R2", 2},  {"R3", 3},
    {"R4", 4}, {"R5", 5}, {"R6", 6},  {"R7", 7},
    {"R8", 8}, {"R9", 9}, {"R10", 10},{"R11", 11},
    {"R12", 12},{"R13", 13},{"R14", 14},{"R15", 15}
};

inline const std::string& asmPostfix = "a"; //choose the type of file that ASM will be read from 
inline const std::string& binPostfix = ".b"; //choose the type of file that "binary" will write to. include dot for now

inline const std::string& asmDir = "Asm";
inline const std::string& binDir = "Bin";
inline const int MAX_Instr_Size = 256;
inline const int nOfBasics = 4;
inline const int nOfArith = 8;
inline const int nOfDataMvm = 6;
inline const int nOfLoadAndJump = 7;
inline const int TOTAL_NUM_OF_INSTRUCTIONS = (nOfBasics + nOfArith + nOfDataMvm + nOfLoadAndJump);

//for instruction set
inline std::array<std::string, TOTAL_NUM_OF_INSTRUCTIONS> Instruction_Set; //set of actual instrucions as they come from strings;
inline std::array<_bytestr, TOTAL_NUM_OF_INSTRUCTIONS> BinaryMatch_Set;
inline std::size_t binMatchItr = 0;
inline std::map<std::string, std::bitset<8>> Instruction_Key; //actual hashmap of each instruction

inline void increment(_byte& bits);

inline i8 convertBitsetToByte(_byte bits);

enum class InstrType { BASIC, ARITH, DATA_MOV, LOAD_JUMP };


/**
 * helps keep binary instructions readable in @def defineInstructions
 * 
 **/
inline void addBinInstruction(const std::string str){
    _bytestr temp;
    for(int i = 0; i < 9; i++){ //always only take 8 (one byte)
        if(i != 4){
        temp.at(i) = str.at(i);
        }
    }
    BinaryMatch_Set.at(binMatchItr++) = temp;
}

/**
 * @attention these inline structures and function are for easy access to mainipulating instructions for 
 * easy addition of your own / more instructions.
 * Instruction_types helps define the (type) that each instruction is and how many bytes it is expecting
 * Errors in either of these functions will cause undefined behavior in not only the assembler, but the Emulator as well
 * please refer to ASM_Instructions for help.
 * good practice in defineInstructions is when you add an instruction, incremement the binary produced by one.
 */
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
inline void defineInstructions(std::array<std::string, TOTAL_NUM_OF_INSTRUCTIONS>& arr, std::array<_bytestr,TOTAL_NUM_OF_INSTRUCTIONS >& bin){
    arr.at(0) = "NOPERATION";
    addBinInstruction("0000:0000");
    arr.at(1) = "HALT";
    addBinInstruction("0000:0001");
    arr.at(2) = "RETURN";
    addBinInstruction("0000:0010");
    arr.at(3) = "CLRFLAGS";
    addBinInstruction("0000:0011");
    //done with basics
    arr.at(4) = "ADD";
    addBinInstruction("0100:0000");
    arr.at(5) = "SUB";
    addBinInstruction("0100:0001");
    arr.at(6) = "DIV";
    addBinInstruction("0100:0010");
    arr.at(7) = "MULTI";
    addBinInstruction("0100:0011");
    arr.at(8) = "OR";
    addBinInstruction("0100:0100");
    arr.at(9) = "AND";
    addBinInstruction("0100:0101");
    arr.at(10) = "!OR";
    addBinInstruction("0100:0110");
    arr.at(11) = "NOT";
    addBinInstruction("0100:0111");
    //done with arith
    //data movement 2 bytes aswell
    arr.at(12) = "MOVE"; 
    addBinInstruction("0100:1000");
    arr.at(13) = "MOVE&CLR";
    addBinInstruction("0100:1001");
    arr.at(14) = "LOAD";
    addBinInstruction("0100:1010");
    arr.at(15) = "STORE";
    addBinInstruction("0100:1011");
    arr.at(16) = "PUSH";
    addBinInstruction("0100:1100");
    arr.at(17) = "POP";
    addBinInstruction("0100:1101");
    //Load and jumps
    arr.at(18) = "LOADIMM";
    addBinInstruction("1000:0000");
    arr.at(19) = "JMP";
    addBinInstruction("1000:0001");
    arr.at(20) = "JMPIF0";
    addBinInstruction("1000:0010");
    arr.at(21) = "JMPIF!0";
    addBinInstruction("1000:0011");
    arr.at(22) = "CALL";
    addBinInstruction("1000:0100");
    arr.at(23) = "JMPIFCRRY";
    addBinInstruction("1000:0101");
    arr.at(24) = "JMPIFAULT";
    addBinInstruction("1000:0110");


}

template <std::integral T>
inline std::optional<std::bitset<8>> getNumberConversion(T incomingNumber);

inline std::bitset<8> getNextInstruction(const std::string& instruction);

inline void defineMap(std::map<std::string, _bytestr>& map){
    for(std::size_t i = 0; i < TOTAL_NUM_OF_INSTRUCTIONS; i++){
        map.insert({Instruction_Set.at(i), BinaryMatch_Set.at(i)});
    }
}

inline std::vector<std::string> tokenize(const std::string& line);

inline void assembleHelper(const std::string& line, std::vector<i8>& binFile);
inline void assemble(const std::string& filePath);

inline std::vector<std::string> getAsmFiles(const std::string& Path, const std::string& binDirPath);

