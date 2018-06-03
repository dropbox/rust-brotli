
use super::super::alloc::{SliceWrapper, SliceWrapperMut, Allocator};
use super::util::floatX;
use super::vectorization::Mem256f;
use super::interface;
use super::interface::CommandProcessor;
use super::ir_interpret::{IRInterpreter, push_base};
use super::pdf::{PDF, similar, pdf_eval, merged_pdf_cost};
use interface::{PredictionModeContextMap, MAX_ADV_LITERAL_CONTEXT_MAP_SIZE, MAX_LITERAL_CONTEXT_MAP_SIZE};
use core::marker::PhantomData;
use enc::backward_references::BrotliEncoderParams;
use super::input_pair::{InputPair, InputReference, InputReferenceMut};

#[derive(Copy,Clone,Default,PartialOrd,PartialEq, Eq, Ord,Debug)]
struct Similarity {
    cost_times_256: u32,
    source: usize,
    buddy: usize,
}

impl Similarity {
    pub fn bad() -> Self {
        Similarity {
            cost_times_256:0xffffffff,
            source:0,
            buddy:65536 * 32768,
        }
    }
    pub fn is_bad(&self) -> bool {
        self.buddy == 65536 * 32768
    }
    pub fn new(to_merge: &PDF,
               to_merge_in:&PDF,
               to_merge_index:usize,
               to_merge_in_index: usize) -> Self {
        let entropy = merged_pdf_cost(to_merge, to_merge_in) - pdf_eval(to_merge, to_merge) - pdf_eval(to_merge_in, to_merge_in) ;
        let ret = Similarity {
            cost_times_256: (entropy * 256.0) as u32,
            source: to_merge_index,
            buddy: to_merge_in_index,
        };
        eprintln!("ENTROPY: {:?} vs {:?}", entropy, ret.cost_times_256);
        ret
    }
}

pub struct Processor<'a,
                 AllocU16:Allocator<u16>,
                 AllocU32:Allocator<u32>,
                 AllocF:Allocator<floatX>,
                 AllocFV:Allocator<Mem256f>,
                 AllocPDF:Allocator<PDF>> {
    p0: PhantomData<AllocU16>,
    p1: PhantomData<AllocU32>,
    p2: PhantomData<AllocF>,
    p3: PhantomData<AllocFV>,
    p4: PhantomData<AllocPDF>,
    pub context_map: PredictionModeContextMap<InputReferenceMut<'a>>,
    input: InputPair<'a>,
    local_byte_offset: usize,
    block_type: u8,
    cur_stride: u8,
    context_histograms: AllocPDF::AllocatedMemory,
    htable_histograms: [AllocPDF::AllocatedMemory;2],
}

fn transitive_closure(data:&mut [usize; 256 * 16]) {
    for index in 0..256 * 16 {
        let mut sub_index = index;
        for round in 0..256 {
            let sub_sub_index = data[sub_index];
            data[index] = sub_sub_index;
            sub_index = sub_sub_index;
            if data[sub_index] == sub_index {
                break;
            }
        }
    }
}
fn relabel_0_256(data:&mut [usize; 256 * 16]) -> bool {
    eprintln!("Transtiive closure on {:?}", &data[..]);
    transitive_closure(data);
    eprintln!("Transtiive closed on {:?}", &data[..]);
    let mut count = 0;
    {
        let mut seen = [false;256 * 16];
        for item in data.iter() {
            if !seen[*item] {
                eprintln!("Potential Remap {} > {}", *item, count);
                count += 1;
                seen[*item] = true;
            }
        }
        if count > 256 {
            return false;
        }
    }
    count = 0;
    let mut seen = [false;256 * 16];
    let mut index = [0u8;256 * 16];
    for item in data.iter_mut() {
        if !seen[*item] {
            eprintln!("Remap {} > {}", *item, count);
            index[*item] = count as u8;
            count += 1;
            seen[*item] = true;
        }
        *item = usize::from(index[*item]);
    }
    true
}

