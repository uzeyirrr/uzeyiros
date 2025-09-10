use crate::kbd::getb;
use crate::cga::Cga;
use crate::println;

pub fn start_simple_shell() {
    let mut input_buffer = [0u8; 256];
    let mut buffer_pos = 0;
    
    loop {
        // Klavye girişini kontrol et
        if let Some(key) = getb() {
            if key == b'\n' || key == b'\r' {
                // Enter tuşuna basıldı
                input_buffer[buffer_pos] = 0;
                let command = core::str::from_utf8(&input_buffer[..buffer_pos]).unwrap_or("");
                handle_command(command);
                buffer_pos = 0;
                println!("uzeyiros> ");
            } else if key == 8 || key == 127 { // Backspace
                if buffer_pos > 0 {
                    buffer_pos -= 1;
                    println!("\x08 \x08"); // Backspace + space + backspace
                }
            } else if buffer_pos < 255 && key >= 32 && key <= 126 {
                // Yazdırılabilir karakter
                input_buffer[buffer_pos] = key;
                buffer_pos += 1;
                println!("{}", key as char);
            }
        }
        
        // CPU'yu biraz dinlendir
        unsafe {
            core::arch::asm!("pause");
        }
    }
}

fn handle_command(command: &str) {
    let cmd = command.trim();
    
    match cmd {
        "help" => {
            println!("\nUzeyiros Shell Komutlari:");
            println!("  help  - Bu yardim mesajini gosterir");
            println!("  clear - Ekrani temizler");
            println!("  echo  - Mesaj yazdirir");
            println!("  info  - Sistem bilgilerini gosterir");
            println!("  exit  - Cikis (henuz calismiyor)");
        }
        "clear" => {
            Cga::new().blank();
            println!("Hosgeldiniz! Ben Uzeyir Ismail Bahtiyar");
            println!("Uzeyiros Isletim Sistemi");
        }
        cmd if cmd.starts_with("echo ") => {
            let message = &cmd[5..]; // "echo " kısmını çıkar
            println!("\n{}", message);
        }
        "echo" => {
            println!("\nEcho komutu kullanimi: echo <mesaj>");
        }
        "info" => {
            println!("\nUzeyiros v1.0.0");
            println!("Rust ile yazilmis isletim sistemi");
            println!("x86_64 mimarisi");
            println!("CGA ekran modu");
        }
        "" => {
            // Boş komut, hiçbir şey yapma
        }
        _ => {
            println!("\nBilinmeyen komut: '{}'", cmd);
            println!("'help' yazarak kullanilabilir komutlari gorebilirsiniz.");
        }
    }
}
