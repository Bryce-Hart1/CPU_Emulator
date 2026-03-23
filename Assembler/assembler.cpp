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

namespace fs = std::filesystem;
using i8 = int8_t; //for instruction set

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



//see ASM_Instructions.md for explainations
void defineInstructions(std::array<std::string, Instr_Size>& arr){
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
    arr.at(9) = "!OR";
    arr.at(10) = "NOT";
    //done with arith
    //data movement
    arr.at(11) = "MOVE"; 
    arr.at(12) = "MOVE&CLR";
    arr.at(13) = "LOAD";
    arr.at(14) = "STORE";
    arr.at(15) = "PUSH";
    arr.at(16) = "POP";
    //Load and jumps
    arr.at(17) = "LOADIMM";
    arr.at(18) = "JMP";
    arr.at(19) = "JMPIF0";
    arr.at(20) = "JMPIF!0";
    arr.at(21) = "CALL";
    arr.at(22) = "JMPIFCRRY";
    arr.at(23) = "JMPIFAULT";

}

/**
 * @brief treating the first 2 bits as the type, this maps all instructions, as seen in
 * @def defineInstructions
 */
void defineMap(std::map<std::string, std::bitset<8>>& map){
    std::bitset<8> bits;
    bits.reset();
    i8 itr = 0;
    for(auto& a : Instruction_Set){
        map.insert({a, bits});
            if(itr == nOfArith || itr == nOfBasics || itr == nOfDataMvm || itr == nOfLoadAndJump){
                bits.reset();
                if(itr == nOfBasics){ //onto 0100 
                    bits.flip(1);
                }
                if(itr == nOfLoadAndJump){ //onto 1000
                    bits.flip(0);
                }
                increment(bits);
            }
    }
}


std::bitset<8> getNextInstruction(const std::string& instruction){
    return Instruction_Key.at(instruction);
}






void increment(std::bitset<8>& bits){
    for(int i = 0; i < bits.size(); ++i){
        if (bits.flip(i).test(i)){
            break;
        }
    }
}



void assemble(const std::string& fileName, const std::string& fileContents){
    try{
        //open file

    }catch(const std::exception& e){

    }
}



std::vector<std::string> getAsmFiles(const std::string& asmDir, const std::string& binDir){

}


int main(){
    auto asmFiles = getAsmFiles(asmDir, binDir); //get all files to assemble


    return 0;
}