impl<'a,
     AllocU16:Allocator<u16>,
     AllocU32:Allocator<u32>,
     AllocF:Allocator<floatX>,
     AllocFV:Allocator<Mem256f>,
     AllocPDF:Allocator<PDF>> Processor<'a,
                                        AllocU16,
                                        AllocU32,
                                        AllocF,
                                        AllocFV,
                                        AllocPDF> {
    pub fn new(m16: &mut AllocU16,
               m32: &mut AllocU32,
               mf: &mut AllocF,
               mfv: &mut AllocFV,
               mpdf: &mut AllocPDF,
               ip: InputPair<'a>,
               cm: PredictionModeContextMap<InputReferenceMut<'a>>,
               params:&BrotliEncoderParams) -> Self {
        Self{
            p0: PhantomData::<AllocU16>::default(),
            p1: PhantomData::<AllocU32>::default(),
            p2: PhantomData::<AllocF>::default(),
            p3: PhantomData::<AllocFV>::default(),
            p4: PhantomData::<AllocPDF>::default(),
            context_map:cm,
            input: ip,
            local_byte_offset:0,
            block_type:0,
            cur_stride:0,
            context_histograms: mpdf.alloc_cell(MAX_ADV_LITERAL_CONTEXT_MAP_SIZE),
            htable_histograms: [mpdf.alloc_cell(256 * 16), mpdf.alloc_cell(256)],
        }
    }
    
    pub fn merge(&mut self) {
        let mut merge_map = [0usize;256 * 16];
        for i in 0..256 {
            for j in 0..16 {
                let linear_input = i + j* 256;
                if !self.htable_histograms[0].slice()[linear_input].has_samples() {
                    merge_map[linear_input] = 0;
                } else {
                    merge_map[linear_input] = i + j * 256;
                }
            }
        }
        for round in 0..256 {
            let mut consumed_map = [false; 256 * 16];
            let mut similarity_map = [Similarity::bad(); 256 * 16];
            for i in 0..256 {
                for j in 0..16 {
                    let linear_input = i + j* 256;
                    if !self.htable_histograms[0].slice()[linear_input].has_samples() {
                        consumed_map[linear_input] = true;
                    }
                    if consumed_map[linear_input] {
                        similarity_map[linear_input] = Similarity::bad();
                    } else {
                        let mut best_score = Similarity::bad();
                        for k in 0..256 {
                            for ks in 0..16 {
                                let linear_candidate = k + ks * 256;
                                if linear_candidate > linear_input && self.htable_histograms[0].slice()[linear_candidate].has_samples() && !consumed_map[linear_candidate] {
                                    let candidate_similarity = Similarity::new(&self.htable_histograms[0].slice()[linear_input],
                                                                               &self.htable_histograms[0].slice()[linear_candidate],
                                                                               linear_input,
                                                                               linear_candidate);
                                    if candidate_similarity < best_score {
                                        best_score = candidate_similarity;
                                    }
                                }
                            }
                        }
                        if !best_score.is_bad() {
                            consumed_map[best_score.buddy] = true;
                            similarity_map[linear_input] = best_score;
                        }
                    }
                }
            }
            similarity_map.sort();
            let mut stop = 0;
            for i in 0..similarity_map.len() {
                if similarity_map[i].is_bad() {
                    break;
                }
                stop = i;
            }
            for i in (stop/2)..stop {
                eprintln!("NOTMerg {:?} <-> {:?}", similarity_map[i], merge_map[similarity_map[i].source]);
            }
            transitive_closure(&mut merge_map);
            for i in 0..(stop / 2) {
                let actual_source = merge_map[similarity_map[i].source];
                eprintln!("Merging {:?} <-> {:?} -> {}", similarity_map[i], similarity_map[i].source, actual_source);
                let buddy_pdf = self.htable_histograms[0].slice()[similarity_map[i].buddy];
                self.htable_histograms[0].slice_mut()[actual_source].add_samples(&buddy_pdf);
                self.htable_histograms[0].slice_mut()[similarity_map[i].buddy] = PDF::default();
                merge_map[similarity_map[i].buddy] = actual_source;
            }
            if stop < 256 * 2 && round >= 4 {
                if relabel_0_256(&mut merge_map) {
                    break;
                }
            }
        }
        //eprintln!("Merging map {:?} and {:?}\n", &merge_map[0][..],&merge_map[1][..]);
        for (index, item) in self.context_map.literal_context_map.slice_mut().iter_mut().enumerate() {
            if index >= MAX_LITERAL_CONTEXT_MAP_SIZE {
                let upper_nibble = (index - MAX_LITERAL_CONTEXT_MAP_SIZE) / MAX_LITERAL_CONTEXT_MAP_SIZE;
                let to_replace = merge_map[usize::from(*item)+ upper_nibble * 256];
                assert!(to_replace < 256);
                *item = to_replace as u8;
            }
        }
    }
}

