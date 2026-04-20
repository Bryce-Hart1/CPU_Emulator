#include <map>
#include <string>
#include "exceptions.hpp"

#pragma once

using u16 = u_int16_t;
namespace lbl{

    class Labels{
        private:
            std::map <std::string, u16> list_of_labels;
            u16 next_label_at_byte_off; //the next label is at byte offset: 
        public:

        Labels(){
            next_label_at_byte_off = 0;
        }
        void add_new_label(const std::string& name, const u16& byte){
            list_of_labels.insert({name, byte});
        }
        /**
         * Current CPU design allows for a max size of 12 bytes for a possible jump opcode, considering the space
         * for the reg to compare.
         */
        std::string label_to_12_bits(const std::string& name){
            const u16 MAX_BYTES = 4095; //if the program is over 4095 lines long, there will be no label to jump to
            int at = list_of_labels.at(name);
            if(at > MAX_BYTES){
                terminal::labelOutOfRange(name);
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