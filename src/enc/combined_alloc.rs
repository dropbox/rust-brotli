pub use alloc::Allocator;
use super::Command;
use super::util::floatX;
use super::v8;
use super::s16;
use super::PDF;
use super::StaticCommand;
use super::HistogramLiteral;
use super::HistogramCommand;
use super::HistogramDistance;
use super::HistogramPair;
use super::ContextType;
use super::HuffmanTree;
use super::ZopfliNode;

/*
struct CombiningAllocator<T1, T2, AllocT1:Allocator<T1>, AllocT2:Allocator<T2>>(AllocT1, AllocT2);

impl <T1, T2, AllocT1:Allocator<T1>, AllocT2:Allocator<T2>> CombiningAllocator<T1, T2, AllocT1, AllocT2> {
  pub fn new(a: AllocT1, b: AllocT2) -> Self {
    CombiningAllocator(a, b)
  }
}

impl <T1, T2, AllocT1:Allocator<T1>, AllocT2:Allocator<T2>> Allocator<T1> for CombiningAllocator<T1, T2, AllocT1, AllocT2> {
  
}

      
impl <T1, T2, AllocT1:Allocator<T1>, AllocT2:Allocator<T2>> Allocator<T2> for CombiningAllocator<T1, T2, AllocT1, AllocT2> {
  
}
*/

//pub trait BrotliAlloc:Allocator<u8> + Allocator<u16> + Allocator<i32> + Allocator<u32> + Allocator<u64> + Allocator<Command> + Allocator<super::util::floatX> + Allocator<v8> + Allocator<s16> + Allocator<PDF> + Allocator<StaticCommand> + Allocator<HistogramLiteral> + Allocator<HistogramCommand> + Allocator<HistogramDistance> + Allocator<HistogramPair> + Allocator<ContextType> + Allocator<HuffmanTree> + Allocator<ZopfliNode>{}
pub struct CombiningAllocator<AllocU8:Allocator<u8>,
                          AllocU16:Allocator<u16>,
                          AllocI32:Allocator<i32>,
                          AllocU32:Allocator<u32>,
                          AllocU64:Allocator<u64>,
                          AllocCommand:Allocator<super::Command>,
                          AllocFloatX:Allocator<super::util::floatX>,
                          AllocV8:Allocator<super::v8>,
                          AllocS16:Allocator<super::s16>,
                          AllocPDF:Allocator<super::PDF>,
                          AllocStaticCommand:Allocator<super::StaticCommand>,
                          AllocHistogramLiteral:Allocator<super::HistogramLiteral>,
                          AllocHistogramCommand:Allocator<super::HistogramCommand>,
                          AllocHistogramDistance:Allocator<super::HistogramDistance>,
                          AllocHistogramPair:Allocator<super::HistogramPair>,
                          AllocContextType:Allocator<super::ContextType>,
                          AllocHuffmanTree:Allocator<super::HuffmanTree>,
                          AllocZopfliNode:Allocator<super::ZopfliNode>,
                          >{
  alloc_u8: AllocU8,
  alloc_u16: AllocU16,
  alloc_i32: AllocI32,
  alloc_u32: AllocU32,
  alloc_u64: AllocU64,
  alloc_c: AllocCommand,
  alloc_f: AllocFloatX,
  alloc_f32x8: AllocV8,
  alloc_i16x16: AllocS16,
  alloc_pdf: AllocPDF,
  alloc_sc: AllocStaticCommand,
  alloc_hl: AllocHistogramLiteral,
  alloc_hc: AllocHistogramCommand,
  alloc_hd: AllocHistogramDistance,
  alloc_hp: AllocHistogramPair,
  alloc_ct: AllocContextType,
  alloc_ht: AllocHuffmanTree,
  alloc_zn: AllocZopfliNode,
}

