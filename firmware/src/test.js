// Datenstruktur: [int, bool] entsprechend der #[n(0)] und #[n(1)] Definition in Rust
// Ein CBOR-Array für die Werte: id=42, status=true
// In hexadezimalem CBOR entspricht das: 0x82 0x18 0x2a 0xf5 (Array mit 2 Elementen, 42, true)
const cborBytes = new Uint8Array([0x82, 0x18, 0x2a, 0xf5]);

// Einen leeren 64-Byte HID-Report vorbereiten
const usbReport = new Uint8Array(64);
usbReport.set(cborBytes, 0); // CBOR an den Anfang kopieren

// Senden an das geöffnete HID-Device (Report ID 0x00 für Raw HID)
await device.sendReport(0x00, usbReport);
