use stm32f1xx_hal::rcc;
use cortex_m::Peripherals;
use crate::hal::stm32f1xx_hal::pac::GPIOC;

pub fn test()
{
    unsafe {
        let gpioc = &*GPIOC::ptr();
        
        // Write directly to the Bit Set/Reset Register (BSRR)
        // This sets Pin 5 HIGH and Pin 6 LOW
        gpioc.bsrr().write(|w| w.bs13().set_bit());
        
        // Read directly from the Input Data Register (IDR)
//        let _is_high = gpioa.idr.read().idr3().bit_is_set();
    }
}
pub fn test2()
{
        unsafe {
//            let dp = Peripherals::steal();
//            let cp = cortex_m::Peripherals::take().unwrap();
//            dp.GPIOC.bsrr().write(|w| w.bs13().set_bit());
    }
}    
