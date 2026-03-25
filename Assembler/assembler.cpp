#include "assembler.hpp"

namespace terminal{
    std::size_t atLine = 0;
    std::vector<std::string> _warnings;
    std::vector<std::string> _errors;
    std::string wrnMes = "[warning]: "; //comes with a space after
    std::string errMes = "[ERROR]: "; //comes with a space after
}


//We could just type cast into bitset, but I would rather tell the user that there is an
//error
template <std::integral T>
std::optional<_bytestr> getNumberConversion(T incomingNumber) {
    using namespace terminal;
    if (incomingNumber > 127 || incomingNumber < -128) {
        _warnings.push_back(wrnMes + "Number converted '" + std::to_string(incomingNumber) + );
        return std::nullopt;
    }
}


std::vector<std::string> tokenize(const std::string& line) {
    std::vector<std::string> tokens;
    std::istringstream stream(line);
    std::string token;
    while (stream >> token){
    tokens.push_back(token);
    }
    return tokens;
}


/**
 * @details little abstraction to take a single line of asm and convert it to binary
 */
void assembleHelper(const std::string& line, std::vector<_bytestr>& binFile){
    if (line.empty() || line[0] == '#'){
        return;
    }

    auto tokens = tokenize(line);
    std::string mnemonic = tokens[0]; // instruction (Ex: ADD)
    auto opcode = Instruction_Key.at(mnemonic);
    InstrType type = Instruction_Types.at(mnemonic);

    if(type == InstrType::BASIC) { // 1 byte — just the opcode
        binFile.push_back(getNextInstruction(tokens.at(0)));

    }else if (type == InstrType::ARITH || type == InstrType::DATA_MOV) {
    binFile.push_back(convertBitsetToByte(getNextInstruction(mnemonic)));

    if (mnemonic == "NOT"){ //special case:
        // only one register, bottom part is 0
            i8 r1 = Register_Key.at(tokens[1]);
            _byte operands(r1 << 4);
            binFile.push_back(convertBitsetToByte(operands));
        }else{
            i8 r1 = Register_Key.at(tokens[1]);
            i8 r2 = Register_Key.at(tokens[2]);
            _byte operands((r1 << 4) | r2);
            binFile.push_back(convertBitsetToByte(operands));}
    }else if (type == InstrType::LOAD_JUMP) {
        // 3 bytes — opcode, [R1 | 0000], address
        i8 r1 = Register_Key.at(tokens[1]);
        usi8 addr = std::stoul(tokens[2], nullptr, 0); // handles 0x prefix
        _byte regByte(r1 << 4);
        binFile.push_back(convertBitsetToByte(getNextInstruction(mnemonic)));
        binFile.push_back(convertBitsetToByte(regByte));
        binFile.push_back(addr);
        }
}

void assemble(const std::string& filePath){
    using namespace std;
    vector<_bytestr> binFile;
    try{
        ifstream inputFile(filePath);
        if(!inputFile.is_open()){
            throw runtime_error("Could not open file: " + filePath);
        }

        string outputFileName = filePath.substr(filePath.find_last_of("/\\") + 1);
        outputFileName = outputFileName.substr(0, outputFileName.find_last_of("."));
        ofstream outputFile(binDir + "/" + outputFileName + binPostfix);

        string line;
        while (getline(inputFile, line)) {
            assembleHelper(line, binFile);
        }
        for(auto& a : binFile){
            cout << a << std::endl;
        }
        inputFile.close();
        outputFile.close();
    }catch(const exception& e){
        cerr << "Error assembling file: " << e.what() << std::endl;
    }
}



std::vector<std::string> getAsmFiles(const std::string& asmDirPath, const std::string& binDirPath){
    std::vector<std::string> files;

    try{
        if(!fs::exists(asmDirPath)){
            throw std::runtime_error("Asm directory does not exist: " + asmDirPath);
        }

        for(const auto& entry : fs::directory_iterator(asmDirPath)){ //will do this for all files
            if(entry.is_regular_file()){
                std::string filename = entry.path().filename().string();
                if(filename.substr(filename.find_last_of(".") + 1) == asmPostfix){
                    files.push_back(entry.path().string());
                }
            }
        }
    }catch(const std::exception& e){
        std::cerr << "Error reading Asm directory: " << e.what() << std::endl;
    }

    return files;
}


int main(){
    defineInstructions(Instruction_Set);
    defineMap(Instruction_Key);

    // Create Bin directory if it doesn't exist
    if(!fs::exists(binDir)){
        fs::create_directory(binDir);
    }

    auto asmFiles = getAsmFiles(asmDir, binDir); //get all files to assemble

    for(const auto& file : asmFiles){
        std::cout << "Assembling: " << file << std::endl;
        assemble(file);
    }

    return 0;
}
