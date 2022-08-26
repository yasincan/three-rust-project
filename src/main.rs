use std::collections::HashMap;

fn main() {
    let introduction = String::from("Merhaba yasin bugün tarig 26.08.2022 Doğum günğün kutlu olsun. Nasılsın ?");
    print_sysmsg(introduction.to_string());

    let motto = "Rust çok efektif bir dil";
    print_sysmsg2(&motto);

    let mut points: [i8; 5] = [0, 8, 3, 8, 35];
    for point in points.iter_mut() {
        *point+=1;//. Referans edilen değişkeni * operatörü ile dereference ederek devam edelim.
        println!("{}", point) 
    }

    print!("\n");

    let colors: Vec<&str> = vec!["sarı", "kırmızı", "mavi"];
    for color in colors.iter() {
         println!("{}\t", color);
    }

    println!("\n");

    let mut color_codes: HashMap<&str, u8> = HashMap::new();
    color_codes.insert("Turuncu",10);
    color_codes.insert("Beyaz", 20);
    color_codes.insert("Mavi", 30);

    match color_codes.get("Mavi") {
        Some(code) => {
            println!("Mavi renk kodu bulundu {}", code)
        }
        None => println!("Renk kodu bulunamadı"),
    }
}

fn print_sysmsg(message: String){
    println!("Sistem mesajı, \n{}", message);
    /*
    Sonunda ! işareti olan fonksiyonlar macro olarak isimlendirilir.
    Macro’lar n sayıda parametre alabilirler ama daha da önemlisi meta programming için kullanılırlar.
    Böylece rust ile rust kodları yazabilir, derleme aşamasında işletilebilir kod bloklarını programa ekleyebiliriz(Ön işlemci direktifleri gibi) 
    */
}

fn print_sysmsg2(message: &str){
    println!("Sistem mesajı, \n{}", message);
}