impl<AllocU8:Allocator<u8>,
     AllocU16:Allocator<u16>,
     AllocI32:Allocator<i32>,
     AllocU32:Allocator<u32>,
     AllocU64:Allocator<u64>,
     AllocCommand:Allocator<super::Command>,
     AllocFloatX:Allocator<::enc::util::floatX>,
     AllocV8:Allocator<super::v8>,
     AllocS16:Allocator<super::s16>,
     AllocPDF:Allocator<super::PDF>,
     AllocStaticCommand:Allocator<super::StaticCommand>,
     AllocHistogramLiteral:Allocator<super::HistogramLiteral>,
     AllocHistogramCommand:Allocator<super::HistogramCommand>,
     AllocHistogramDistance:Allocator<super::HistogramDistance>,
     AllocHistogramPair:Allocator<super::HistogramPair>,
     AllocContextType:Allocator<super::ContextType>,
     AllocHuffmanTree:Allocator<super::HuffmanTree>,
     AllocZopfliNode:Allocator<super::ZopfliNode>,
     > CombiningAllocator<AllocU8,
                          AllocU16,
                          AllocI32,
                          AllocU32,
                          AllocU64,
                          AllocCommand,
                          AllocFloatX,
                          AllocV8,
                          AllocS16,
                          AllocPDF,
                          AllocStaticCommand,
                          AllocHistogramLiteral,
                          AllocHistogramCommand,
                          AllocHistogramDistance,
                          AllocHistogramPair,
                          AllocContextType,
                          AllocHuffmanTree,
                          AllocZopfliNode,
                          > {
  pub fn new(alloc_u8: AllocU8,
         alloc_u16: AllocU16,
         alloc_i32: AllocI32,
         alloc_u32: AllocU32,
         alloc_u64: AllocU64,
         alloc_c: AllocCommand,
         alloc_f: AllocFloatX,
         alloc_f32x8: AllocV8,
         alloc_i16x16: AllocS16,
         alloc_pdf: AllocPDF,
         alloc_sc: AllocStaticCommand,
         alloc_hl: AllocHistogramLiteral,
         alloc_hc: AllocHistogramCommand,
         alloc_hd: AllocHistogramDistance,
         alloc_hp: AllocHistogramPair,
         alloc_ct: AllocContextType,
         alloc_ht: AllocHuffmanTree,
         alloc_zn: AllocZopfliNode) -> Self {
    CombiningAllocator{
      alloc_u8:     alloc_u8,
      alloc_u16:    alloc_u16,  
      alloc_i32:   alloc_i32,
      alloc_u32:   alloc_u32, 
      alloc_u64:   alloc_u64, 
      alloc_c:   alloc_c,
      alloc_f:   alloc_f, 
      alloc_f32x8:   alloc_f32x8,
      alloc_i16x16:  alloc_i16x16,
      alloc_pdf:   alloc_pdf,
      alloc_sc:   alloc_sc,
      alloc_hl:   alloc_hl, 
      alloc_hc:   alloc_hc, 
      alloc_hd:   alloc_hd,
      alloc_hp:   alloc_hp, 
      alloc_ct:   alloc_ct, 
      alloc_ht:   alloc_ht,
      alloc_zn:   alloc_zn,
    }
  }
}

  
impl<AllocU8:Allocator<u8>,
     AllocU16:Allocator<u16>,
     AllocI32:Allocator<i32>,
     AllocU32:Allocator<u32>,
     AllocU64:Allocator<u64>,
     AllocCommand:Allocator<super::Command>,
     AllocFloatX:Allocator<super::util::floatX>,
     AllocV8:Allocator<super::v8>,
     AllocS16:Allocator<super::s16>,
     AllocPDF:Allocator<super::PDF>,
     AllocStaticCommand:Allocator<super::StaticCommand>,
     AllocHistogramLiteral:Allocator<super::HistogramLiteral>,
     AllocHistogramCommand:Allocator<super::HistogramCommand>,
     AllocHistogramDistance:Allocator<super::HistogramDistance>,
     AllocHistogramPair:Allocator<super::HistogramPair>,
     AllocContextType:Allocator<super::ContextType>,
     AllocHuffmanTree:Allocator<super::HuffmanTree>,
     AllocZopfliNode:Allocator<super::ZopfliNode>,
     > Allocator<u8> for CombiningAllocator<AllocU8,
                                            AllocU16,
                                            AllocI32,
                                            AllocU32,
                                            AllocU64,
                                            AllocCommand,
                                            AllocFloatX,
                                            AllocV8,
                                            AllocS16,
                                            AllocPDF,
                                            AllocStaticCommand,
                                            AllocHistogramLiteral,
                                            AllocHistogramCommand,
                                            AllocHistogramDistance,
                                            AllocHistogramPair,
                                            AllocContextType,
                                            AllocHuffmanTree,
                                            AllocZopfliNode,
                                            > {
  type AllocatedMemory = AllocU8::AllocatedMemory;
  fn alloc_cell(&mut self, size: usize) -> <Self as Allocator<u8>>::AllocatedMemory {
    self.alloc_u8.alloc_cell(size)
  }
  fn free_cell(&mut self, data: <Self as Allocator<u8>>::AllocatedMemory) {
    self.alloc_u8.free_cell(data)
  }
}


