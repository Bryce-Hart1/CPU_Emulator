#ifndef LABELS_HPP
#define LABELS_HPP

#include <map>
#include <string>
#include <cstdint>
#include "exceptions.hpp"

using u16 = std::uint16_t; // u_int16_t is POSIX-only; use the portable <cstdint> type
namespace lbl{

    class Labels{
        private:
            std::map <std::string, u16> list_of_labels;
            u16 next_label_at_byte_off; //the next label is at byte offset: 
        public:

        Labels(){
            next_label_at_byte_off = 0;
        }
        void add_new_label(const std::string& name, const u16& byte_Offset){
            list_of_labels.insert({name, byte_Offset});
        }
        /**
         * Current CPU design allows for a max size of 12 bytes for a possible jump opcode, considering the space
         * for the reg to compare.
         */
        std::size_t size_of(){
            return list_of_labels.size();
        }

        std::string to_12_bits(const std::string& name){
            const u16 MAX_BYTES = 4095; //if the program is over 4095 lines long, there will be no label to jump to
            int at;
            try{
                at = list_of_labels.at(name);
                if(at > MAX_BYTES){
                    terminal::labelOutOfRange(name);
                    return "000000000000";
                }
            }catch(const std::exception& e){
                std::cerr << e.what() << std::endl;
                std::cout << "tried to reach label that does not exist" << std::endl;
                return "000000000000";
            }

            /**
             * The program will never process a number larger then 4095, which is the largest
             * unsigned 12 bit number
             */
            u16 decr = 2048;
            std::string rtn = "";
            const std::size_t MAX_ADD_LENGTH = 12; //max address length
            for(int i = 0; i < MAX_ADD_LENGTH; i++){
                if(at / decr == 1){
                    rtn.push_back('1');
                }else{
                    rtn.push_back('0');
                }
                decr /= 2;
            }
            return rtn;
        }







    };
    inline bool is_labelTg_valid(const std::string&line){
        const std::string labelTg = "label_"; //what label tag looks like
        const std::string shouldBeLabel = line.substr(0, 6);
        if(labelTg != shouldBeLabel){
            return false;
        }
        return true;
    }
    //we already know that there is a valid label_ tag at the beginning, continue from there:
    //we also know that the line is one token as well
    inline std::string extract_label_name(const std::string& line){
        const std::string labelTg = "label_"; //what label tag looks like
        if(line.size() == labelTg.size()){
            terminal::expectedLabel(); //rest of string is blank!
        }
        return line.substr(labelTg.size());
    }


}

#endif // LABELS_HPP
