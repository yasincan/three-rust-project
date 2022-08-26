fn test() {
    let mut number1: i8 = 127; //8 bit tamsayı alıyor => 2 üzeri 8 => yani  -127 ile 128 arası 
    // Buradaki mut immutable varsayılan olarak tüm değişkenler immutable oarak doğar yani mut opratörünün
    // aksi belirtilmedikçe ship oldukları veriler değişmez.
    println!("number1 sayısının değeri {}", number1);

    let number2: u8 = 256; // 8 bit pozitif tam sayı alıyor => yani 0-255 arası sayı alabilir.
    println!("number2 sayısını değeri {}", number2)
    
    number1 +=1; //1 arttırdığında 128 olur

    //DİĞER DEÜİŞKEN TANIMLAMALARI
    let corner1 = 1.234; // varsayılan olarak f32
    let corner: f32 = 1.23456; // 32 bitlik floating number
    let corner = 1_u8; // son ek vererek de değişkenin hangi türden olacağını söyleyebiliriz. u16,
}
