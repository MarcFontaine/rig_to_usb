// 1. Initialisierung (nur einmalig beim Starten der App)
const bufferPtr = wasmInstance.exports.get_buffer_ptr();
const maxCapacity = wasmInstance.exports.get_buffer_capacity();

// Diese Funktion kannst du nun extrem oft (z.B. in jedem Request/Frame) aufrufen:
function fastProcess(inputData) {
    const len = inputData.length;
    
    if (len > maxCapacity) {
        throw new Error("Daten sind zu groß für den vordefinierten WASM-Buffer!");
    }

    // ACHTUNG: Die View muss innerhalb der Funktion erstellt werden, 
    // falls der WASM-Speicher insgesamt wächst. Das erzeugt KEIN Memory Leak.
    const wasmBufferView = new Uint8Array(
        wasmInstance.exports.memory.buffer, 
        bufferPtr, 
        len
    );

    // 2. Daten direkt in den festen WASM-Speicher kopieren
    wasmBufferView.set(inputData);

    // 3. WASM-Logik ausführen
    const resultStatus = wasmInstance.exports.process_bytes(len);
    if (resultStatus !== 0) return;

    // 4. Ergebnis direkt aus der View lesen (ohne Freigabe-Schritt!)
    // wasmBufferView enthält jetzt die modifizierten Daten.
    
    // Optional: Falls du die Daten außerhalb brauchst, kopiere sie lokal heraus:
    // const output = new Uint8Array(wasmBufferView);
}

// Bereite das Import-Objekt vor. 
// Wir brauchen eine Referenz auf die spätere WASM-Instanz, um auf deren Speicher zuzugreifen.
let wasmInstance;

const importObject = {
    env: {
        host_panic: function(ptr, len) {
            // Lese den Text direkt aus dem WASM-Speicherbereich
            const memoryBuffer = wasmInstance.exports.memory.buffer;
            const utf8Decoder = new TextDecoder("utf-8");
            const bytes = new Uint8Array(memoryBuffer, ptr, len);
            const errorMessage = utf8Decoder.decode(bytes);
            
            // Gib den Fehler gut sichtbar in der Browser-Konsole aus
            console.error("🚨 CRITICAL WASM PANIC:", errorMessage);
        }
    }
};

// WASM-Datei laden und instanziieren (Pfade anpassen)
WebAssembly.instantiateStreaming(fetch("wasm_nostd_example.wasm"), importObject)
    .then(result => {
        wasmInstance = result.instance;
        
        console.log("WASM geladen. Trigger jetzt den Panic...");
        // Rufe die Testfunktion auf, die den Absturz verursacht
        wasmInstance.exports.trigger_panic();
    })
    .catch(err => {
        // Fängt den nachfolgenden "RuntimeError: unreachable" ab,
        // damit die Seite nicht komplett blockiert.
        console.log("WASM wurde nach dem Panic erfolgreich gestoppt.");
    });
