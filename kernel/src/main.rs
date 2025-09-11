#![allow(clippy::upper_case_acronyms)]
#![allow(internal_features)]
#![feature(core_intrinsics)]
//#![feature(proc_macro_hygiene)]
#![feature(sync_unsafe_cell)]
#![cfg_attr(test, allow(dead_code))]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![forbid(unsafe_op_in_unsafe_fn)]

mod acpi;
mod bio;
mod cga;
mod console;
mod exec;
mod file;
mod fs;
mod fslog;
mod initcode;
mod ioapic;
mod kalloc;
mod kbd;
mod kmem;
mod param;
mod pci;
mod pipe;
mod proc;
mod sd;
mod sleeplock;
mod smp;
mod spinlock;
mod syscall;
mod sysfile;
mod trap;
mod uart;
mod vm;
mod volatile;
mod x86_64;
mod xapic;

#[cfg(test)]
use std::{print, println};

use crate::vm::PageTable;
use crate::x86_64 as arch;
use arch::CPU;
use arch::Page;
#[cfg(all(target_arch = "x86_64", target_os = "none"))]
use arch::pic as PIC;
use core::result;
use core::sync::atomic::{AtomicBool, Ordering};

type Result<T> = result::Result<T, &'static str>;

/// Marks that a type can be safely cast from all zeroes.
/// 
/// # Safety
/// All zero bit patterns must be valid for type T.
pub unsafe trait FromZeros {}

unsafe impl<T: ?Sized> FromZeros for *const T {}
unsafe impl<T: ?Sized> FromZeros for *mut T {}
unsafe impl FromZeros for bool {}
unsafe impl FromZeros for char {}
unsafe impl FromZeros for f32 {}
unsafe impl FromZeros for f64 {}
unsafe impl FromZeros for isize {}
unsafe impl FromZeros for usize {}
unsafe impl FromZeros for i8 {}
unsafe impl FromZeros for u8 {}
unsafe impl FromZeros for i16 {}
unsafe impl FromZeros for u16 {}
unsafe impl FromZeros for i32 {}
unsafe impl FromZeros for u32 {}
unsafe impl FromZeros for i64 {}
unsafe impl FromZeros for u64 {}

#[cfg(all(target_arch = "x86_64", target_os = "none"))]
static mut PERCPU0: Page = Page::empty();
fn kpgtbl() -> &'static mut PageTable {
    static mut KPGTBL: PageTable = PageTable::empty();
    let kpgtbl = &raw mut KPGTBL;
    unsafe { &mut *kpgtbl }
}

/// # Safety
///
/// Starting an operating system is inherently unsafe.
#[cfg(all(target_arch = "x86_64", target_os = "none"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(boot_info: u64) {
    unsafe {
        CPU::init(&mut *(&raw mut PERCPU0), 0);
        console::init();
        PIC::init();
        trap::vector_init();
        trap::init();
        kalloc::early_init(kmem::early_pages());
        kmem::early_init(boot_info);
        vm::init(kpgtbl());
        kpgtbl().switch();
        acpi::init();
        ioapic::init(acpi::ioapics());
        xapic::init();
        kbd::init();
        uart::init();
        // Note: pci::init() calls sd::init.
        pci::init(kpgtbl());
        bio::init();
        pipe::init();
        syscall::init();
        smp::init();
        smp::start_others(acpi::cpus());
        kmem::init();
        proc::init(kpgtbl());
    }

    let semaphore = AtomicBool::new(false);
    mpmain(0, &semaphore);
}

/// # Safety
///
/// Starting a CPU is inherently unsafe.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mpenter(percpu: &mut Page, id: u32, semaphore: &AtomicBool) {
    unsafe {
        CPU::init(percpu, id);
        trap::init();
        kpgtbl().switch();
        xapic::init();
        syscall::init();
    }
    mpmain(id, semaphore)
}

fn mpmain(id: u32, semaphore: &AtomicBool) {
    println!("cpu{} starting", id);
    
    if id == 0 {
        // Ana CPU: GUI'yi başlat
        let mut cga = crate::cga::Cga::new();
        cga.blank();
        start_simple_shell();
    } else {
        // Diğer CPU'lar: Sinyal gönder ve bekle
        signal_up(semaphore);
        loop {
            unsafe {
                core::arch::asm!("pause");
            }
        }
    }
}



