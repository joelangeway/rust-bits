use std::num::Int;

pub fn blit(src : &[u64], src_offset : usize, dst : &mut [u64], dst_offset : usize, count : usize) {
  let src_end_offset = src_offset + count;
  let dst_end_offset = dst_offset + count;
  let src_word_start = src_offset >> 6;
  let dst_word_start = dst_offset >> 6;
  let src_word_end = src_end_offset >> 6;
  let dst_word_end = dst_end_offset >> 6;
  let src_bit_start = src_offset & 0x3f;
  let dst_bit_start = dst_offset & 0x3f;
//  let src_bit_end = src_end_offset & 0x3f;
  let dst_bit_end = dst_end_offset & 0x3f;
  
  if dst_word_start == dst_word_end  {
    let dst_mask = ((1u64 << count) - 1) << dst_bit_start;
    let w = 
      if src_word_start == src_word_end  {
        (src[src_word_start] >> src_bit_start) << dst_bit_start
      } else {
        ((src[src_word_start] >> src_bit_start) | (src[src_word_start + 1] << (64 - src_bit_start))) << dst_bit_start
      };
    dst[dst_word_start] = (dst[dst_word_start] & !dst_mask) | (dst_mask & w);
  } else {
    let mut src_word_pos = src_word_start;
    let mut src_bit_pos = src_bit_start;
    let mut dst_word_pos = dst_word_start;
    let mut dst_bit_pos = dst_bit_start;
    
    if dst_bit_pos > 0 {
      let bn = 64 - dst_bit_pos;
      let m = ((1u64 << bn) - 1) << dst_bit_pos;
      if src_bit_pos <= dst_bit_pos {
        dst[dst_word_pos] = ( dst[dst_word_pos] & !m ) | (m & ( ( src[ src_word_pos ] >> src_bit_pos ) << dst_bit_pos ));
      } else {
        let sn0 = 64 - src_bit_start;
        dst[dst_word_pos] = ( dst[dst_word_pos] & !m ) | (m & (
              ( src[ src_word_pos     ] >> (src_bit_pos - dst_bit_pos ) ) |
              ( src[ src_word_pos + 1 ] << (dst_bit_pos + sn0)      ) ) );
      }
      src_bit_pos += bn;
      if src_bit_pos >= 64 {
        src_word_pos += 1;
        src_bit_pos = src_bit_pos - 64; 
      }
      dst_bit_pos = 0;
      dst_word_pos += 1;
    }

    let src_shift_1 = 64 - src_bit_pos;

    if src_bit_pos == 0 {
      while dst_word_pos < dst_word_end {
        dst[dst_word_pos] = src[src_word_pos];
        src_word_pos += 1;
        dst_word_pos += 1;
      }
    } else {
      while dst_word_pos < dst_word_end {
        dst[dst_word_pos] = (src[src_word_pos] >> src_bit_pos) | (src[src_word_pos + 1] << src_shift_1);
        src_word_pos += 1;
        dst_word_pos += 1;
      }
    }
    if dst_bit_end > 0 {
      let end_mask = (1u64 << dst_bit_end) - 1;
      if dst_bit_pos >= src_shift_1 {
        dst[dst_word_pos] = ( dst[dst_word_pos] & !end_mask ) | (end_mask & (src[src_word_pos] >> src_bit_pos));
      } else {
        dst[dst_word_pos] = ( dst[dst_word_pos] & !end_mask ) | (end_mask & (
            (src[src_word_pos] >> src_bit_pos) | (src[src_word_pos + 1] << src_shift_1) ));
      }
    }
  }
}

#[test]
pub fn test_blit_1() {
  let src_bits = [ 
      0x1010101010101010u64, 0x2323230320232323u64, 0x4515454545451545u64, 0x6767676767676717u64 
  ];
  let mut dst_bits = [ 
      0x8989898989898989u64, 0xababababababababu64, 0xcdcdcdcdcdcdcdcdu64, 0xefefefefefefefefu64
  ];
  let control_bits = [
      0x2023898989898989u64, 0x4545154523232303u64, 0xcdcdcd1745154545u64, 0xefefefefefefefefu64
  ];

  //let src_str    = format!("{:x}{:x}{:x}{:x}", src_bits[3],     src_bits[2],     src_bits[1],     src_bits[0]);
  //let dst_str     = format!("{:x}{:x}{:x}{:x}", dst_bits[3],     dst_bits[2],     dst_bits[1],     dst_bits[0]);
  let control_str = format!("{:x}{:x}{:x}{:x}", control_bits[3], control_bits[2], control_bits[1], control_bits[0]);
  //println!("\nsrc_bits:      {}",     src_str);
  //println!("dst_bits:      {}",     dst_str);
  //println!("control_bits:  {}", control_str);
  
  blit(& src_bits, 80, &mut dst_bits, 48, 120);
  
  let test_str = format!("{:x}{:x}{:x}{:x}", dst_bits[3],     dst_bits[2],     dst_bits[1],     dst_bits[0]);

  //println!("control_bits:  {}", control_str);
  //println!("test_bits:     {}",    test_str);

  assert_eq!(control_str, test_str);
  
}

pub fn find_next_one(words : &[u64], offset : usize, limit : usize) -> usize {
  let mut word_pos = offset >> 6;
  let word_limit = limit >> 6;
  let start_bit_pos = offset & 0x3f;
  let bit_limit = limit & 0x3f;
  let start_mask = !((1u64 << start_bit_pos) - 1);
  let first_word = words[word_pos] & start_mask;
  if first_word != 0 {
    return (word_pos << 6) + first_word.trailing_zeros();
  }
  word_pos += 1;
  while(word_pos < word_limit && 0 == words[word_pos]) {
    word_pos += 1;
  }
  if(word_pos < word_limit) {
    return (word_pos << 6) +  words[word_pos].trailing_zeros();
  }
  let end_mask = (1u64 << bit_limit) - 1;
  let last_word = words[word_limit] & end_mask;
  if 0 != last_word {
    return (word_pos << 6) + last_word.trailing_zeros();
  }
  return limit;
}

struct OnesIterator <'a> {
  pub words : &'a[u64],
  pub offset : usize,
  pub limit : usize,
}

impl <'a> Iterator for OnesIterator<'a> {
  type Item = usize;

  fn next(&mut self) -> Option<usize> {
    let next_offset = find_next_one(&self.words, self.offset, self.limit);
    if next_offset >= self.limit {
      None
    } else {
      self.offset = next_offset + 1;
      Some(next_offset)
    }
  }
}

pub fn iter_ones<'a>(words : &'a[u64], offset : usize, limit : usize) -> OnesIterator<'a> {
  OnesIterator { words: words, offset: offset, limit: limit }
}

#[test]
pub fn test_iter_ones_1() {
  let bitarray = [0x0004000002000001u64, 0x2000001000000800u64,0x0000800000400000u64,0x0000000000000100u64 ];
  let onesarray = [ 0usize, 25, 50, 75, 100, 125, 150, 175, 200 ];

  let mut vec = Vec::new();
  for i in iter_ones(&bitarray, 80, 200) {
    println!("Found a one: {}", i);
    vec.push(i);
  }
  assert_eq!([100usize, 125, 150, 175], vec);

  vec.clear();
  for i in iter_ones(&bitarray, 0, 51) {
    println!("Found a one: {}", i);
    vec.push(i);
  }
  assert_eq!([0usize, 25, 50], vec);
}

