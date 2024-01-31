use std::sync::Arc;

use rustfft::{num_complex::Complex, Fft, FftPlanner};

struct Dsp {
    fft: Arc<dyn Fft<f32>>
}

impl Dsp {
    pub fn new(length: usize) -> Self {
        let mut planner: FftPlanner<f32> = FftPlanner::new();
        let fft = planner.plan_fft_forward(length);
        Dsp { fft }
    }
    
    pub fn process(&self, buffer: &mut Vec<Complex<f32>>) {
        self.fft.process(buffer);
    }
}