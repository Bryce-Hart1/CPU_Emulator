use crate::IO;

// Helps with things like font rendering, when the cpu writes to the screen address, values can be mapped here.
//when a unknown value is written into screen.rs, it can call apon functions here to help

use std::collections::HashMap;










/**
 * Void function that takes in arguements to draw at the current x and y the message. This will find a way to print it without clipping
 * This allows for simplicity for writing text to the screen in the simulation. may change later
 */
pub fn screen_request_to_draw_at(x: u8, y: u8, msg: &str){
    for i in 0..msg.len(){
        let c: char = msg.chars().nth(i).unwrap();
        let map = what_is_char(c);
        for i in 0..map.len(){
            for j in 0..map[i].len(){

            }
        }





    }
}






/**
 * returns a 2d array of each char. DO NOT rely on this function for padding,
 * the padding must be added in later
 * all chars have height of 5
 * This is current "standard" font for cpuEM
 * Should contain all printable ASCII values, if the char "input" is out of range it will print
 * an unknown char, _unknown_char
 * 
 */
pub fn what_is_char(input: char) -> Vec<Vec<bool>> {
    let rtn: Vec<Vec<bool>> = Vec::new();
    let mut map: HashMap<char, Vec<Vec<bool>>> = HashMap::new();


    //unknown character is trying to be displayed
    let _unknown_char = 
    vec![
    vec![true, false, true, false, true],
    vec![false, true, false, true, false],
    vec![true, false, true, false, true],
    vec![false, true, false, true, false],
    vec![true, false, true, false, true]
    ];

    
    //ascii 32 5*5 (space key)
    let _space = 
    vec![
    vec![false, false, false, false, false],
    vec![false, false, false, false, false],
    vec![false, false, false, false, false],
    vec![false, false, false, false, false],
    vec![false, false, false, false, false]
    ];
    map.insert(' ', _space);
    //ascii 33 5*1 !
    let _expl =
    vec![
        vec![true],
        vec![true],
        vec![true],
        vec![false],
        vec![true],
    ];
    map.insert('!', _expl);
    //ascii 34 5*3 "
    let _quote = 
    vec![
        vec![true, false, true],
        vec![true, false, true],
        vec![false, false, false],
        vec![false, false, false],
        vec![false, false, false],
    ];
    map.insert('"', _quote);

    //ascii 35 #
    let _hashtag = 
    vec![
    vec![false, true, false, true, false],
    vec![true, true, true, true, true],
    vec![false, true, false, true, false],
    vec![true, true, true, true, true],
    vec![false, true, false, true, false]    
    ];
    map.insert('#', _hashtag);

    //ascii 36 $
    let _cash =
    vec![
    vec![true, true, true, true, true],
    vec![true, false, true, false, false],
    vec![true, true, true, true, true],
    vec![false, false, true, false, true],
    vec![true, true, true, true, true]    
    ];    
    map.insert('$', _cash);
    //ascii 37 %
    let _percent = 
    vec![
    vec![true, false, false, false, true],
    vec![false, false, false, true, false],
    vec![false, false, true, false, false],
    vec![false, true, false, false, false],
    vec![true, false, false, false, true]    
    ]; 
    map.insert('%', _percent);

    //ascii 38 & 
    //may. change, dont know how readable this is
    let _and =
    vec![
    vec![false, true, true, true, false],
    vec![true, false, false, false, true],
    vec![false, true, true, false, false],
    vec![true, false, false, true, true],
    vec![false, true, true, false, true]
    ];   
    map.insert('&', _and);

    //ascii 39
    let _singleComma = 
    vec![
    vec![true],
    vec![true],
    vec![false],
    vec![false],
    vec![false]
    ];
    map.insert('\'', _singleComma);

    //ascii 40 ( 
    let _leftPara = 
    vec![
    vec![false, true],
    vec![true, false],
    vec![true, false],
    vec![true, false],
    vec![false, true]
    ];   
    map.insert('(', _leftPara);

    //ascii 41 )
    let _rightPara = 
    vec![
    vec![true, false],
    vec![false, true],
    vec![false, true],
    vec![false, true],
    vec![true, false]
    ];   
    map.insert(')', _rightPara);

    //ascii 42 * 
    let _multi = 
    vec![
    vec![false, false, false],
    vec![true, false, true],
    vec![false, true, false],
    vec![true, false, true],
    vec![false, false, false]
    ];
    map.insert('*', _multi);

    //ascii 43 + 
   let _plus = 
    vec![
    vec![false, false, false],
    vec![false, true, false],
    vec![true, true, true],
    vec![false, true, false],
    vec![false, false, false]
    ];    
    map.insert('+', _plus);

    //ascii 44 , 
    let _comma = 
    vec![
    vec![false, false],
    vec![false, false],
    vec![false, false],
    vec![false, true],
    vec![true, false]
    ];   


    //ascii 45 - 
    let _minus = 
    vec![
    vec![false, false],
    vec![false, false],
    vec![true, true],
    vec![false, false],
    vec![false, false]
    ];   


    //ascii 46 .
    let _period = 
    vec![
    vec![false],
    vec![false],
    vec![false],
    vec![false],
    vec![true]
    ];   


    //ascii 47 /
   let _l_slash = 
    vec![
    vec![false, false, true],
    vec![false, false, true],
    vec![false, true, false],
    vec![true, false, false],
    vec![true, false, false]
    ];  


    //ascii 48 0
    let _zero = 
    vec![
    vec![false, true, true],
    vec![true, false, true],
    vec![true, false, true],
    vec![true, false, true],
    vec![true, true, false]
    ];     


    //ascii 49 1
    let _one = 
    vec![
    vec![false, true],
    vec![true, true],
    vec![false, true],
    vec![false, true],
    vec![false, true]
    ];    

    //ascii 50 2
    let _two = 
    vec![
    vec![false, true, false],
    vec![true, false, true],
    vec![false, true, true],
    vec![true, false, false],
    vec![true, true, true]
    ];     


    //ascii 51 3 
    let _three =
    vec![
    vec![true, true, true],
    vec![false, false, true],
    vec![true, true, true],
    vec![false, false, true],
    vec![true, true, true]
    ];    


    //ascii 52 4
    let _four =
    vec![
    vec![true, false, true],
    vec![true, false, true],
    vec![true, true, true],
    vec![false, false, true],
    vec![false, false, true]
    ];    


    let _five = 
    vec![
    vec![true, true, true],
    vec![true, false, false],
    vec![false, true, true],
    vec![false, false, true],
    vec![true, true, true]
    ];    


    let _six = 
    vec![
    vec![true, true, true],
    vec![true, false, false],
    vec![true, true, true],
    vec![true, false, true],
    vec![true, true, true]
    ];   


    let _seven = 
    vec![
    vec![true, true, true],
    vec![false, false, true],
    vec![false, true, false],
    vec![true, false, false],
    vec![true, false, false]
    ];    


    let _eight = 
    vec![
    vec![true, true, true],
    vec![true, false, true],
    vec![true, true, true],
    vec![true, false, true],
    vec![true, true, true]
    ];    

    //ascii 57: 9
    let _nine = 
    vec![
    vec![true, true, true],
    vec![true, false, true],
    vec![true, true, true],
    vec![false, false, true],
    vec![false, false, true]
    ];   

    //ascii 58 colon :
    let _colon = 
    vec![
    vec![false],
    vec![true],
    vec![false],
    vec![true],
    vec![false]
    ];    

    //ascii 59 ; semicolon
    let _semicolon = 
    vec![
    vec![false, false],
    vec![false, true],
    vec![false, false],
    vec![false, true],
    vec![true, false]
    ];       
    //ascii 60 < 
    let _greater_than =
    vec![
    vec![false, false],
    vec![false, true],
    vec![true, false],
    vec![false, true],
    vec![false, false]
    ];       
    //ascii 61 =
    let _equals =
    vec![
    vec![false, false],
    vec![true, true],
    vec![false, false],
    vec![true, true],
    vec![false, false]
    ];   
    //ascii 62 > 
    let _less_than =
    vec![
    vec![false, false],
    vec![true, false],
    vec![false, true],
    vec![true, false],
    vec![false, false]
    ];    
    //ascii 63 ?
    let _question = 
    vec![
    vec![true, true, true],
    vec![true, false, true],
    vec![false, true, false],
    vec![false, false, false],
    vec![false, true, false]
    ];   
    //ascii 64 @
    let _at = 
    vec![
    vec![false, true, true, true, false],
    vec![true, false, false, false, true],
    vec![true, false, true, true, true],
    vec![true, false, true, false, true],
    vec![false, true, true, true, false]
    ];  
    //ascii 65 A
    let _A = 
    vec![
    vec![true, true, true],
    vec![true, false, true],
    vec![true, true, true],
    vec![true, false, true],
    vec![true, false, true]
    ];


    //ascii 66
    let _B = 
    vec![
    vec![true, true, true],
    vec![true, false, true],
    vec![true, true, false],
    vec![true, false, true],
    vec![true, true, true]
    ];   


    let _C = 
    vec![
    vec![true, true, true],
    vec![true, false, false],
    vec![true, false, false],
    vec![true, false, false],
    vec![true, true, true]
    ];   


    let _D = 
    vec![
    vec![true, true, false],
    vec![true, false, true],
    vec![true, false, true],
    vec![true, false, true],
    vec![true, true, false]
    ];   


    let _E = 
    vec![
    vec![true, true, true],
    vec![true, false, false],
    vec![true, true, true],
    vec![true, false, false],
    vec![true, true, true]
    ];   


    let _F = 
    vec![
    vec![true, true, true],
    vec![true, false, false],
    vec![true, true, true],
    vec![true, false, false],
    vec![true, false, false]
    ];   

    let _G = 
    vec![
    vec![false, true, true, true],
    vec![true, false, false, false],
    vec![true, false, true, true],
    vec![true, false, false, true],
    vec![false, true, true, false]
    ];   
    let _H = 
    vec![
    vec![true, false, true],
    vec![true, false, true],
    vec![true, true, true],
    vec![true, false, true],
    vec![true, false, true]
    ];   

    //ascii 73 I
    let _I =
    vec![
    vec![true, true, true],
    vec![false, true, false],
    vec![false, true, false],
    vec![false, true, false],
    vec![true, true, true]
    ];

    let _J = 
    vec![
    vec![false, false, true],
    vec![false, false, true],
    vec![false, false, true],
    vec![true, false, true],
    vec![true, true, true]
    ];    
    let _K =
    vec![
    vec![true, false, true],
    vec![true, true, false],
    vec![true, false, false],
    vec![true, true, false],
    vec![true, false, true]
    ];     
    let _L =
    vec![
    vec![true, false, false],
    vec![true, false, false],
    vec![true, false, false],
    vec![true, false, false],
    vec![true, true, true]
    ];     
    let _M = 
    vec![
    vec![true, false, false, false, true],
    vec![true, true, false, true, true],
    vec![true, false, true, false, true],
    vec![true, false, true, false, true],
    vec![true, false, false, false, true]
    ];   
    //ascii 78
    let _N = 
    vec![
    vec![true, false, false, true],
    vec![true, true, false, true],
    vec![true, false, true, true],
    vec![true, false, false, true],
    vec![true, false, false, true]
    ];   

    //ascii 79 O
    let _O = 
    vec![
    vec![true, true, true],
    vec![true, false, true],
    vec![true, false, true],
    vec![true, false, true],
    vec![true, true, true]
    ];     

    // ascii 80 P
    let _P = 
    vec![
    vec![true, true, true],
    vec![true, false, true],
    vec![true, true, true],
    vec![true, false, false],
    vec![true, false, false]
    ];     

    let _Q = 
    vec![
    vec![false, true, false, false],
    vec![true, false, true, false],
    vec![true, false, true, false],
    vec![true, false, true, false],
    vec![false, true, false, true]
    ];    
    let _R = 
    vec![
    vec![true, true, true],
    vec![true, false, true],
    vec![true, true, true],
    vec![true, true, false],
    vec![true, false, true]
    ];    
    let _S = 
    vec![
    vec![true, true, true],
    vec![true, false, false],
    vec![true, true, true],
    vec![false, false, true],
    vec![true, true, true]
    ];    
    let _T = 
    vec![
    vec![true, true, true],
    vec![false, true, false],
    vec![false, true, false],
    vec![false, true, false],
    vec![false, true, false]
    ];       
    let _U = 
    vec![
    vec![true, false, true],
    vec![true, false, true],
    vec![true, false, true],
    vec![true, false, true],
    vec![true, true, true]
    ];   
    let _V = 
    vec![
    vec![true, false, true],
    vec![true, false, true],
    vec![true, false, true],
    vec![true, false, true],
    vec![false, true, false]
    ];   
    let _W = 
    vec![
    vec![true, false, false, false, true],
    vec![true, false, true, false, true],
    vec![true, false, true, false, true],
    vec![true, true, false, true, true],
    vec![true, false, false, false, true]
    ];   
    let _X = 
    vec![
    vec![true, false, true],
    vec![true, false, true],
    vec![false, true, false],
    vec![true, false, true],
    vec![true, false, true]
    ];         
    let _Y = 
    vec![
    vec![true, false, true],
    vec![true, false, true],
    vec![false, true, false],
    vec![false, true, false],
    vec![false, true, false]
    ];       
      
    let _Z = 
    vec![
    vec![true, true, true],
    vec![false, false, true],
    vec![false, true, false],
    vec![true, false, false],
    vec![true, true, true]
    ];         

    //ascii 91
    let _left_bracket = 
    vec![
    vec![true, true],
    vec![true, false],
    vec![true, false],
    vec![true, false],
    vec![true, true],
    ]; 
    //ascii 92
    let _back_slash = 
    vec![
    vec![true, false, false],
    vec![true, false, false],
    vec![false, true, false],
    vec![false, false, true],
    vec![false, false, true],
    ];     
    //ascii 93
    let _right_bracket = 
    vec![
    vec![true, true],
    vec![false, true],
    vec![false, true],
    vec![false, true],
    vec![true, true],
    ];    
    //ascii 94
    let _hat = 
    vec![
    vec![false, false, false],
    vec![false, true, false],
    vec![true, false, true],
    vec![false, false, false],
    vec![false, false, false]
    ];  
    //ascii 95
    let _underscore =
    vec![
    vec![false, false, false],
    vec![false, false, false],
    vec![false, false, false],
    vec![false, false, false],
    vec![true, true, true]
    ];     

    // lowercase sits on rows 2-4 with ascenders up to row 0. Letters with a middle stroke
    // (e, s, z) use rows 1-4, and descenders (g, j, p, q, y) are raised a row so the tail fits.
    //ascii 96 ` backtick
    let _backtick = 
    vec![
    vec![true, false],
    vec![false, true],
    vec![false, false],
    vec![false, false],
    vec![false, false]
    ];
    //ascii 97 a
    let _a = 
    vec![
    vec![false, false, false],
    vec![false, false, false],
    vec![false, true, true],
    vec![true, false, true],
    vec![false, true, true]
    ];
    //ascii 98 b
    let _b = 
    vec![
    vec![true, false, false],
    vec![true, false, false],
    vec![true, true, false],
    vec![true, false, true],
    vec![true, true, false]
    ];
    //ascii 99 c
    let _c = 
    vec![
    vec![false, false, false],
    vec![false, false, false],
    vec![false, true, true],
    vec![true, false, false],
    vec![false, true, true]
    ];
    //ascii 100 d
    let _d = 
    vec![
    vec![false, false, true],
    vec![false, false, true],
    vec![false, true, true],
    vec![true, false, true],
    vec![false, true, true]
    ];
    //ascii 101 e
    let _e = 
    vec![
    vec![false, false, false],
    vec![false, true, false],
    vec![true, true, true],
    vec![true, false, false],
    vec![false, true, true]
    ];
    //ascii 102 f
    let _f = 
    vec![
    vec![false, true, true],
    vec![false, true, false],
    vec![true, true, true],
    vec![false, true, false],
    vec![false, true, false]
    ];
    //ascii 103 g
    let _g = 
    vec![
    vec![false, false, false],
    vec![false, true, true],
    vec![true, false, true],
    vec![false, true, true],
    vec![true, true, false]
    ];
    //ascii 104 h
    let _h = 
    vec![
    vec![true, false, false],
    vec![true, false, false],
    vec![true, true, false],
    vec![true, false, true],
    vec![true, false, true]
    ];
    //ascii 105 i
    let _i = 
    vec![
    vec![true],
    vec![false],
    vec![true],
    vec![true],
    vec![true]
    ];
    //ascii 106 j
    let _j = 
    vec![
    vec![false, true],
    vec![false, false],
    vec![false, true],
    vec![false, true],
    vec![true, false]
    ];
    //ascii 107 k
    let _k = 
    vec![
    vec![true, false, false],
    vec![true, false, false],
    vec![true, false, true],
    vec![true, true, false],
    vec![true, false, true]
    ];
    //ascii 108 l
    let _l = 
    vec![
    vec![true, false],
    vec![true, false],
    vec![true, false],
    vec![true, false],
    vec![false, true]
    ];
    //ascii 109 m
    let _m = 
    vec![
    vec![false, false, false, false, false],
    vec![false, false, false, false, false],
    vec![true, true, true, true, false],
    vec![true, false, true, false, true],
    vec![true, false, true, false, true]
    ];
    //ascii 110 n
    let _n = 
    vec![
    vec![false, false, false],
    vec![false, false, false],
    vec![true, true, false],
    vec![true, false, true],
    vec![true, false, true]
    ];
    //ascii 111 o
    let _o = 
    vec![
    vec![false, false, false],
    vec![false, false, false],
    vec![false, true, false],
    vec![true, false, true],
    vec![false, true, false]
    ];
    //ascii 112 p
    let _p = 
    vec![
    vec![false, false, false],
    vec![true, true, false],
    vec![true, false, true],
    vec![true, true, false],
    vec![true, false, false]
    ];
    //ascii 113 q
    let _q = 
    vec![
    vec![false, false, false],
    vec![false, true, true],
    vec![true, false, true],
    vec![false, true, true],
    vec![false, false, true]
    ];
    //ascii 114 r
    let _r = 
    vec![
    vec![false, false, false],
    vec![false, false, false],
    vec![false, true, true],
    vec![true, false, false],
    vec![true, false, false]
    ];
    //ascii 115 s
    let _s = 
    vec![
    vec![false, false, false],
    vec![false, true, true],
    vec![true, false, false],
    vec![false, false, true],
    vec![true, true, false]
    ];
    //ascii 116 t
    let _t = 
    vec![
    vec![false, false, false],
    vec![false, true, false],
    vec![true, true, true],
    vec![false, true, false],
    vec![false, true, true]
    ];
    //ascii 117 u
    let _u = 
    vec![
    vec![false, false, false],
    vec![false, false, false],
    vec![true, false, true],
    vec![true, false, true],
    vec![false, true, true]
    ];
    //ascii 118 v
    let _v = 
    vec![
    vec![false, false, false],
    vec![false, false, false],
    vec![true, false, true],
    vec![true, false, true],
    vec![false, true, false]
    ];
    //ascii 119 w
    let _w = 
    vec![
    vec![false, false, false, false, false],
    vec![false, false, false, false, false],
    vec![true, false, false, false, true],
    vec![true, false, true, false, true],
    vec![false, true, false, true, false]
    ];
    //ascii 120 x
    let _x = 
    vec![
    vec![false, false, false],
    vec![false, false, false],
    vec![true, false, true],
    vec![false, true, false],
    vec![true, false, true]
    ];
    //ascii 121 y
    let _y = 
    vec![
    vec![false, false, false],
    vec![true, false, true],
    vec![true, false, true],
    vec![false, true, true],
    vec![true, true, false]
    ];
    //ascii 122 z
    let _z = 
    vec![
    vec![false, false, false],
    vec![true, true, true],
    vec![false, true, false],
    vec![true, false, false],
    vec![true, true, true]
    ];
    //ascii 123 {
    let _left_brace = 
    vec![
    vec![false, true, true],
    vec![false, true, false],
    vec![true, true, false],
    vec![false, true, false],
    vec![false, true, true]
    ];
    //ascii 124 | pipe
    let _pipe = 
    vec![
    vec![true],
    vec![true],
    vec![true],
    vec![true],
    vec![true]
    ];
    //ascii 125 }
    let _right_brace = 
    vec![
    vec![true, true, false],
    vec![false, true, false],
    vec![false, true, true],
    vec![false, true, false],
    vec![true, true, false]
    ];
    //ascii 126 ~ tilde
    let _tilde = 
    vec![
    vec![false, false, false, false],
    vec![false, true, false, true],
    vec![true, false, true, false],
    vec![false, false, false, false],
    vec![false, false, false, false]
    ];
    




    // Register the remaining glyphs. These were all defined above but never inserted, so every
    // digit, letter and many symbols previously fell through to _unknown_char when looked up.
    map.insert(',', _comma);
    map.insert('-', _minus);
    map.insert('.', _period);
    map.insert('/', _l_slash);
    map.insert('0', _zero);
    map.insert('1', _one);
    map.insert('2', _two);
    map.insert('3', _three);
    map.insert('4', _four);
    map.insert('5', _five);
    map.insert('6', _six);
    map.insert('7', _seven);
    map.insert('8', _eight);
    map.insert('9', _nine);
    map.insert(':', _colon);
    map.insert(';', _semicolon);
    map.insert('<', _greater_than); // holds the ascii 60 '<' glyph (variable is misnamed)
    map.insert('=', _equals);
    map.insert('>', _less_than);    // holds the ascii 62 '>' glyph (variable is misnamed)
    map.insert('?', _question);
    map.insert('@', _at);
    map.insert('A', _A);
    map.insert('B', _B);
    map.insert('C', _C);
    map.insert('D', _D);
    map.insert('E', _E);
    map.insert('F', _F);
    map.insert('G', _G);
    map.insert('H', _H);
    map.insert('I', _I);
    map.insert('J', _J);
    map.insert('K', _K);
    map.insert('L', _L);
    map.insert('M', _M);
    map.insert('N', _N);
    map.insert('O', _O);
    map.insert('P', _P);
    map.insert('Q', _Q);
    map.insert('R', _R);
    map.insert('S', _S);
    map.insert('T', _T);
    map.insert('U', _U);
    map.insert('V', _V);
    map.insert('W', _W);
    map.insert('X', _X);
    map.insert('Y', _Y);
    map.insert('Z', _Z);
    map.insert('[', _left_bracket);
    map.insert('\\', _back_slash);
    map.insert(']', _right_bracket);
    map.insert('^', _hat);
    map.insert('_', _underscore);
    map.insert('`', _backtick);
    map.insert('a', _a);
    map.insert('b', _b);
    map.insert('c', _c);
    map.insert('d', _d);
    map.insert('e', _e);
    map.insert('f', _f);
    map.insert('g', _g);
    map.insert('h', _h);
    map.insert('i', _i);
    map.insert('j', _j);
    map.insert('k', _k);
    map.insert('l', _l);
    map.insert('m', _m);
    map.insert('n', _n);
    map.insert('o', _o);
    map.insert('p', _p);
    map.insert('q', _q);
    map.insert('r', _r);
    map.insert('s', _s);
    map.insert('t', _t);
    map.insert('u', _u);
    map.insert('v', _v);
    map.insert('w', _w);
    map.insert('x', _x);
    map.insert('y', _y);
    map.insert('z', _z);
    map.insert('{', _left_brace);
    map.insert('|', _pipe);
    map.insert('}', _right_brace);
    map.insert('~', _tilde);

    return map.get(&input)
        .cloned()
        .unwrap_or(_unknown_char)
}


pub fn add_padding(x: u8, y: u8) -> Vec<Vec<bool>>{
    let mut rtn: Vec<Vec<bool>> = Vec::new();
    for _ in 0..x {
        let mut line: Vec<bool> = Vec::new();
        for _ in 0..y {
            line.push(false);
        }
        rtn.push(line);
    }

    return rtn;
}