use static_cell::StaticCell;
use stm32f1xx_hal::{
    pac,
    serial::RxDma1,
};
use stm32f1xx_hal::dma::CircReadDma;
use crate::board::{Board};

const RX_BUFFER_LEN: usize = 50;
static RX_BUFFER_CELL: StaticCell<[u8; RX_BUFFER_LEN]> = StaticCell::new();

pub struct PollingReceiver {
    rx_dma: RxDma1,
    last_read_index: usize,
    buf: &'static mut [u8; RX_BUFFER_LEN],
}

impl PollingReceiver {
    pub fn new(rx: stm32f1xx_hal::serial::Rx<pac::USART1>, rx_dma_channel: stm32f1xx_hal::dma::dma1::C5) -> Self {
        let static_buf = RX_BUFFER_CELL.init([0; RX_BUFFER_LEN]);
        let rx_dma = rx.with_dma(rx_dma_channel);
        let rx_dma_stolen = unsafe { core::ptr::read(&rx_dma) };
        let two_d_buffer: &'static mut [[u8; RX_BUFFER_LEN/2]; 2] = unsafe {
          core::mem::transmute(&mut * static_buf)
        };
	unsafe {
	    let usart1 = &*pac::USART1::ptr();
	    let _sr = usart1.sr().read();
	    let _dr = usart1.dr().read();
	}
	let _circ_dma = rx_dma.circ_read(two_d_buffer);
        Self {
            rx_dma: rx_dma_stolen,
            last_read_index:RX_BUFFER_LEN-1,
	    buf: static_buf,
        }
    }
}

impl Iterator for PollingReceiver {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
	let remaining_bytes = self.rx_dma.channel.get_ndtr() as usize;
	if remaining_bytes == 0 {
            unsafe {
                let usart1 = &*pac::USART1::ptr();
                let _sr = usart1.sr().read();
                let _dr = usart1.dr().read();
            }
	    return None // should return an error
	}
        if self.last_read_index == remaining_bytes - 1 {
            unsafe {
                let usart1 = &*pac::USART1::ptr();
                let _sr = usart1.sr().read();
                let _dr = usart1.dr().read();
            }
	    return None
        }
        let byte = self.buf[RX_BUFFER_LEN - 1 - self.last_read_index];

        self.last_read_index = (self.last_read_index + RX_BUFFER_LEN - 1) % RX_BUFFER_LEN;
        Some(byte)
    }
}

pub fn poll_trx (b: &mut Board)
{
  while let Some(byte) = b.cat_rx.next() {
      defmt::info!("Byte empfangen: {:a}", byte & 0x7F );
  }
}
