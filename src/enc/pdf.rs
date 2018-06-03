use core;
use super::util::{floatX, FastLog2};
#[derive(Copy, Clone, Default, Debug)]
pub struct PDF([i32;16]);

impl PDF {
    pub fn add_sample(&mut self, nibble: u8) {
        self.0[nibble as usize] += 1;
    }
    pub fn add_samples(&mut self, pdf: &PDF) {
        for (dst, src) in self.0.iter_mut().zip(pdf.0.iter()) {
            *dst += *src;
        }
    }
    pub fn has_samples(&self) -> bool {
        for item in self.0.iter() {
            if *item != 0 {
                return true;
            }
        }
        false
    }
}

pub fn merged_pdf_cost(pdfA:&PDF, pdfB:&PDF) -> floatX {
    let mut pdf = pdfA.clone();
    pdf.add_samples(&pdfB);
    pdf_eval(&pdf, &pdf)
}

pub fn pdf_eval(nibble_string: &PDF, pdf: &PDF) -> floatX{
    let mut total = 1;
    for prob in pdf.0.iter() {
        total += prob;
    }
    if total != 1 {
        total -= 1;
    }
    let divisor = FastLog2(total as u32 as u64);
    let mut cost: floatX = 0.0;
    for (count, prob) in nibble_string.0.iter().zip(pdf.0.iter()) {
        cost += *count as floatX *(divisor - FastLog2(*prob as u32 as u64))
    }
    cost
}

#[inline]
pub fn similar(a: &PDF, b: &PDF) -> bool {
    let cost0 = pdf_eval(b,b);
    let cost1 = pdf_eval(b,a);
    cost1 - cost0 < 44.0 // less than a nibble difference
    /*
    let mut a_total = 1;
    let mut b_total = 1;
    for (ai, bi) in a.0.iter().zip(b.0.iter()) {
        a_total += *ai;
        b_total += *bi;
    }
    let m_total = core::cmp::max(a_total, b_total);
    let a_scale = m_total * 256 / a_total;
    let b_scale = m_total * 256 / b_total;
    let mut error = 0;
    for (ai, bi) in a.0.iter().zip(b.0.iter()) {
        error += (*ai as i32 * a_scale - *bi as i32 * b_scale).abs();
    }
    error < (m_total <<5)
*/
}
