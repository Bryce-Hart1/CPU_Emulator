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
#include <string>
#include <bitset>
#include <functional>
#include <array>
#include <optional>
#include <limits>
#include <concepts>
#include <sstream>
#include <format>
#include <cmath>

#include "exceptions.hpp" 
#include "labels.hpp"


inline const std::string &asmPostfix = ".em";  // choose the type of file that ASM will be read from
inline const std::string &binPostfix = ".b"; // choose the type of file that "binary" will write to. include dot for now

inline const std::string &asmDir = "asm"; //names of directory
inline const std::string &binDir = "Bin";
inline const int MAX_Instr_Size = 256;
inline const int nOfBasics = 4;
inline const int nOfArith = 8;
inline const int nOfDataMvm = 7;
inline const int nOfLoadAndJump = 7;
inline const int TOTAL_NUM_OF_INSTRUCTIONS = (nOfBasics + nOfArith + nOfDataMvm + nOfLoadAndJump);

// for instruction set
inline std::array<std::string, TOTAL_NUM_OF_INSTRUCTIONS> Instruction_Set; // set of actual instrucions as they come from strings;
inline std::array<_bytestr, TOTAL_NUM_OF_INSTRUCTIONS> BinaryMatch_Set;
inline std::size_t binMatchItr = 0;
inline std::map<std::string, _bytestr> Instruction_Key; // actual hashmap of each instruction

lbl::Labels found_labels; //found labels in the EM. 

enum class InstrType
{
    BASIC,
    ARITH,
    DATA_MOV,
    LOAD_JUMP
};




/**
 * gets reg as a full byte, has its respective overload that converts one reg
 * If there is only one key passed in. It will exist at top (R1)
 */
inline bytestr getRegisterKey(std::string R1, std::string R2){
    std::string byte = "";
    bytestr r;
    std::map<std::string, std::string> Register_Key = {
    {"R0", "0000"}, {"R1", "0001"}, {"R2", "0010"}, {"R3", "0011"}, {"R4", "0100"}, {"R5", "0101"}, {"R6", "0110"}, {"R7", "0111"}, {"R8", "1000"},
    {"R9", "1001"}, {"R10", "1010"}, {"R11", "1011"}, {"R12", "1100"}, {"R13", "1101"}, {"R14", "1110"}, {"R15", "1111"}, {"0000", "0000"}};

    try{
        byte += Register_Key.at(R1);
        byte += Register_Key.at(R2);
    }catch(const std::exception& e){
        terminal::SyntaxError(R1);
    }

    for(int i = 0; i < 8 && i < byte.size(); i++){
        r.change(i, byte.at(i));
    }

    return r;
}

inline bytestr getRegisterKey(std::string R1){
    return getRegisterKey(R1, "0000");
}

/**
 * helps keep binary instructions readable in @def defineInstructions
 *
 **/
inline void addBinInstruction(const std::string str)
{
    _bytestr temp;
    int tempIdx = 0;
    for (int i = 0; i < str.length() && tempIdx < 8; i++)
    { // always only take 8 (one byte), skip the colon at position 4
        if (i != 4)
        {
            temp.at(tempIdx++) = str.at(i);
        }
    }
    BinaryMatch_Set.at(binMatchItr++) = temp;
}

inline int hex_to_int(const std::string& passed) {
    std::string hexVal;

    if (!passed.empty() && (passed.back() == 'H' || passed.back() == 'h')) {
        hexVal = passed.substr(0, passed.size() - 1);
    } else if (!passed.empty() && (passed.front() == 'h' || passed.front() == 'H')) {
        hexVal = passed.substr(1);
    } else {
        hexVal = passed;
    }

    int finalVal = 0;

    for (char c : hexVal) {
        c = std::toupper(c);

        int digit;
        if (c >= '0' && c <= '9') digit = c - '0';
        else if (c >= 'A' && c <= 'F') digit = c - 'A' + 10;
        else return -1;

        finalVal = finalVal * 16 + digit;
    }

    return finalVal;
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
    {"NOPERATION", InstrType::BASIC}, {"HALT", InstrType::BASIC}, {"RETURN", InstrType::BASIC}, {"CLRFLAGS", InstrType::BASIC},

    {"ADD", InstrType::ARITH},
    {"SUB", InstrType::ARITH},
    {"DIV", InstrType::ARITH},
    {"MULTI", InstrType::ARITH},
    {"OR", InstrType::ARITH},
    {"AND", InstrType::ARITH},
    {"!OR", InstrType::ARITH},
    {"NOT", InstrType::ARITH},

    {"MOVE", InstrType::DATA_MOV},
    {"MOVE&CLR", InstrType::DATA_MOV},
    {"PUSH", InstrType::DATA_MOV},
    {"POP", InstrType::DATA_MOV},

    {"INT", InstrType::DATA_MOV}, //not really? but im not making a special enum

    {"LOADIMM", InstrType::LOAD_JUMP},
    {"JMP", InstrType::LOAD_JUMP},
    {"JMPIF0", InstrType::LOAD_JUMP},
    {"JMPIF!0", InstrType::LOAD_JUMP},
    {"CALL", InstrType::LOAD_JUMP},
    {"JMPIFCRRY", InstrType::LOAD_JUMP},
    {"JMPIFAULT", InstrType::LOAD_JUMP},
    {"LOAD", InstrType::DATA_MOV},
    {"STORE", InstrType::DATA_MOV},
};