fn start_simple_shell() {
    use crate::cga::Cga;
    
    let mut cga = Cga::new();
    
    // GUI'yi başlat
    start_gui(&mut cga);
    
    // Basit klavye input loop
    let mut input_buffer = [0u8; 256];
    let mut input_pos = 0;
    let mut prompt_x = 5;
    let mut prompt_y = 22;
    let prompt_text = "uzeyiros> ";
    let input_start_x = prompt_x + prompt_text.len();
    
    loop {
        // Klavye input kontrolü (basit polling)
        if let Some(key) = check_keyboard_input() {
            if key == b'\r' || key == b'\n' {
                // Enter tuşu - komutu işle
                if input_pos > 0 {
                    let command = core::str::from_utf8(&input_buffer[..input_pos]).unwrap_or("");
                    let output_lines = handle_command(command);
                    input_pos = 0;
                    
                    // Prompt'u aşağıya kaydır
                    prompt_y += output_lines + 1;
                    if prompt_y > 20 { // Ekranın altına çıkmasın
                        prompt_y = 22;
                    }
                    
                    // Prompt'u yeni konumda göster
                    cga.put_string_at(prompt_x, prompt_y, prompt_text);
                }
            } else if key == 8 || key == 127 { // Backspace
                if input_pos > 0 {
                    input_pos -= 1;
                    cga.put_char_at(input_start_x + input_pos, prompt_y, b' ');
                }
            } else if input_pos < 255 && key >= 32 && key <= 126 {
                // Normal karakter - prompt'un yanına yaz
                input_buffer[input_pos] = key;
                cga.put_char_at(input_start_x + input_pos, prompt_y, key);
                input_pos += 1;
            }
        }
        
        // CPU'yu biraz dinlendir
        unsafe {
            core::arch::asm!("pause");
        }
    }
}

fn start_gui(cga: &mut crate::cga::Cga) {
    // Ekranı temizle
    cga.blank();
    
// ASCII Logo - UzeyirOS (Yeni Tasarım)
let logo_lines = [
    "                                                                 ",
    "                                                                 ",
    " _|    _|                                                        ",
    "                                        _|                       ",
    " _|    _|  _|_|_|_|    _|_|    _|    _|      _|  _|_|            ",
    " _|    _|      _|    _|_|_|_|  _|    _|  _|  _|_|                ",
    " _|    _|    _|      _|        _|    _|  _|  _|                  ",
    "   _|_|    _|_|_|_|    _|_|_|    _|_|_|  _|  _|                  ",
    "                                     _|                          ",
    "                                 _|_|                            ",
    "                                                                 ",
    "                     UzeyirOS v1.4.0                             ",
    "                                                                 ",
];

    
    // Logoyu ekrana yazdır
    for (i, line) in logo_lines.iter().enumerate() {
        if i < 25 { // Ekran yüksekliği sınırı
            cga.put_string_at(0, i, line);
        }
    }
    
    // Sistem bilgileri kutusu (logo'nun altında)
    cga.draw_box(5, 15, 70, 6);
    cga.put_string_at(7, 16, "Uzeyiros v1.1.0 - Personal Operating System");
    cga.put_string_at(7, 17, "Author: Uzeyir Ismail Bahtiyar | Email: uzeyirismailbahtiyar@gmail.com");
    cga.put_string_at(7, 18, "Language: Rust | Architecture: x86_64 | Display: CGA");
    cga.put_string_at(7, 19, "Status: Running | GUI: Active | Shell: Ready");
    cga.put_string_at(7, 20, "Commands: help, portfolio, about, contact, skills, projects, info, clear, exit");
    
    // Prompt
    cga.put_string_at(5, 22, "uzeyiros> ");
}

