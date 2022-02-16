# rust_study
$ cargo run

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
