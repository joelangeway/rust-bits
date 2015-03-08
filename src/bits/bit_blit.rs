
#[doc="
Destructively copies an extent from one array of bits to another. Bits are represented by `[u64]`, 64 per element in little-engian order, numbered from zero. If `src` overlaps with `dst` in anyway, it is undefined. If the corresponding offset plus `count` exceeds the bounds of the `[u64]` slice, it is undefined.
"]
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
