#include "assembler.hpp"



void increment(_byte& bits){
    for(int i = 0; i < bits.size(); ++i){
        if (bits.flip(i).test(i)){
            break;
        }
    }
}

i8 convertBitsetToByte(_byte byte){
    return static_cast<usi8>(byte.to_ulong());
}

//We could just type cast into bitset, but I would rather tell the user that there is an
//error
template <std::integral T>
std::optional<_byte> getNumberConversion(T incomingNumber) {
    if (incomingNumber > 127 || incomingNumber < -128) {
        return std::nullopt;
    }

    const auto converted = static_cast<i8>(incomingNumber);
    const i8 magnitude = static_cast<i8>(std::abs(converted));

    _byte bits(magnitude); 
    bits[7] = (converted < 0); //since 7 is Most significant bit
    return bits;
}


_byte getNextInstruction(const std::string& instruction){
    return Instruction_Key.at(instruction);
}

/**
 * @brief treating the first 2 bits as the type, this maps all instructions, as seen in
 * @def defineInstructions
 */
void defineMap(std::map<std::string, _byte>& map){
    _byte bits;
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
void assembleHelper(const std::string& line, std::vector<i8>& binFile){
    if (line.empty() || line[0] == '#'){
        return;
    }

    auto tokens = tokenize(line);
    std::string mnemonic = tokens[0]; // instruction (Ex: ADD)
    auto opcode = Instruction_Key.at(mnemonic);
    InstrType type = Instruction_Types.at(mnemonic);

    if(type == InstrType::BASIC) { // 1 byte — just the opcode
        binFile.push_back(convertBitsetToByte(getNextInstruction(mnemonic)));

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
    vector<i8> binFile;
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
            outputFile << a;
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
