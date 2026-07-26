use core::sync::atomic::{AtomicU32, Ordering};
#[unsafe(no_mangle)]
#[used]
#[unsafe(link_section = ".noinit.BOOTLOADER_TAG")]
  
static BOOTLOADER_TAG: [AtomicU32; 2] = [AtomicU32::new(0), AtomicU32::new(0)];

pub fn jump_to_bootloader_flag()
{
    BOOTLOADER_TAG[0].store(0xDEAD_BEEF, Ordering::Relaxed);
    BOOTLOADER_TAG[1].store(0xCC00_FFEE, Ordering::Relaxed);    
}

pub fn jump_to_bootloader() -> !
{
    defmt::info!("Resetting into Bootloader");    
    jump_to_bootloader_flag();
    panic!("Panic halting CPU to trigger watchdog");
    // statt panic loop { cortex_m::asm::wfi(); } 
}
