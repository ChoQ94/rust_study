

fn main() {

    ////////// 2/17
    println!("size of a char: {}", std::mem::size_of::<char>()); //4bytes
    println!("size of a a: {}", "a".len()); //1
    println!("size of a aaa: {}", "aaa".len());

    let slice = "Hello world! 안녕";
    println!("Slice is {} bytes and {} character", slice.len(), slice.chars().count());




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