macro_rules! implement_allocator {
  ($bound_name: tt,
   $type_name: tt,
   $local_name: tt) => {
    impl<AllocU8:Allocator<u8>,
         AllocU16:Allocator<u16>,
         AllocI32:Allocator<i32>,
         AllocU32:Allocator<u32>,
         AllocU64:Allocator<u64>,
         AllocCommand:Allocator<super::Command>,
         AllocFloatX:Allocator<super::util::floatX>,
         AllocV8:Allocator<super::v8>,
         AllocS16:Allocator<super::s16>,
         AllocPDF:Allocator<super::PDF>,
         AllocStaticCommand:Allocator<super::StaticCommand>,
         AllocHistogramLiteral:Allocator<super::HistogramLiteral>,
         AllocHistogramCommand:Allocator<super::HistogramCommand>,
         AllocHistogramDistance:Allocator<super::HistogramDistance>,
         AllocHistogramPair:Allocator<super::HistogramPair>,
         AllocContextType:Allocator<super::ContextType>,
         AllocHuffmanTree:Allocator<super::HuffmanTree>,
         AllocZopfliNode:Allocator<super::ZopfliNode>,
         > Allocator<$type_name> for CombiningAllocator<AllocU8,
                                                 AllocU16,
                                                 AllocI32,
                                                 AllocU32,
                                                 AllocU64,
                                                 AllocCommand,
                                                 AllocFloatX,
                                                 AllocV8,
                                                 AllocS16,
                                                 AllocPDF,
                                                 AllocStaticCommand,
                                                 AllocHistogramLiteral,
                                                 AllocHistogramCommand,
                                                 AllocHistogramDistance,
                                                 AllocHistogramPair,
                                                 AllocContextType,
                                                 AllocHuffmanTree,
                                                 AllocZopfliNode,
                                                 > {
      type AllocatedMemory = $bound_name::AllocatedMemory;
      fn alloc_cell(&mut self, size: usize) -> <Self as Allocator<$type_name>>::AllocatedMemory {
        self.$local_name.alloc_cell(size)
      }
      fn free_cell(&mut self, data: <Self as Allocator<$type_name>>::AllocatedMemory) {
        self.$local_name.free_cell(data)
      }
    }
};
}

implement_allocator!(AllocU16, u16, alloc_u16);
implement_allocator!(AllocI32, i32, alloc_i32);
implement_allocator!(AllocU32, u32, alloc_u32);
implement_allocator!(AllocU64, u64, alloc_u64);
implement_allocator!(AllocCommand, Command, alloc_c);
implement_allocator!(AllocFloatX, floatX, alloc_f);
implement_allocator!(AllocV8, v8, alloc_f32x8);
implement_allocator!(AllocS16, s16, alloc_i16x16);
implement_allocator!(AllocPDF, PDF, alloc_pdf);
implement_allocator!(AllocStaticCommand, StaticCommand, alloc_sc);
implement_allocator!(AllocHistogramLiteral, HistogramLiteral, alloc_hl);
implement_allocator!(AllocHistogramCommand, HistogramCommand, alloc_hc);
implement_allocator!(AllocHistogramDistance, HistogramDistance, alloc_hd);
implement_allocator!(AllocHistogramPair, HistogramPair, alloc_hp);
implement_allocator!(AllocContextType, ContextType, alloc_ct);
implement_allocator!(AllocHuffmanTree, HuffmanTree, alloc_ht);
implement_allocator!(AllocZopfliNode, ZopfliNode, alloc_zn);