fn check_keyboard_input() -> Option<u8> {
    // Basit PS/2 klavye polling
    unsafe {
        // Port 0x64'ten status byte'ı oku
        let status: u8;
        core::arch::asm!("in al, 0x64", out("al") status);
        
        // Data ready bit kontrolü (bit 0)
        if status & 1 != 0 {
            // Port 0x60'tan keycode oku
            let keycode: u8;
            core::arch::asm!("in al, 0x60", out("al") keycode);
            
            // Basit keycode to ASCII dönüşümü
            match keycode {
                0x1C => Some(b'\r'), // Enter
                0x0E => Some(8),     // Backspace
                0x02 => Some(b'1'),
                0x03 => Some(b'2'),
                0x04 => Some(b'3'),
                0x05 => Some(b'4'),
                0x06 => Some(b'5'),
                0x07 => Some(b'6'),
                0x08 => Some(b'7'),
                0x09 => Some(b'8'),
                0x0A => Some(b'9'),
                0x0B => Some(b'0'),
                0x10 => Some(b'q'),
                0x11 => Some(b'w'),
                0x12 => Some(b'e'),
                0x13 => Some(b'r'),
                0x14 => Some(b't'),
                0x15 => Some(b'y'),
                0x16 => Some(b'u'),
                0x17 => Some(b'i'),
                0x18 => Some(b'o'),
                0x19 => Some(b'p'),
                0x1E => Some(b'a'),
                0x1F => Some(b's'),
                0x20 => Some(b'd'),
                0x21 => Some(b'f'),
                0x22 => Some(b'g'),
                0x23 => Some(b'h'),
                0x24 => Some(b'j'),
                0x25 => Some(b'k'),
                0x26 => Some(b'l'),
                0x2C => Some(b'z'),
                0x2D => Some(b'x'),
                0x2E => Some(b'c'),
                0x2F => Some(b'v'),
                0x30 => Some(b'b'),
                0x31 => Some(b'n'),
                0x32 => Some(b'm'),
                0x39 => Some(b' '), // Space
                _ => None,
            }
        } else {
            None
        }
    }
}

fn handle_command(command: &str) -> usize {
    use crate::cga::Cga;
    let mut cga = Cga::new();
    let cmd = command.trim();
    
    // Çıktıları daha aşağıya yaz (satır 22'den başla)
    let mut output_line = 22;
    
    match cmd {
        "help" => {
            cga.put_string_at(5, output_line, "Uzeyiros Shell Commands:");
            output_line += 1;
            cga.put_string_at(5, output_line, "  help      - Show this help message");
            output_line += 1;
            cga.put_string_at(5, output_line, "  clear     - Clear screen");
            output_line += 1;
            cga.put_string_at(5, output_line, "  echo      - Print message");
            output_line += 1;
            cga.put_string_at(5, output_line, "  info      - Show system info");
            output_line += 1;
            cga.put_string_at(5, output_line, "  portfolio - Portfolio main menu");
            output_line += 1;
            cga.put_string_at(5, output_line, "  about     - About me");
            output_line += 1;
            cga.put_string_at(5, output_line, "  contact   - Contact info");
            output_line += 1;
            cga.put_string_at(5, output_line, "  skills    - Technical skills");
            output_line += 1;
            cga.put_string_at(5, output_line, "  projects  - My projects");
            output_line += 1;
            cga.put_string_at(5, output_line, "  exit      - Return to main GUI");
            output_line - 22 // 10 satır
        }
        "clear" => {
            cga.blank();
            start_gui(&mut cga);
            0 // clear komutu prompt'u değiştirmez
        }
        cmd if cmd.starts_with("echo ") => {
            let message = &cmd[5..];
            cga.put_string_at(5, output_line, "Echo: ");
            cga.put_string_at(11, output_line, message);
            1 // 1 satır
        }
        "echo" => {
            cga.put_string_at(5, output_line, "Echo usage: echo <message>");
            1 // 1 satır
        }
        "info" => {
            cga.put_string_at(5, output_line, "Uzeyiros v1.1.0");
            output_line += 1;
            cga.put_string_at(5, output_line, "Operating system written in Rust");
            output_line += 1;
            cga.put_string_at(5, output_line, "x86_64 architecture");
            3 // 3 satır
        }
        "portfolio" => {
            start_portfolio_gui(&mut cga);
            0
        }
        "about" => {
            start_about_gui(&mut cga);
            0
        }
        "contact" => {
            start_contact_gui(&mut cga);
            0
        }
        "skills" => {
            start_skills_gui(&mut cga);
            0
        }
        "projects" => {
            start_projects_gui(&mut cga);
            0
        }
        "exit" => {
            start_gui(&mut cga);
            0 // Ana GUI'ye dön
        }
        "" => {
            0 // Boş komut
        }
        _ => {
            cga.put_string_at(5, output_line, "Unknown command: '");
            cga.put_string_at(25, output_line, cmd);
            cga.put_string_at(25 + cmd.len(), output_line, "'");
            output_line += 1;
            cga.put_string_at(5, output_line, "Type 'help' to see available commands.");
            2 // 2 satır
        }
    }
}

