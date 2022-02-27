# rust_study
$ cargo run

## 2/17
[5강]
    println!("size of a char: {}", std::mem::size_of<char>()); //4bytes
        standard, memory 
        size_of => 사이즈 구하는 함수!!
        최상단에 use std::mem::size_of; 를 선언해서  size_of 만 사용 가능

    .len() 은 gives the size of the sting in bytes
    *u8 = 8 bites =  1byte  일반적인 아스키 코드는 1byte

    .chars 는 "Hello" => "H""E""L" .. 식으로 변경,   . count() 하면 그 개수셈(공백 포함)
## 2/16
[3강]
    unsigned
    signed 

    let my_number: u8 = 100; //i8 u30?? 타입 모름.
        let my_other_number = 50;
        let third_number = my_number + my_other_number;
        //뒤 부분을 알아서 u8 으로 판단하여 계산됨.

[4강]
    mutability => 변경가능
    shadowing => 같은 이름을 다시 쓰는 것

(mutability)
    immutable by default
    mut

    let my = 10;
    my = 9;  
         변경 불가
    let mut my_number: u8 = 100; 
    my = 9
        변경 가능
(shadowing)
    let vari = 10;
    vari = 9;      (x)

    let varu = 10;
    let varu = "zzzz";    (o)
