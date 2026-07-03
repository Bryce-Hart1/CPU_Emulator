#ifndef BYTESTR_HPP
#define BYTESTR_HPP

#include <array>
#include <string>
#include <algorithm>
#include <cmath>

class bytestr{
    private:
    std::array<char, 8> _bits;
    public:
    bytestr(){
        for(auto& c : _bits){
            c = '0';
        }
    }
    bytestr(int unsignedInt) : bytestr() {
        int_to_un(unsignedInt);
    }
    bytestr(const std::string& inputStr) : bytestr() {
        string_to(inputStr);
    }

    void change(int n, bool v){
        if(n > 7 || n < 0) return;
        _bits.at(n) = v ? '1' : '0';
    }

    void change(int n, char v){
        if(n > 7 || n < 0 || v != '0' && v != '1'){
            return;
        }
        _bits.at(n) = v;
    }

    //change half of the byte. is_top -> first 4, !is_top -> last 4
    void set_half(bool is_top, std::array<char, 4> half){
        if(is_top){
            for(int i = 0; i < 4; i++)
                _bits.at(i) = half.at(i);
        }else{
            for(int i = 0; i < 4; i++)
                _bits.at(i+4) = half.at(i);
        }
    }

    std::array<char, 4> get_half(bool is_top){
        std::array<char, 4> half;
        if(is_top){
            for(int i = 0; i < 4; i++)
                half.at(i) = _bits.at(i);
        }else{
            for(int i = 0; i < 4; i++)
                half.at(i) = _bits.at(i+4);
        }
        return half;
    }
    std::array<char, 8> get_obj(){
        return _bits;
    }

    std::string to_string(){
        std::string rtn;
        for(int i = 0; i < 8; i++){
            rtn += _bits.at(i);
        }
        return rtn;
    }

    //works with non full strings
    void string_to(const std::string& input){
        if(input.size() > 8)
            return;

        int itr = 7;

        for(int i = input.size()-1; 0 <= i; i--){  // i-- not i++
            if(input.at(i) == '1'){
                this->change(itr, '1');
            }else{
                this->change(itr, '0');
            }
            if(itr == 0){
                break;
            }
            itr--;
        }
    }

    
    void int_to_un(int convert){
        convert %= 256;
        int div = 128;
        for(int i = 0; i < 8; i++){
            if(convert >= div){
                _bits.at(i) = '1';
                convert -= div;
            }else{
                _bits.at(i) = '0';
            }
            div /= 2;
        }
    }

};

#endif // BYTESTR_HPP
