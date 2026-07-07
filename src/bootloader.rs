use stm32g4xx_hal::pac;

pub fn jump_to_st_bootloader() -> ! {
    cortex_m::interrupt::disable();

    let mut core = unsafe { cortex_m::Peripherals::steal() };
    core.SYST.disable_counter();
    core.SYST.disable_interrupt();
    for i in 0..8 {
	unsafe {
          core.NVIC.icer[i].write(0xFFFFFFFF);
          core.NVIC.icpr[i].write(0xFFFFFFFF);
        }
    }

    // 4. Clocks auf Standard zurücksetzen (HSRUN/PLL abschalten, zurück auf HSI)
    let dp = unsafe { pac::Peripherals::steal() };
    // Schaltet PLL aus und setzt HSI als System-Clock-Quelle (Werkszustand)
    dp.RCC.cr().modify(|_, w| w.pllon().clear_bit());
    dp.RCC.cfgr().modify(|_, w| w.sw().hsi());

    dp.SYSCFG.memrmp().modify(|_, w| unsafe { w.mem_mode().bits(0b01) });
    
    // 5. Speicheradresse des Bootloaders für den STM32G474 definieren
//    const BOOTLOADER_ADDR: u32 = 0x1FFF_0000;

    unsafe {
	let msp_ptr = 0x0000_0000 as *const u32;
        let reset_ptr = 0x0000_0004 as *const u32;

        // read_volatile liest die Hardware-Werte absolut sicher ein
        let initial_msp = core::ptr::read_volatile(msp_ptr) as *const u32;
        let reset_handler_addr = core::ptr::read_volatile(reset_ptr);

        cortex_m::asm::bootstrap(initial_msp, reset_handler_addr as *const u32);
    }
}
