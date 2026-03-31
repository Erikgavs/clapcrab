use std::time;

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
    // None because at the beggining there are no claps
    let mut last_clap: Option<std::time::Instant> = None;

    let stream = device.build_input_stream(
        &config.into(),
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let hight = data.iter().map(|s| s.abs()).fold(0.0f32, f32::max);

            // A loud sound just started — mark it and wait for the next block
            if hight > 0.1 && !hight_sound {
                hight_sound = true;
            // Volume dropped quickly after being loud — short sound = clap
            } else if hight < 0.01 && hight_sound {
                hight_sound = false;

                match last_clap {
                    // logic that involves 2 claps
                    // spacing between claps involved here
                    Some(time) => {
                        let elapsed = time.elapsed();
                        if elapsed.as_millis() > 100 && elapsed.as_millis() < 700 {
                            println!("Double clap");
                            last_clap = None;
                        } else {
                            last_clap = Some(std::time::Instant::now());
                        }
                    }
                    None => {
                        last_clap = Some(std::time::Instant::now());
                    }
                }

            // Still quiet, nothing happening — reset just in case
            } else if hight < 0.01 {
                hight_sound = false;
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
