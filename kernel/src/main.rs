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
        // Ana CPU için ekranı temizle ve hoşgeldin mesajı göster
        crate::cga::Cga::new().blank();
        println!("Hosgeldiniz! Ben Uzeyir Ismail Bahtiyar");
        println!("Uzeyiros Isletim Sistemi basariyla baslatildi!");
    }
    signal_up(semaphore);
    proc::scheduler();
}

fn signal_up(semaphore: &AtomicBool) {
    semaphore.store(true, Ordering::Release);
}

#[cfg(not(test))]
mod runtime {
    use super::{AtomicBool, Ordering};
    use core::panic::PanicInfo;

    static PANIC_SEQ: AtomicBool = AtomicBool::new(false);

    #[panic_handler]
    pub fn panic(info: &PanicInfo) -> ! {
        use crate::panic_println;
        panic_println!("@");
        while PANIC_SEQ.swap(true, Ordering::AcqRel) {}
        panic_println!("RUST PANIC: {:?}", info);
        PANIC_SEQ.store(false, Ordering::Release);
        #[allow(clippy::empty_loop)]
        loop {}
    }
}