impl<'a,
     AllocU16:Allocator<u16>,
     AllocU32:Allocator<u32>,
     AllocF:Allocator<floatX>,
     AllocFV:Allocator<Mem256f>,
     AllocPDF:Allocator<PDF>> IRInterpreter for Processor <'a, AllocU16, AllocU32, AllocF, AllocFV, AllocPDF> {
    #[inline]
    fn inc_local_byte_offset(&mut self, inc: usize) {
        self.local_byte_offset += inc;
    }
    #[inline]
    fn local_byte_offset(&self) -> usize {
        self.local_byte_offset
    }
    #[inline]
    fn update_block_type(&mut self, new_type: u8, stride: u8) {
        self.block_type = new_type;
        self.cur_stride = stride;
    }
    #[inline]
    fn block_type(&self) -> u8 {
        self.block_type
    }
    #[inline]
    fn literal_data_at_offset(&self, index:usize) -> u8 {
        self.input[index]
    }
    #[inline]
    fn literal_context_map(&self) -> &[u8] {
        self.context_map.literal_context_map.slice()
    }
    #[inline]
    fn prediction_mode(&self) -> ::interface::LiteralPredictionModeNibble {
        self.context_map.literal_prediction_mode()
    }
    #[inline]
    fn update_cost(&mut self, stride_prior: [u8;8], stride_prior_offset: usize, selected_bits: u8, cm_prior: usize, cm_prior_lower: usize, literal: u8) {
        //let stride = self.cur_stride as usize;
        //self.update_cost_base(stride_prior, stride_prior_offset, selected_bits, cm_prior, literal)
        let high_bit = literal >> 4;
        let low_bit = literal & 0xf;
        let context_map_low_index = usize::from(selected_bits) + 64 * usize::from(self.block_type);
        let context_map_high_index = (usize::from(selected_bits) + 64 * usize::from(self.block_type)) + (64 * 256 * (high_bit as usize + 1));
        self.context_histograms.slice_mut()[context_map_low_index].add_sample(low_bit);
        self.context_histograms.slice_mut()[context_map_high_index].add_sample(high_bit);
        let high_indirect_index = usize::from(self.context_map.literal_context_map.slice()[context_map_high_index]);
        assert_eq!(high_indirect_index, cm_prior);
        let low_indirect_index = usize::from(self.context_map.literal_context_map.slice()[context_map_low_index]);
        assert_eq!(low_indirect_index, cm_prior_lower);
        assert_eq!(low_indirect_index, high_indirect_index);
        self.htable_histograms[0].slice_mut()[low_indirect_index + 256 * high_bit as usize].add_sample(low_bit);
        self.htable_histograms[1].slice_mut()[high_indirect_index].add_sample(high_bit);
    }

    
}



impl<'a, 'b,
     AllocU16:Allocator<u16>,
     AllocU32:Allocator<u32>,
     AllocF:Allocator<floatX>,
     AllocFV:Allocator<Mem256f>,
     AllocPDF:Allocator<PDF>> CommandProcessor<'b> for Processor <'a, AllocU16, AllocU32, AllocF, AllocFV, AllocPDF> {
    fn push<Cb: FnMut(&[interface::Command<InputReference>])>(&mut self,
                                                              val: interface::Command<InputReference<'b>>,
                                                              callback: &mut Cb) {
        push_base(self, val, callback)
    }

}
