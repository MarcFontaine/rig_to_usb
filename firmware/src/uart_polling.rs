use static_cell::StaticCell;
use stm32f1xx_hal::{
    pac,
    serial::RxDma1,
};

const RX_BUF_LEN: usize = 50;
static RX_BUFFER_CELL: StaticCell<[u8; RX_BUF_LEN]> = StaticCell::new();

pub struct PollingReceiver {
    rx_dma: RxDma1,
    last_read_index: usize,
}

impl PollingReceiver {
    pub fn new(rx: stm32f1xx_hal::serial::Rx<pac::USART1>, rx_dma_channel: stm32f1xx_hal::dma::dma1::C5) -> Self {
        let static_buf = RX_BUFFER_CELL.init([0; RX_BUF_LEN]);
        // Startet die DMA im zirkulären Modus im Hintergrund
        let rx_dma = rx.with_dma(rx_dma_channel);
        let rx_dma = rx_dma.read_circular(static_buf);

        Self {
            rx_dma,
            last_read_index: 0,
        }
    }

    /// Diese Funktion rufen Sie einfach in Ihrer Hauptschleife auf
    pub fn update(&mut self) {
        // 1. Hardware-Counter abfragen (zählt von 50 rückwärts gegen 0)
        let remaining_bytes = self.rx_dma.channel.get_remaining_transfers() as usize;
        let current_hardware_index = RX_BUF_LEN - remaining_bytes;

        // 2. Prüfen, ob neue Daten im RAM gelandet sind
        if current_hardware_index != self.last_read_index {
            let buf_ptr = RX_BUFFER_CELL.as_ptr();
            let full_buffer = unsafe { &*buf_ptr };

            // 3. Neue Bytes exakt ausschneiden (unter Berücksichtigung des Ringpuffer-Umbruchs)
            if current_hardware_index > self.last_read_index {
                let new_data = &full_buffer[self.last_read_index..current_hardware_index];
                process_bytes(new_data);
            } else {
                let part1 = &full_buffer[self.last_read_index..RX_BUF_LEN];
                let part2 = &full_buffer[0..current_hardware_index];
                process_bytes(part1);
                process_bytes(part2);
            }

            // Lese-Zeiger auf den aktuellen Stand nachziehen
            self.last_read_index = current_hardware_index;
        }

        // 4. IDLE-Flag per Hardware-Read löschen, um der USART-Hardware
        // zu signalisieren, dass wir die Leitung überprüft haben
        unsafe {
            let usart1 = &*pac::USART1::ptr();
            let _sr = usart1.sr.read();
            let _dr = usart1.dr.read();
        }
    }
}

fn process_bytes(bytes: &[u8]) {
    // Hier kommen Ihre Daten an – Byte für Byte, sobald sie im RAM liegen!
}
