use static_cell::StaticCell;

use crate::hal::{*};
use crate::hal;
use stm32f1xx_hal::dma::WriteDma;

pub struct DynamicDmaBuffer {
    pub buf: &'static mut [u8; CAT_TX_BUFFER_LEN],
    pub len: usize,
}

unsafe impl embedded_dma::ReadBuffer for DynamicDmaBuffer {
    type Word = u8;

    unsafe fn read_buffer(&self) -> (*const u8, usize) {
        (self.buf.as_ptr(), self.len)
    }
}

type UartTxTransfer = hal::dma::Transfer<
    hal::dma::R,
    DynamicDmaBuffer,
    serial::TxDma1
>;

pub enum TxState {
    Ready {
	tx: serial::TxDma1,
	buf: DynamicDmaBuffer,
    },
    Busy(UartTxTransfer),
    Locked,
}
use TxState::Ready;
use TxState::Busy;
pub use TxState::Locked;

const CAT_TX_BUFFER_LEN : usize = 50;
static TX_BUFFER_CELL: StaticCell<[u8; CAT_TX_BUFFER_LEN]>= StaticCell::new();

pub fn init_tx (tx : serial::TxDma1) -> TxState
{
    Ready {
	tx: tx,
	buf: DynamicDmaBuffer {
	    buf: TX_BUFFER_CELL.init([0; CAT_TX_BUFFER_LEN]),
            len: 0,
	},
    }
}

pub fn send_uart
    (mut tx_state: TxState, bytes: &[u8])
    -> TxState
{
	if let Busy(ref tx) = tx_state {
	    if tx.is_done() {
		if let Busy(tx) = tx_state {
		    let (buf, tx_new) = tx.wait();
		    tx_state = Ready {
			tx: tx_new,
			buf: buf
		    };
		}
	    }
	}
        match tx_state {
	    Ready{tx, mut buf} => {
		let len = bytes.len().min(CAT_TX_BUFFER_LEN);
		buf.buf[..len].copy_from_slice(&bytes[..len]);
                buf.len = len;
		let transfer = tx.write(buf);
		tx_state = Busy(transfer);
	    }
	    Busy(_) => {
	    }
	    Locked => {
	    }
	}
    tx_state
}
