#include "assembler.hpp"
#include "labels.hpp"

// tokenize a single line
std::vector<std::string> tokenize(const std::string& line) {
    std::vector<std::string> tokens;
    std::istringstream stream(line);
    std::string token;
    while (stream >> token){
    tokens.push_back(token);
    }
    return tokens;
}

void do_first_pass(const std::ifstream& file){
    using namespace lbl;
    Labels labels_so_far = Labels();
    //TODO: finish this
}

/**
 * @details little abstraction to take a single line of asm and convert it to binary
 */
void assembleHelper(const std::string& line, std::vector<_bytestr>& binFile){
    //if line is blank or a comment
    if (line.empty() || line[0] == '#'){
        return;
    }
    //if there is no tokens on this line
    auto tokens = tokenize(line);
    if (tokens.empty()) {
        return;
    }

    std::string mnemonic = tokens[0]; // instruction (Ex: ADD)
    auto opcode = Instruction_Key.at(mnemonic);
    InstrType type = Instruction_Types.at(mnemonic);
        
    std::string stripped = line.substr(0, line.find('#'));
    
    if (stripped.empty()) return;  // was a full-line comment or blank
    
    if (tokens.empty()) return;
    if(type == InstrType::BASIC) { // 1 byte — just the opcode
        binFile.push_back(getNextInstruction(mnemonic));

    }else if (type == InstrType::ARITH || type == InstrType::DATA_MOV) { //opcode, 1/2 addresses
        binFile.push_back(getNextInstruction(mnemonic)); //byte 1

        if(mnemonic == "INT"){
            const int SCREEN = 10;
            const int FILLSCREEN = 11;
            const int DISK_READ = 13;
            const int DISK_WRITE = 14;

            int incoming = hex_to_int(tokens.at(1));
            if(incoming == -1){
                terminal::hexValueNonValid(tokens.at(1));
                incoming = 0;
            }
            try{
                if(incoming != SCREEN && incoming != DISK_READ && incoming != DISK_WRITE && incoming != FILLSCREEN){ 
                    terminal::nonvalidBiosOperation(incoming);
                } else {
                    _bytestr int_of_hex = int_to_byteStr(incoming);
                    binFile.push_back(int_of_hex);
                }
            } catch(const std::exception& e){
                std::cerr << e.what() << std::endl;
            }
            return;  //INT is fully handled, don't fall through to register.
        }

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
            terminal::incrementLineWorkingOn();
        }
        for(std::size_t i = 0; i < (binFile.size()); i++){
            _bytestr translate = binFile.at(i);
            for(int j = 0; j < 8; j++){
                outputFile << translate.at(j);
            }// one byte
            //my thinking ~150 characters 150 / 8 = 18ish bytes
            if(i % 18 == 0 && i != 0){
                outputFile << '\n'; //newline to div up
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
    using namespace terminal;
    if(_errors.size() != 0){
        //do something to stop the making of the file, eventually
    }
    for(auto& warn : _warnings){
        std::cout << warn << std::endl;
    } 
    for(auto& err : _errors){
        std::cout << err << std::endl;
    }
    if(_warnings.size() != 0){
        std::cout << _warnings.size() << " warnings";
    }
    if(_warnings.size() != 0 && _errors.size() != 0){
        std::cout << " and";
    }
    if(_errors.size() != 0){
        std::cout << _errors.size() << " errors";
    }
    if(_warnings.size() != 0 || _errors.size() != 0){
    std::cout << " generated" << std::endl;
    }
    _warnings.clear();
    _errors.clear(); // clear bc we already have printed
    }

    

    return 0;
}
