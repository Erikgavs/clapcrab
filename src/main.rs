use anyhow::Context;
use cpal::traits::DeviceTrait;
use cpal::traits::HostTrait;
use cpal::traits::StreamTrait;

fn main() -> anyhow::Result<()> {
    // micro-detection
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .context("Error finding microphone")?;
    let config = device
        .default_input_config()
        .context("Error obtaining the config of the device")?;
    println!("{:?} micro config", config);

    let mut hight_sound = false;

    let stream = device.build_input_stream(
        &config.into(),
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let hight = data.iter().map(|s| s.abs()).fold(0.0f32, f32::max);

            if hight > 0.01 {
                println!("Clap detected {}", hight);
            }
        },
        |err| {
            eprintln!("Error in stream {}", err);
        },
        None,
    )?;

    stream.play()?;
    std::thread::sleep(std::time::Duration::from_secs(10));

    Ok(())
}
