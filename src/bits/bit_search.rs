use std::num::Int;

#[doc="
Finds the first set bit in an array of bits. Bits are represented by `words: [u64]`, 64 per elements in little-engian order, numbered from zero. Bit positions from `offset : usize`, inclusive, to `limit : usize`, exclusive, are considered. Returns None if no bits are set in considered range. Exceeding the bounds of the slice is undefined.
"]
pub fn find_first_one(words : &[u64], offset : usize, limit : usize) -> Option<usize> {
  let mut word_pos = offset >> 6;
  let word_limit = limit >> 6;
  let start_bit_pos = offset & 0x3f;
  let bit_limit = limit & 0x3f;
  let start_mask = !((1u64 << start_bit_pos) - 1);
  let end_mask = (1u64 << bit_limit) - 1;
  let first_word = words[word_pos] & start_mask & (if (word_pos == word_limit) { end_mask } else { !0u64 });
  if first_word != 0 {
    return Some((word_pos << 6) + first_word.trailing_zeros() as usize);
  }
  if(word_pos == word_limit) { return None; }
  word_pos += 1;
  while(word_pos < word_limit) {
    let w = words[word_pos];
    if 0 != w {
      return Some((word_pos << 6) + w.trailing_zeros() as usize);
    }
    word_pos += 1;
  }
  if bit_limit > 0 {
    let end_mask = (1u64 << bit_limit) - 1;
    let last_word = words[word_limit] & end_mask;
    if 0 != last_word {
      return Some((word_pos << 6) + last_word.trailing_zeros() as usize);
    }
  }
  return None;
}

#[test]
pub fn test_find_first_one_1() {
  let bitarray = [0x0004000002000001u64, 0x2000001000000800u64,0x0000800000400000u64,0x0000000000000300u64 ];
  let onesarray = [ 0usize, 25, 50, 75, 100, 125, 150, 175, 200, 201 ];

  assert_eq!(Some(100usize), find_first_one(&bitarray, 90,  256));
  assert_eq!(Some(200usize), find_first_one(&bitarray, 176, 256));
  assert_eq!(Some(200usize), find_first_one(&bitarray, 200, 256));
  assert_eq!(Some(201usize), find_first_one(&bitarray, 201, 256));
  assert_eq!(Some(100usize), find_first_one(&bitarray, 76,  125));
  assert_eq!(Some(50usize), find_first_one(&bitarray,  26,  75));
  assert_eq!(Some(0usize), find_first_one(&bitarray,  0,  256));
  assert_eq!(None, find_first_one(&bitarray,  1,  25));
  assert_eq!(None, find_first_one(&bitarray,  51,  75));
}

#[doc="
Finds the last set bit in an array of bits. Bits are represented by `words: [u64]`, 64 per elements in little-engian order, numbered from zero. Bit positions from `offset : usize`, inclusive, to `limit : usize`, exclusive, are considered. Returns None if no bits are set in considered range. Exceeding the bounds of the slice is undefined.
"]
pub fn find_last_one(words : &[u64], offset : usize, limit : usize) -> Option<usize> {
  let mut word_pos = limit >> 6;
  let start_bit_pos = limit & 0x3f;
  let word_bottom = offset >> 6;
  let bit_bottom = offset & 0x3f;
  let start_mask = (1u64 << start_bit_pos) - 1;
  let bottom_mask = !((1u64 << bit_bottom) - 1);
  if start_bit_pos > 0 {
    let first_word = words[word_pos] & start_mask & (if (word_pos == word_bottom) { bottom_mask } else { !0u64 });
    if first_word != 0 {
      return Some((word_pos << 6) + 63 - first_word.leading_zeros() as usize);
    }
  }
  if word_pos == word_bottom {
    return None;
  }
  word_pos -= 1;
  while word_pos > word_bottom {
    let w = words[word_pos];
    if 0 != w {
      return Some((word_pos << 6) + 63 - w.leading_zeros() as usize);
    }
    word_pos -= 1;
  }
  
  let last_word = words[word_bottom] & bottom_mask;
  if 0 != last_word {
    return Some((word_pos << 6) + 63 - last_word.leading_zeros() as usize);
  }

  return None;
}

#[test]
pub fn test_find_last_one_1() {
  let bitarray = [0x0004000002000001u64, 0x2000001000000800u64,0x0000000000000000u64,0x0000000000000300u64 ];
  let onesarray = [ 0usize, 25, 26, 50, 75, 100, 125, 200, 201 ];

  assert_eq!(Some(201usize), find_last_one(&bitarray, 0,  256));
  assert_eq!(Some(200usize), find_last_one(&bitarray, 199, 201));
  assert_eq!(Some(201usize), find_last_one(&bitarray, 200, 256));
  assert_eq!(None, find_last_one(&bitarray, 202, 256));
  assert_eq!(Some(125usize), find_last_one(&bitarray, 0,  199));
  assert_eq!(Some(100usize), find_last_one(&bitarray,  0, 125));
  assert_eq!(Some(201usize), find_last_one(&bitarray,  0,  256));
  assert_eq!(None, find_last_one(&bitarray,  126,  200));
  assert_eq!(None, find_last_one(&bitarray,  51,  75));
}