// see ASM_Instructions.md for explainations
inline void defineInstructions(std::array<std::string, TOTAL_NUM_OF_INSTRUCTIONS> &arr, 
    std::array<_bytestr, TOTAL_NUM_OF_INSTRUCTIONS> &bin){
    arr.at(0) = "NOPERATION";
    addBinInstruction("0000:0000");
    arr.at(1) = "HALT";
    addBinInstruction("0000:0001");
    arr.at(2) = "RETURN";
    addBinInstruction("0000:0010");
    arr.at(3) = "CLRFLAGS";
    addBinInstruction("0000:0011");
    // done with basics
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
    // done with arith
    // data movement 2 bytes aswell

    arr.at(12) = "MOVE";
    addBinInstruction("0100:1000");
    arr.at(13) = "MOVE&CLR";
    addBinInstruction("0100:1001");
    /**
     * @attention LOAD and STORE are now three byte instructions. They were in data movement,
     * but they are not 2 byte instructions, so now they are at the end, here
     */
    arr.at(14) = "LOAD";
    addBinInstruction("1000:0111");
    arr.at(15) = "STORE";
    addBinInstruction("1000:1000");

    arr.at(16) = "PUSH";
    addBinInstruction("0100:1010");
    arr.at(17) = "POP";
    addBinInstruction("0100:1011");

    // Load and jumps
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
    //interupt (2 bytes)
    arr.at(25) = "INT";
    addBinInstruction("0100:1100");

}



//We could just type cast into bitset, but I would rather tell the user that there is an
//error
template <std::integral T>
std::optional<_bytestr> getNumberConversion(T incomingNumber) {
    using namespace terminal;
    if (incomingNumber > 127 || incomingNumber < -128) {
        terminal::addIntegralOutOfRange(incomingNumber); //warning for now
        return terminal::emptyByteStr;
    }
    _bytestr r;
    int n = (static_cast<int>(incomingNumber));
    if(n < 0){
        r.at(0) = '1';
        n = std::abs(n);
    }
    int thisBit= 64;
    for(int i = 1; i < 8; i++){
        if(n >= thisBit){
            n %= thisBit;
            r.at(i) = '1';
        }else{
            r.at(i) = '0';
        }
        thisBit /= 2;
    }
    return r;
}
    

inline _bytestr getNextInstruction(const std::string &instruction){
    return Instruction_Key.at(instruction);
}

/**
 * for RAM address.
 */
inline _bytestr getTranslatedAddress(std::string hex){
    try {
        int incoming = hex_to_int(hex);
        if(incoming == -1){
            return terminal::emptyByteStr;
        }
        if(incoming > 255 || incoming < 0){
            terminal::AddressOutOfRange(incoming);
            return terminal::emptyByteStr;
        }
        int thisBit = 128;
        _bytestr r;
        for(int i = 0; i < 8; i++){
            if(incoming >= thisBit){
                incoming -= thisBit;
                r.at(i) = '1';
            } else {
                r.at(i) = '0';
            }
            thisBit /= 2;
        }
        return r;
    }catch(const std::exception& e){
        terminal::SyntaxError(hex);
        return terminal::emptyByteStr;
    }
}
    
//sets up passed in map with all instructions and their matching binary
inline void defineMap(std::map<std::string, _bytestr> &map)
{
    for (std::size_t i = 0; i < TOTAL_NUM_OF_INSTRUCTIONS; i++){

        map.insert({Instruction_Set.at(i), BinaryMatch_Set.at(i)});
    }
}



inline std::vector<std::string> tokenize(const std::string &line);

inline void assembleHelper(const std::string &line, std::vector<i8> &binFile);
inline void assemble(const std::string &filePath);

inline std::vector<std::string> getAsmFiles(const std::string &Path, const std::string &binDirPath);
lbl::Labels do_first_pass(std::ifstream& file);