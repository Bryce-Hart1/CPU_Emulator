#include <exception>
#include <vector>

#pragma once



namespace fs = std::filesystem;
using i8 = int8_t; // for instruction set
using usi8 = uint8_t;
using _byte = std::bitset<8>; // a bitset<8> namespace
using _bytestr = std::array<char, 8>;

namespace terminal{
    std::size_t numberOfBytesProcessed = 0;
    std::size_t atLine = 0;
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
    void SyntaxError(const std::string& parsed){
        _errors.push_back(errMes + atLinMsg + std::to_string(atLine) + parsed + " is not valid syntax");
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
}
