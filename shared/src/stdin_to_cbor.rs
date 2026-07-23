use rig_to_usb_logic::cmd::Cmd;

fn main() {
    let mut input = String::new();

    match std::io::stdin().read_line(&mut input) {
       Ok(_size) => {
	   match serde_json::from_str::<Cmd>(input.trim()) {
	       Ok(cmd) => {
		   eprintln!("Parse OK: {:?}", cmd);
		   match minicbor::to_vec(&cmd) {
		      Ok(encoded_bytes) => {
			  eprintln!("CBOR len: {} Bytes", encoded_bytes.len());

			  use std::io::{self, Write};
			  let _ = io::stdout().write_all(&encoded_bytes);
			  let _ = io::stdout().flush();
                          std::process::exit(0);
		      }
		      Err(err) => {
			  eprintln!("CBOR Error: {}", err);
		      }
		  }
	       }
	       Err(err) => {
		   eprintln!("Parse Error: {}", err);
                   std::process::exit(1);
	       }
	   }
       }
       Err(err) => {
	   eprintln!("Error reading from stdin: {}", err);
	   std::process::exit(1);
       }
   }
}

