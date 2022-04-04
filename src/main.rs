//macro = function that writes code.
fn give_number(one: i32, two: i32) -> i32{
    one * two
}

fn print_number(one: i16, two: i16) -> i16{
    let mul = {
        let first = 10;
        first * one * two
    };
    mul
    
}

fn main() {
    let my_number = give_number(9,8);
    let zz = print_number(9,2);
    println!("{}", my_number);
    println!("{}",zz);

    

    

    //////////2/28
    // let city = "SEoul";
    // let year = 25;
    // println!("the city of {0} in{1} {0}", city=city,year=year);
    // println!("{city} {year}");
    

    ////////// 2/28
    // let my_number = 9.;  // f64
    // let other_number = 1; // i32

    // println!("{}", my_number as i32+other_number);


    ////////// 2/17
    // println!("size of a char: {}", std::mem::size_of::<char>()); //4bytes
    // println!("size of a a: {}", "a".len()); //1
    // println!("size of a aaa: {}", "aaa".len());

    // let slice = "Hello world! 안녕";
    // println!("Slice is {} bytes and {} character", slice.len(), slice.chars().count());




    ////////// 2/16
    // let x = 9; // 관심없음 => 쉐도윙
    // let x = double(x);
    // let x = triple(x);
    // println!("{} ㅋㅋㅋ",my_number);

    //코드블럭!
    // let my = 9;
    // {
    //     let my = "zzz";
    //     println!("{}", my)
    // }
    // println!("{}", my)
}
