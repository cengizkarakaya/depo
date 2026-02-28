use lib::*;
use std::io::{self, BufRead};

fn main() {
    ust_cizgi();


    println!("Bir satır metin girin:");

    let stdin = io::stdin();
    let mut input = String::new();

    // Tek satır oku
    stdin.lock()
        .read_line(&mut input)
        .expect("Satır okunamadı");

    let metin = input.trim_end(); // sondaki \n'i temizle

    // 1. Toplam karakter sayısı (Unicode scalar value)
    let toplam_karakter = metin.chars().count();

    // 2. Boşluk hariç karakter sayısı
    let bosluk_haric = metin.chars()
        .filter(|c| !c.is_whitespace())
        .count();

    // 3. Kelime sayısı
    // split_whitespace() → Unicode whitespace'leri doğru işler
    let kelime_sayisi = metin.split_whitespace().count();

    // 4. En uzun kelime (karakter sayısı bazında)
    let en_uzun_kelime = metin.split_whitespace()
        .max_by_key(|&kelime| kelime.chars().count())
        .unwrap_or("");  // boş input durumunda

    let en_uzun_uzunluk = en_uzun_kelime.chars().count();

    println!("\nSonuçlar:");
    println!("──────────────────────────────");
    println!("Toplam karakter sayısı      : {toplam_karakter}");
    println!("Boşluk hariç karakter sayısı : {bosluk_haric}");
    println!("Kelime sayısı               : {kelime_sayisi}");

    if !en_uzun_kelime.is_empty() {
        println!("En uzun kelime              : \"{en_uzun_kelime}\"");
        println!("En uzun kelimenin uzunluğu  : {en_uzun_uzunluk} karakter");
    } else {
        println!("En uzun kelime              : (hiç kelime yok)");
    }



    alt_cizgi();
}
