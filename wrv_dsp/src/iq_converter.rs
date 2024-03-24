pub struct Converter {
    avg: f32,
    hbc: f32,
    len: usize,
    fir_index: usize,
    delay_index: usize,
    fir_kernel: Vec<f32>,
    fir_queue: Vec<f32>,
    delay_line: Vec<f32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_delay_interleaved() {
        let mut c = Converter::new();
        let mut samples : Vec<f32> = vec![1.0; 50];
        // delay  half of filter length
        c.delay_interleaved(&mut samples[1..]);
        let first_index_of_delayed_q = samples.iter().skip(1).step_by(2).position(|&x| x == 1.0);

        assert_eq!(Some(c.len / 2), first_index_of_delayed_q)
    }
}

impl Converter {
    pub fn new() -> Self {
        let len = HB_KERNEL.len();
        let fir_kernel : Vec<f32> = HB_KERNEL.iter().step_by(2).copied().collect();
        let converter_length = len / 2 + 1;
        Self {
            avg: 0.0,
            hbc: HB_KERNEL[len / 2],
            len: converter_length,
            fir_index: 0,
            delay_index: 0,
            fir_kernel,
            fir_queue: vec![0.0; converter_length * SIZE_FACTOR],
            delay_line: vec![0.0; converter_length / 2],
        }
    }

    pub fn reset(self: &mut Self) {
        self.avg = 0.0;
        self.fir_index = 0;
        self.delay_index = 0;
        self.fir_queue = vec![0.0; self.fir_queue.len()];
        self.delay_line = vec![0.0; self.delay_line.len()];
    }

    pub fn process(self: &mut Self, samples: &mut [f32]) {
        self.remove_dc(samples);
        unsafe { self.translate_fs_4(samples); }
    }
    
    // #[cfg(target_arch = "wasm32")]
    // #[target_feature(enable = "simd128")]
    unsafe fn translate_fs_4(self: &mut Self, samples: &mut [f32]) {
        let hbc = self.hbc;

        // use std::arch::wasm32::{f32x4, _mm_loadu_ps, _mm_mul_ps, _mm_storeu_ps, _mm_set_ps};

        // let mut buf = samples.as_mut_ptr();
        // let rot = unsafe { _mm_set_ps(hbc, 1.0, -hbc, -1.0) };

        // for _ in 0..len / 4 {
        //     let vec = unsafe { _mm_loadu_ps(buf) };
        //     let result = unsafe { _mm_mul_ps(vec, rot) };
        //     unsafe { _mm_storeu_ps(buf, result) };
        //     buf = buf.add(4);
        // }
        
        for i in 0..samples.len() / 4 {
            let j = i << 2;
            samples[j] = -samples[j];
            samples[j + 1] = -samples[j + 1] * hbc;
            //samples[j + 2] = samples[j + 2]; // Uncomment if needed
            samples[j + 3] = samples[j + 3] * hbc;
        }
        
        self.fir_interleaved_24(samples);
        self.delay_interleaved(&mut samples[1..]);
    }

    fn remove_dc(self: &mut Self, samples: &mut [f32]) {
        let mut avg = self.avg;

        for sample in samples.iter_mut() {
            *sample -= avg;
            avg += SCALE * *sample;
        }

        self.avg = avg;
    }

    fn fir_interleaved_24(self: &mut Self, samples: &mut [f32]) {

        let mut acc;
        let mut fir_index = self.fir_index;
        let fir_len = self.len;
        
        for sample in samples.iter_mut().step_by(2) {
            let queue = &mut self.fir_queue[fir_index..fir_index + 24];
            let fir_kernel = &self.fir_kernel;

            queue[0] = *sample;

            acc = fir_kernel[0] * (queue[0] + queue[24 - 1])
                + fir_kernel[1] * (queue[1] + queue[24 - 2])
                + fir_kernel[2] * (queue[2] + queue[24 - 3])
                + fir_kernel[3] * (queue[3] + queue[24 - 4])
                + fir_kernel[4] * (queue[4] + queue[24 - 5])
                + fir_kernel[5] * (queue[5] + queue[24 - 6])
                + fir_kernel[6] * (queue[6] + queue[24 - 7])
                + fir_kernel[7] * (queue[7] + queue[24 - 8])
                + fir_kernel[8] * (queue[8] + queue[24 - 9])
                + fir_kernel[9] * (queue[9] + queue[24 - 10])
                + fir_kernel[10] * (queue[10] + queue[24 - 11])
                + fir_kernel[11] * (queue[11] + queue[24 - 12]);

            *sample = acc;

            if fir_index == 0 {
                fir_index = fir_len * (SIZE_FACTOR - 1);
                let len_minus_one = fir_len - 1;
                self.fir_queue.copy_within(..len_minus_one, fir_index + 1);
            }
            fir_index -= 1;
        }

        self.fir_index = fir_index;
    }

    fn delay_interleaved(self: &mut Self, samples: &mut [f32]) {
        let half_len = self.len >> 1;
        let mut index = self.delay_index;
        
        for sample in samples.iter_mut().step_by(2) {
            let res = self.delay_line[index];
            self.delay_line[index] = *sample;
            *sample = res;

            index = (index + 1) % half_len;
        }

        self.delay_index = index;
    }
}

const SIZE_FACTOR : usize = 32;
const DEFAULT_ALIGNMENT : usize = 16;
const HPF_COEFF : f32 = 0.01;
const SCALE : f32 = 0.01;

const HB_KERNEL : [f32; 47] = [
    -0.000998606272947510,
	 0.000000000000000000,
	 0.001695637278417295,
	 0.000000000000000000,
	-0.003054430179754289,
	 0.000000000000000000,
	 0.005055504379767936,
	 0.000000000000000000,
	-0.007901319195893647,
	 0.000000000000000000,
	 0.011873357051047719,
	 0.000000000000000000,
	-0.017411159379930066,
	 0.000000000000000000,
	 0.025304817427568772,
	 0.000000000000000000,
	-0.037225225204559217,
	 0.000000000000000000,
	 0.057533286997004301,
	 0.000000000000000000,
	-0.102327462004259350,
	 0.000000000000000000,
	 0.317034472508947400,
	 0.500000000000000000,
	 0.317034472508947400,
	 0.000000000000000000,
	-0.102327462004259350,
	 0.000000000000000000,
	 0.057533286997004301,
	 0.000000000000000000,
	-0.037225225204559217,
	 0.000000000000000000,
	 0.025304817427568772,
	 0.000000000000000000,
	-0.017411159379930066,
	 0.000000000000000000,
	 0.011873357051047719,
	 0.000000000000000000,
	-0.007901319195893647,
	 0.000000000000000000,
	 0.005055504379767936,
	 0.000000000000000000,
	-0.003054430179754289,
	 0.000000000000000000,
	 0.001695637278417295,
	 0.000000000000000000,
	-0.000998606272947510
];

