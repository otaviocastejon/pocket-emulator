//! cpal output queue.

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{OutputCallbackInfo, SampleFormat, StreamError};

const MAX_QUEUE_SAMPLES: usize = 48000 * 2 * 4;

pub struct AudioOutput {
    queue: Arc<Mutex<VecDeque<f32>>>,
    _stream: cpal::Stream,
    /// Actual device sample rate (use for APU resampling period).
    pub sample_rate: u32,
}

impl AudioOutput {
    pub fn try_default() -> Option<Self> {
        let host = cpal::default_host();
        let device = host.default_output_device()?;
        let supported = device.default_output_config().ok()?;
        let sample_format = supported.sample_format();
        let mut cfg = supported.config();
        cfg.channels = 2;
        let sample_rate = cfg.sample_rate.0;

        let queue = Arc::new(Mutex::new(VecDeque::with_capacity(8192)));

        let stream = match sample_format {
            SampleFormat::F32 => {
                let q = Arc::clone(&queue);
                device
                    .build_output_stream(
                        &cfg,
                        move |data: &mut [f32], _: &OutputCallbackInfo| fill_output(&q, data),
                        |e: StreamError| log::warn!("audio stream error: {e}"),
                        None,
                    )
                    .ok()?
            }
            SampleFormat::I16 => {
                let q = Arc::clone(&queue);
                device
                    .build_output_stream(
                        &cfg,
                        move |data: &mut [i16], _: &OutputCallbackInfo| {
                            let mut buf = q.lock().unwrap();
                            for chunk in data.chunks_exact_mut(2) {
                                let l = buf.pop_front().unwrap_or(0.0).clamp(-1.0, 1.0);
                                let r = buf.pop_front().unwrap_or(0.0).clamp(-1.0, 1.0);
                                chunk[0] = (l * 32767.0) as i16;
                                chunk[1] = (r * 32767.0) as i16;
                            }
                        },
                        |e: StreamError| log::warn!("audio stream error: {e}"),
                        None,
                    )
                    .ok()?
            }
            SampleFormat::U16 => {
                let q = Arc::clone(&queue);
                device
                    .build_output_stream(
                        &cfg,
                        move |data: &mut [u16], _: &OutputCallbackInfo| {
                            let mut buf = q.lock().unwrap();
                            for chunk in data.chunks_exact_mut(2) {
                                let l = buf.pop_front().unwrap_or(0.0).clamp(-1.0, 1.0);
                                let r = buf.pop_front().unwrap_or(0.0).clamp(-1.0, 1.0);
                                chunk[0] = (((l * 0.5 + 0.5).clamp(0.0, 1.0)) * 65535.0) as u16;
                                chunk[1] = (((r * 0.5 + 0.5).clamp(0.0, 1.0)) * 65535.0) as u16;
                            }
                        },
                        |e: StreamError| log::warn!("audio stream error: {e}"),
                        None,
                    )
                    .ok()?
            }
            _ => return None,
        };

        stream.play().ok()?;
        Some(AudioOutput {
            queue,
            _stream: stream,
            sample_rate,
        })
    }

    pub fn enqueue_interleaved(&self, samples: Vec<f32>) {
        let mut q = self.queue.lock().unwrap();
        let incoming = samples.len();
        // Drop oldest audio instead of clearing everything — avoids loud volume pumping.
        while q.len() + incoming > MAX_QUEUE_SAMPLES && !q.is_empty() {
            q.pop_front();
        }
        for s in samples {
            q.push_back(s);
        }
    }
}

fn fill_output(queue: &Arc<Mutex<VecDeque<f32>>>, data: &mut [f32]) {
    let mut q = queue.lock().unwrap();
    for chunk in data.chunks_exact_mut(2) {
        chunk[0] = q.pop_front().unwrap_or(0.0);
        chunk[1] = q.pop_front().unwrap_or(0.0);
    }
}
