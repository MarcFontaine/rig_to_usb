use gstreamer as gst;
use gstreamer::prelude::*;
use gstreamer_webrtc;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    gst::init()?;

    let pipeline_str = &args[1];
    let pipeline = gst::parse::launch(pipeline_str)?
        .dynamic_cast::<gst::Pipeline>()
        .map_err(|_| "Failed to cast parsed element to a Pipeline")?;

    pipeline.set_state(gst::State::Playing)?;

    let bus = pipeline.bus().expect("Pipeline must have a bus");

    let webrtc = pipeline
        .by_name("webrtc")
        .expect("Element webrtc not found");
    setup_webrtcsink(&webrtc);

    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => {
                println!("End of stream reached.");
                break;
            }
            MessageView::Error(err) => {
                eprintln!(
                    "Error received from element {:?}: {}",
                    err.src().map(|s| s.name()),
                    err.error()
                );
                eprintln!("Debugging information: {:?}", err.debug());
                break;
            }
            _ => {} // Ignore other messages
        }
    }

    pipeline.set_state(gst::State::Null)?;
    Ok(())
}

fn setup_webrtcsink(webrtcsink: &gst::Element) {
    webrtcsink.connect("consumer-added", false, move |args| {
        // The signal arguments provide: [webrtcsink, peer_id, webrtcbin]
        let peer_id: String = args[1].get().unwrap();
        let webrtcbin: gst::Element = args[2].get().unwrap();

        println!("New consumer added with Peer ID: {}", peer_id);

        // 2. Emit 'create-data-channel' on the underlying webrtcbin
        // Parameters: (channel-label, options-dict)

        let options = gst::Structure::builder("application/data-channel")
            .field("ordered", true)
            .field("max-retransmits", 0)
            .field("max-packet-lifetime", -1)
            .field("protocol", "mein-custom-protokoll")
            .build();

        let channel = webrtcbin.emit_by_name::<gstreamer_webrtc::WebRTCDataChannel>(
            "create-data-channel",
            &[&"remote-qth", &options],
        );
        println!(
            "Successfully created data channel: {}",
            channel.label().as_deref().unwrap_or("unknown")
        );
        channel.connect("on-message-string", false, move |args| {
            let msg: String = args[1].get().unwrap();
            println!("Received message from peer {}: {}", peer_id, msg);
            None
        });
        None
    });
}
