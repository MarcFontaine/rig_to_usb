use std::env;
use gstreamer as gst;
use gstreamer::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    // Initialize GStreamer before calling any other functions
    gst::init()?;

    // 1. Define your pipeline string exactly as you would in gst-launch-1.0
    //  let pipeline_str = "videotestsrc ! videoconvert ! autovideosink audiotestsrc ! audioconvert ! audioresample ! autoaudiosink";

    let pipeline_str = &args[1];
    // 2. Parse the string into a GStreamer Element (downcasted to a Pipeline)
    let pipeline = gst::parse::launch(pipeline_str)?
        .dynamic_cast::<gst::Pipeline>()
        .map_err(|_| "Failed to cast parsed element to a Pipeline")?;

    // 3. Start playing the pipeline
    pipeline.set_state(gst::State::Playing)?;

    // 4. Retrieve the pipeline bus to monitor events and errors
    let bus = pipeline.bus().expect("Pipeline must have a bus");

    // 5. Block the thread and wait for an Error or End-Of-Stream (EOS) message
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

    // 6. Stop the pipeline before exiting
    pipeline.set_state(gst::State::Null)?;
    Ok(())
}
