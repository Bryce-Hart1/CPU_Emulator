
// Helps with things like font rendering, when the cpu writes to the screen address, values can be mapped here.
//when a unknown value is written into screen.rs, it can call apon functions here to help

use std::collections::HashMap;


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
        vec![false],
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
    let _precent = 
    vec![
    vec![true, false, false, false, true],
    vec![false, false, false, true, false],
    vec![false, false, true, false, false],
    vec![false, true, false, false, false],
    vec![true, false, false, false, true]    
    ]; 

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

    //ascii 43 + 
   let _plus = 
    vec![
    vec![false, false, false],
    vec![false, true, false],
    vec![true, true, true],
    vec![false, true, false],
    vec![false, false, false]
    ];    

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
    vec![false, true, false],
    vec![false, true, false],
    vec![false, true, false],
    vec![true, false, false]
    ];  


    //ascii 48 0
    let _zero = 
    vec![
    vec![true, true, true],
    vec![true, false, true],
    vec![true, false, true],
    vec![true, false, true],
    vec![true, true, true]
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
    map.insert(':', _colon);

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

    rtn
}