fn signal_up(semaphore: &AtomicBool) {
    semaphore.store(true, Ordering::Release);
}

fn start_portfolio_gui(cga: &mut crate::cga::Cga) {
    cga.blank();
    
    // Ana başlık
    cga.put_string_at(5, 2, "+==============================================================================+");
    cga.put_string_at(5, 3, "|                    UZEYIR ISMAIL BAHTIYAR                        |");
    cga.put_string_at(5, 4, "|                        PORTFOLIO                                 |");
    cga.put_string_at(5, 5, "+==============================================================================+");
    
    // Menü seçenekleri
    cga.put_string_at(10, 8, "1. About Me (about)");
    cga.put_string_at(10, 9, "2. Contact Info (contact)");
    cga.put_string_at(10, 10, "3. Technical Skills (skills)");
    cga.put_string_at(10, 11, "4. My Projects (projects)");
    cga.put_string_at(10, 12, "5. System Info (info)");
    cga.put_string_at(10, 13, "6. Return to Main Menu (exit)");
    
    // Alt çerçeve
    cga.put_string_at(5, 15, "+==============================================================================+");
    cga.put_string_at(5, 16, "|  Type command: about, contact, skills, projects, info, exit        |");
    cga.put_string_at(5, 17, "+==============================================================================+");
}

fn start_about_gui(cga: &mut crate::cga::Cga) {
    cga.blank();
    
    // Başlık
    cga.put_string_at(5, 2, "+==============================================================================+");
    cga.put_string_at(5, 3, "|                                  ABOUT ME                                  |");
    cga.put_string_at(5, 4, "+==============================================================================+");
    
    // İçerik
    cga.put_string_at(10, 6, "Hello! I am Uzeyir Ismail Bahtiyar.");
    cga.put_string_at(10, 7, "I am a passionate software developer.");
    cga.put_string_at(10, 8, "");
    cga.put_string_at(10, 9, "I have experience in system programming,");
    cga.put_string_at(10, 10, "operating system development, and");
    cga.put_string_at(10, 11, "low-level programming.");
    cga.put_string_at(10, 12, "");
    cga.put_string_at(10, 13, "This UzeyirOS project is part of my journey");
    cga.put_string_at(10, 14, "in developing my own operating system");
    cga.put_string_at(10, 15, "using Rust for bare-metal programming.");
    
    // Alt çerçeve
    cga.put_string_at(5, 17, "+==============================================================================+");
    cga.put_string_at(5, 18, "|                    Type 'exit' to return to main menu                      |");
    cga.put_string_at(5, 19, "+==============================================================================+");
}

fn start_contact_gui(cga: &mut crate::cga::Cga) {
    cga.blank();
    
    // Başlık
    cga.put_string_at(5, 2, "+==============================================================================+");
    cga.put_string_at(5, 3, "|                                CONTACT                                   |");
    cga.put_string_at(5, 4, "+==============================================================================+");
    
    // İletişim bilgileri
    cga.put_string_at(10, 6, "Email: uzeyirismailbahtiyar@gmail.com");
    cga.put_string_at(10, 7, "GitHub: https://github.com/uzeyirrr");
    cga.put_string_at(10, 8, "LinkedIn: https://www.linkedin.com/in/uzeyirismail/");
    cga.put_string_at(10, 9, "Website: yezuri.com");
    cga.put_string_at(10, 10, "");
    cga.put_string_at(10, 11, "Feel free to contact me to discuss");
    cga.put_string_at(10, 12, "projects or potential collaborations!");
    
    // Alt çerçeve
    cga.put_string_at(5, 14, "+==============================================================================+");
    cga.put_string_at(5, 15, "|                    Type 'exit' to return to main menu                      |");
    cga.put_string_at(5, 16, "+==============================================================================+");
}

