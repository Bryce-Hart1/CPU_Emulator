#include <exception>
#include <vector>
#include "bytestr.hpp"

#pragma once



namespace fs = std::filesystem;
using i8 = int8_t; // for instruction set
using usi8 = uint8_t;
using _byte = std::bitset<8>; // a bitset<8> namespace
using _bytestr = std::array<char, 8>;// not class obj, static array
namespace terminal{
    std::size_t numberOfBytesProcessed = 0;
    std::size_t atLine = 1; //line starts at 1 not zero
    std::vector<std::string> _warnings;
    std::vector<std::string> _errors;
    const std::string& wrnMes = "[warning]: "; //comes with a space after
    const std::string& errMes = "[ERROR]: "; //comes with a space after
    const std::string& atLinMsg = "At line: "; //comes with space after
    const _bytestr emptyByteStr = {'0', '0', '0', '0', '0', '0', '0', '0'};
    //incrmements the line that assembler is at
    void incrementLineWorkingOn(){
        atLine++;
    }
    std::string _at(const std::size_t& atLine){
        return (atLinMsg + std::to_string(atLine) + ' ');
    }
    void SyntaxError(const std::string& parsed){
        _errors.push_back(errMes + _at(atLine) + parsed + " is not valid syntax");
    }
    void AddressOutOfRange(int address){
        _errors.push_back(errMes + atLinMsg + std::to_string(atLine) + std::to_string(address) + "is out of range");
    }
    void nonvalidBiosOperation(int address){
        _errors.push_back(errMes + atLinMsg + std::to_string(atLine)  + std::to_string(address) + "(decimal) " +
    "is not a valid BIOS interrupt.\n Please see ASM_instructions for valid operations");
    }
    void hexValueNonValid(const std::string& hex){
        _errors.push_back(errMes + atLinMsg + std::to_string(atLine)  + "Hex value is non valid input:" + hex + " .");
    }
    void invalidLabelName(const std::string& attemptedLabel){
        _errors.push_back(errMes + atLinMsg + std::to_string(atLine) +  "label: " + attemptedLabel 
        + " is not valid label name, check Docs for info on how to format.");
    }
    void expectedLabel(){
        _errors.push_back(errMes + atLinMsg + std::to_string(atLine) + "Label name expected when label_ is written.");
    }
    void labelOutOfRange(const std::string& labels_name){
        _errors.push_back(errMes + atLinMsg + std::to_string(atLine) +  "label: " + labels_name 
        + " is out of range of possible addressing. Please check Docs on solutions.");
    }
    template <std::integral Type>
    void addIntegralOutOfRange(Type incomingNumber){
        _warnings.push_back(wrnMes + "Number converted " + std::to_string(incomingNumber) + " is not translatable to 8 bytes ");

    }
    void programLargerThanMaxSize(const std::size_t& bytes_larger){
        _errors.push_back(errMes + "Program is " + std::to_string(bytes_larger) + 
        " larger than the max offset of allowed bytes. Please refer docs");
    }
    
    namespace intern{ //internal errors, to help with debugging. Working version should never display these
        //internal str_to_half throws
        const std::string i = "INTERNAL: ";
        void str_to_half(){
            std::cout << i << "INVALID SIZE INTAKE IN STR_TO_HALF HELPER";
        }
    }
}


namespace help{
    //takes first 4 of string and outputs as array
    inline std::array<char, 4> str_to_half_bytestr(const std::string& input){
        std::array<char, 4> rtn = {'0', '0', '0', '0'};
        if(input.size() < 4){
            terminal::intern::str_to_half();
            return rtn;
        }
        for(int i = 0; i < 4; i++)
            rtn.at(i) = input.at(i);
        return rtn;
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
    
}