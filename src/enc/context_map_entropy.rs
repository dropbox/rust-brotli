use super::super::alloc;
use super::super::alloc::SliceWrapper;
use super::interface;
use super::input_pair::{InputPair, InputReference};
use super::histogram::ContextType;
use super::constants::{kSigned3BitContextLookup, kUTF8ContextLookup};
pub struct ContextMapEntropy<'a, AllocU32:alloc::Allocator<u32>> {
    input: InputPair<'a>,
    context_map: interface::PredictionModeContextMap<InputReference<'a>>,
    block_type: u8,
    local_byte_offset: usize,
    nop: AllocU32::AllocatedMemory,
}
impl<'a, AllocU32:alloc::Allocator<u32>> ContextMapEntropy<'a, AllocU32> {
   pub fn new(m32: &mut AllocU32, input: InputPair<'a>, prediction_mode: interface::PredictionModeContextMap<InputReference<'a>>) -> Self {
      ContextMapEntropy::<AllocU32>{
         input: input,
         context_map: prediction_mode,
         block_type: 0,
         local_byte_offset: 0,
         nop:  AllocU32::AllocatedMemory::default(),
      }
   }
   pub fn track_cdf_speed(&mut self,
                      data: &[u8],
                      mut prev_byte: u8, mut prev_prev_byte: u8,
                      block_type: u8) {
       /*
       scratch.bucket_populations.slice_mut().clone_from_slice(self.entropy_tally.bucket_populations.slice());
       scratch.bucket_populations.slice_mut()[65535] += 1; // to demonstrate that we have
       scratch.bucket_populations.slice_mut()[65535] -= 1; // to demonstrate that we have write capability
       let mut stray_count = 0 as find_stride::floatY;
       for val in data.iter() {
           let huffman_table_index = compute_huffman_table_index_for_context_map(prev_byte, prev_prev_byte, self.context_map, block_type);
           let loc = &mut scratch.bucket_populations.slice_mut()[huffman_table_index * 256 + *val as usize];
           //let mut stray = false;
           if *loc == 0 {
               stray_count += 1.0;
               //stray = true;
           } else {
               *loc -= 1;
           }
           //println!("{} {:02x}{:02x} => {:02x} (bt: {}, ind: {}, cnt: {})", if stray {"S"} else {"L"}, prev_byte, prev_prev_byte, *val, block_type, huffman_table_index, *loc);
           prev_prev_byte = prev_byte;
           prev_byte = *val;
       }
       if self.entropy_tally.cached_bit_entropy == 0.0 as find_stride::floatY {
           self.entropy_tally.cached_bit_entropy = find_stride::HuffmanCost(self.entropy_tally.bucket_populations.slice());
       }
       debug_assert_eq!(find_stride::HuffmanCost(self.entropy_tally.bucket_populations.slice()),
                        self.entropy_tally.cached_bit_entropy);

       scratch.cached_bit_entropy = find_stride::HuffmanCost(scratch.bucket_populations.slice());
       self.entropy_tally.cached_bit_entropy - scratch.cached_bit_entropy + stray_count * 8.0
*/
   }
   pub fn free(&mut self, m32: &mut AllocU32) {
       
   }
}
fn Context(p1: u8, p2: u8, mode: ContextType) -> u8 {
  match mode {
    ContextType::CONTEXT_LSB6 => {
      return (p1 as (i32) & 0x3fi32) as (u8);
    }
    ContextType::CONTEXT_MSB6 => {
      return (p1 as (i32) >> 2i32) as (u8);
    }
    ContextType::CONTEXT_UTF8 => {
      return (kUTF8ContextLookup[p1 as (usize)] as (i32) |
              kUTF8ContextLookup[(p2 as (i32) + 256i32) as (usize)] as (i32)) as (u8);
    }
    ContextType::CONTEXT_SIGNED => {
      return ((kSigned3BitContextLookup[p1 as (usize)] as (i32) << 3i32) +
              kSigned3BitContextLookup[p2 as (usize)] as (i32)) as (u8);
    }
  }
  //  0i32 as (u8)
}

fn compute_huffman_table_index_for_context_map<SliceType: alloc::SliceWrapper<u8> > (
    prev_byte: u8,
    prev_prev_byte: u8,
    context_map: interface::PredictionModeContextMap<SliceType>,
    block_type: u8,
) -> usize {
    let prior = Context(prev_byte, prev_prev_byte, context_map.literal_prediction_mode.to_context_enum().unwrap());
    assert!(prior < 64);
    let context_map_index = ((block_type as usize)<< 6) | prior as usize;
    if context_map_index < context_map.literal_context_map.slice().len() {
        context_map.literal_context_map.slice()[context_map_index] as usize
    } else {
        prior as usize
    }
}

impl<'a, 'b, AllocU32:alloc::Allocator<u32>> interface::CommandProcessor<'b> for ContextMapEntropy<'a, AllocU32> {
    fn push<Cb: FnMut(&[interface::Command<InputReference>])>(&mut self,
                                                             val: interface::Command<InputReference<'b>>,
                                                             callback: &mut Cb) {
        match val {
           interface::Command::BlockSwitchCommand(_) |
           interface::Command::BlockSwitchDistance(_) |
           interface::Command::PredictionMode(_) => {}
           interface::Command::Copy(ref copy) => {
             self.local_byte_offset += copy.num_bytes as usize;
           },
           interface::Command::Dict(ref dict) => {
             self.local_byte_offset += dict.final_size as usize;
           },
           interface::Command::BlockSwitchLiteral(block_type) => self.block_type = block_type.block_type(),
           interface::Command::Literal(ref lit) => {
               let mut priors= [0u8, 0u8];
               if self.local_byte_offset > 1 {
                   priors[0] = self.input[self.local_byte_offset - 2];
                   priors[1] = self.input[self.local_byte_offset - 1];
               }
               for literal in lit.data.slice().iter() {                   
                   let huffman_table_index = compute_huffman_table_index_for_context_map(priors[1], priors[0], self.context_map, self.block_type);

                   // FIXME..... self.entropy_tally.bucket_populations.slice_mut()[((huffman_table_index as usize) << 8) | *literal as usize] += 1;
                    //println!("I {:02x}{:02x} => {:02x} (bt: {}, ind: {} cnt: {})", priors[1], priors[0], *literal, self.block_type, huffman_table_index, self.entropy_tally.bucket_populations.slice_mut()[((huffman_table_index as usize) << 8) | *literal as usize]);
                   priors[0] = priors[1];
                   priors[1] = *literal;
               }
               self.local_byte_offset += lit.data.slice().len();
           }
        }
        let cbval = [val];
        callback(&cbval[..]);
    }
}