fn start_skills_gui(cga: &mut crate::cga::Cga) {
    cga.blank();
    
    // Başlık
    cga.put_string_at(5, 2, "+==============================================================================+");
    cga.put_string_at(5, 3, "|                            TECHNICAL SKILLS                             |");
    cga.put_string_at(5, 4, "+==============================================================================+");
    
    // Yetenekler
    cga.put_string_at(10, 6, "Rust - System programming, bare-metal");
    cga.put_string_at(10, 7, "Linux - System administration, shell scripting");
    cga.put_string_at(10, 8, "C/C++ - System programming, embedded");
    cga.put_string_at(10, 9, "Python - Web development, automation");
    cga.put_string_at(10, 10, "JavaScript/TypeScript - Frontend/Backend");
    cga.put_string_at(10, 11, "SQL - Database design and management");
    cga.put_string_at(10, 12, "Docker - Containerization");
    cga.put_string_at(10, 13, "Cloud - AWS, Azure");
    cga.put_string_at(10, 14, "Git - Version control");
    cga.put_string_at(10, 15, "Mobile - React Native");
    
    // Alt çerçeve
    cga.put_string_at(5, 17, "+==============================================================================+");
    cga.put_string_at(5, 18, "|                    Type 'exit' to return to main menu                      |");
    cga.put_string_at(5, 19, "+==============================================================================+");
}

fn start_projects_gui(cga: &mut crate::cga::Cga) {
    cga.blank();
    
    // Başlık
    cga.put_string_at(5, 2, "+==============================================================================+");
    cga.put_string_at(5, 3, "|                               MY PROJECTS                               |");
    cga.put_string_at(5, 4, "+==============================================================================+");
    
    // Projeler
    cga.put_string_at(10, 6, "UzeyirOS - Operating system written in Rust");
    cga.put_string_at(10, 7, "   * Bare-metal programming");
    cga.put_string_at(10, 8, "   * Multi-processor support");
    cga.put_string_at(10, 9, "   * Interactive GUI and shell");
    cga.put_string_at(10, 10, "");
    cga.put_string_at(10, 11, "Web Applications");
    cga.put_string_at(10, 12, "   * React/Node.js projects");
    cga.put_string_at(10, 13, "   * RESTful APIs");
    cga.put_string_at(10, 14, "");
    cga.put_string_at(10, 15, "Mobile Applications");
    cga.put_string_at(10, 16, "   * React Native projects");
    cga.put_string_at(10, 17, "   * Cross-platform solutions");
    
    // Alt çerçeve
    cga.put_string_at(5, 19, "+==============================================================================+");
    cga.put_string_at(5, 20, "|                    Type 'exit' to return to main menu                      |");
    cga.put_string_at(5, 21, "+==============================================================================+");
}

#[cfg(not(test))]
mod runtime {
    use super::{AtomicBool, Ordering};
    use core::panic::PanicInfo;

    static PANIC_SEQ: AtomicBool = AtomicBool::new(false);

    #[panic_handler]
    pub fn panic(info: &PanicInfo) -> ! {
        use crate::panic_println;
        
        // Deadlock riskini azaltmak için basit bir yaklaşım
        panic_println!("@");
        
        // Sadece bir kez panic mesajını yazdır
        if !PANIC_SEQ.load(Ordering::Acquire) {
            PANIC_SEQ.store(true, Ordering::Release);
            panic_println!("RUST PANIC: {:?}", info);
        }
        
        // Sonsuz döngü - sistem durdur
        #[allow(clippy::empty_loop)]
        loop {
            unsafe {
                core::arch::asm!("cli"); // Interrupt'ları kapat
                core::arch::asm!("hlt"); // CPU'yu durdur
            }
        }
    }
}
