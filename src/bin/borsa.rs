use lib::*;

fn main() {
    ust_cizgi();

    // ------------------------------------------------------------
    // BAŞLIK
    // ------------------------------------------------------------

    println!("{}{}", MEDIUMPURPLE, "-".repeat(52));
    println!("{}{}", PALE_TURQUOISE, "🦀".repeat(26));
    println!("{}{}", MEDIUMPURPLE, "-".repeat(52));
    println!("{}", RESET);

    // ------------------------------------------------------------
    // VERİLER
    // ------------------------------------------------------------

    let vade: u8 = 5;
    let mut anapara: f64 = 7600.0;
    let yillik_artis: f64 = 0.50;
    let usd: f64 = 35.0;

    // ------------------------------------------------------------
    // ANAPARA
    // ------------------------------------------------------------

    println!("{}{}", MEDIUMPURPLE, "-".repeat(26));
    println!(
        "{}{}Anapara:{} {}",
        BOLD,
        CYAN,
        RESET,
        anapara
    );
    println!("{}", RESET);

    // ------------------------------------------------------------
    // YILLAR
    // ------------------------------------------------------------

    for yil in 1..=vade {
        anapara += anapara * yillik_artis;

        println!(
            "{}{:2}. yıl sonunda:{} {}",
            CYAN,
            yil,
            RESET,
            anapara.round()
        );
    }

    // ------------------------------------------------------------
    // TL HESABI
    // ------------------------------------------------------------

    let toplam_tl_milyon =
        ((anapara * usd) / 1_000_000.0 * 100.0).round() / 100.0;

    // ------------------------------------------------------------
    // SONUÇ
    // ------------------------------------------------------------

    println!();
    println!("{}{}", MEDIUMPURPLE, "-".repeat(26));
    println!(
        "{}Toplam para:{} {} ({} milyon ₺)",
        BOLD,
        RESET,
        anapara.round(),
        toplam_tl_milyon
    );
    println!("{}{}", MEDIUMPURPLE, "-".repeat(26));
    println!("{}", RESET);

    // ------------------------------------------------------------
    // ALT BAŞLIK
    // ------------------------------------------------------------

    println!("{}{}", MEDIUMPURPLE, "-".repeat(52));
    println!("{}{}", PALE_TURQUOISE, "🦀".repeat(26));
    println!("{}{}", MEDIUMPURPLE, "-".repeat(52));
    println!("{}", RESET);

    alt_cizgi();
}
