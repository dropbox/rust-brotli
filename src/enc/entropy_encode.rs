pub struct HuffmanTree {
    pub total_count_ : u32,
    pub index_left_ : i16,
    pub index_right_or_value_ : i16,
}



fn InitHuffmanTree(
    mut xself : &mut HuffmanTree,
    mut count : u32,
    mut left : i16,
    mut right : i16
) {
    (*xself).total_count_ = count;
    (*xself).index_left_ = left;
    (*xself).index_right_or_value_ = right;
}


fn BrotliSetDepth(
    p0: i32, pool: &mut[HuffmanTree], depth: &mut [u8], max_depth: i32)-> bool {
  let mut stack: [i16;16] = [0;16];
  let mut level = 0i32;
  let mut p = p0;
  assert!(max_depth <= 15);
  stack[0] = -1;
  loop {
    if (pool[p as usize].index_left_ >= 0) {
      level+=1;
      if (level > max_depth) {return false};
      stack[level as usize] = pool[p as usize].index_right_or_value_;
      p = pool[p as usize].index_left_ as i32;
      continue;
    } else {
      depth[pool[p as usize].index_right_or_value_ as usize] = level as u8;
    }
    while (level >= 0 && stack[level as usize] == -1){ level-=1;}
    if (level < 0){ return true;}
    p = stack[level as usize] as i32;
    stack[level as usize] = -1;
  }
}