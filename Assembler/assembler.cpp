#include "assembler.hpp"




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
    if (tokens.empty()) {
        return;
    }

    std::string mnemonic = tokens[0]; // instruction (Ex: ADD)
    auto opcode = Instruction_Key.at(mnemonic);
    InstrType type = Instruction_Types.at(mnemonic);

    if(type == InstrType::BASIC) { // 1 byte — just the opcode
        binFile.push_back(getNextInstruction(mnemonic));

    }else if (type == InstrType::ARITH || type == InstrType::DATA_MOV) { //opcode, 1/2 addresses
        binFile.push_back(getNextInstruction(mnemonic)); //byte 1

        if (mnemonic == "NOT"){ //special case:
            if (tokens.size() < 2) return;
            binFile.push_back(getRegisterKey(tokens[1])); //byte 2
        }else{
            if (tokens.size() < 3) return;
            binFile.push_back(getRegisterKey(tokens[1], tokens[2])); //byte 2
        }

    }else if (type == InstrType::LOAD_JUMP) {
        // 3 bytes — opcode, [R1 | 0000], address
        if (tokens.size() < 3) return;
        binFile.push_back(getNextInstruction(mnemonic)); //byte 1
        binFile.push_back(getRegisterKey(tokens[1])); //byte 2 (half : 0000)
        binFile.push_back(getTranslatedAddress(tokens[2])); //byte 3
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
        for(std::size_t i = 0; i < (binFile.size()); i++){
            _bytestr translate = binFile.at(i);
            for(int j = 0; j < 8; j++){
                outputFile << translate.at(j);
            }
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
    defineInstructions(Instruction_Set, BinaryMatch_Set);
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
