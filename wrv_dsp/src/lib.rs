use std::sync::Arc;
use rustfft::{num_complex::Complex, Fft, FftPlanner};

mod iq_converter;

fn convert_to_complex(buffer: &mut [f32]) -> &mut [Complex<f32>] {
    unsafe {
        let ptr = buffer.as_ptr() as *mut Complex<f32>;
        let len = buffer.len();

        assert!(len % 2 == 0);
        
        std::slice::from_raw_parts_mut(ptr, len / 2)
    }
}

pub struct Dsp {
    fft: Arc<dyn Fft<f32>>
}

impl Dsp {
    pub fn new(length: u32) -> Self {
        let mut planner: FftPlanner<f32> = FftPlanner::new();
        let fft = planner.plan_fft_forward(length as usize);
        Dsp { fft }
    }
    
    pub fn process(&self, buffer: &mut [f32]) {
        self.fft.process(convert_to_complex(buffer));
    }
}

#[cfg(test)]
mod tests {
    use crate::iq_converter;

    #[test]
    fn it_works() {
        let c = iq_converter::Converter::new();
        //assert_eq!(c.fir_kernel.len(), c.len);
    }
}