use std::num::Int;

use bits::bit_search::{find_first_one, find_last_one};

pub struct ForwardOnesIterator <'a> {
  pub words : &'a[u64],
  pub current_word : u64,
  pub word_pos : usize,
  pub word_limit : usize,
  pub end_mask : u64,
}

impl <'a> Iterator for ForwardOnesIterator<'a> {
  type Item = usize;

  fn next(&mut self) -> Option<usize> {
    if(self.current_word == 0) { 
      if(self.word_pos >= self.word_limit) { return None; }
      self.word_pos += 1;
      self.current_word = self.words[self.word_pos];
      while(self.word_pos < self.word_limit) {
        self.current_word = self.words[self.word_pos];
        if(self.current_word != 0) { break; }
        self.word_pos += 1;
      }
      if(self.word_pos == self.word_limit) {
        self.current_word &= self.end_mask;
      }
      if(self.current_word == 0) { return None; }
    }
    let b = self.current_word.trailing_zeros() as usize;
    self.current_word ^= 1u64 << b;
    return Some((self.word_pos << 6) + b);
  }
}

#[doc="
Returns an Iterator of `usize` that enumerates all the bit positions greater than or equal to `offset` and less than `limit`, in ascending order, which contain 1.
"]
pub fn iter_ones<'a>(words : &'a[u64], offset : usize, limit : usize) -> ForwardOnesIterator<'a> {
  let word_start = offset >> 6;
  let bit_start = offset & 0x3f;
  let bit_limit = (limit - 1) & 0x3f;
  let word_limit = ((limit - 1) >> 6);
  let start_mask = !((1u64 << bit_start) - 1);
  let end_mask = if bit_limit == 63 { !0u64 } else {(1u64 << (1 + bit_limit)) - 1};
  ForwardOnesIterator { 
    words: words, 
    current_word: words[word_start] & start_mask & (if word_start == word_limit { end_mask } else { !0u64 }),
    word_pos: word_start, word_limit: word_limit, end_mask: end_mask
  }
}

#[test]
pub fn test_iter_ones_1() {
  let bitarray = [0x0004000002000001u64, 0x2000001000000800u64,0x0000800000400000u64,0x0000000000000300u64 ];
  let onesarray = [ 0usize, 25, 50, 75, 100, 125, 150, 175, 200, 201 ];

  let mut k = 0;

  let mut vec = Vec::new();
  for i in iter_ones(&bitarray, 80, 200) {
    k += 1;
    assert!(k < 100);
    println!("Found a one: {}", i);
    vec.push(i);
  }
  assert_eq!([100usize, 125, 150, 175], vec);

  vec.clear();
  for i in iter_ones(&bitarray, 80, 201) {
    k += 1;
    assert!(k < 100);
    println!("Found a one: {}", i);
    vec.push(i);
  }
  assert_eq!([100usize, 125, 150, 175, 200], vec);


  vec.clear();
  for i in iter_ones(&bitarray, 0, 51) {
    k += 1;
    assert!(k < 100);
    println!("Found a one: {}", i);
    vec.push(i);
  }
  assert_eq!([0usize, 25, 50], vec);

  vec.clear();
  for i in iter_ones(&bitarray, 74, 76) {
    k += 1;
    assert!(k < 100);
    println!("Found a one: {}", i);
    vec.push(i);
  }
  assert_eq!([75usize], vec);

  vec.clear();
  for i in iter_ones(&bitarray, 128, 256) {
    k += 1;
    assert!(k < 100);
    println!("Found a one: {}", i);
    vec.push(i);
  }
  assert_eq!([150usize, 175, 200, 201], vec);
}

pub struct BackwardOnesIterator <'a> {
  pub words : &'a[u64],
  pub offset : usize,
  pub limit : usize,
}

impl <'a> Iterator for BackwardOnesIterator<'a> {
  type Item = usize;

  fn next(&mut self) -> Option<usize> {
    match find_last_one(&self.words, self.offset, self.limit) {
      None => None,
      Some(next_limit) => {
        self.limit = next_limit;
        Some(next_limit)
      }
    }
  }
}

#[doc="
Returns an Iterator of `usize` that enumerates all the bit positions greater than or equal to `offset` and less than `limit`, in descending order, which contain 1.
"]
pub fn backward_iter_ones<'a>(words : &'a[u64], offset : usize, limit : usize) -> BackwardOnesIterator<'a> {
  BackwardOnesIterator { words: words, offset: offset, limit: limit }
}

#[test]
pub fn test_backward_iter_ones_1() {
  let bitarray = [0x0004000002000001u64, 0x2000001000000800u64,0x0000800000400000u64,0x0000000000000300u64 ];
  let onesarray = [ 0usize, 25, 50, 75, 100, 125, 150, 175, 200, 201 ];

  let mut vec = Vec::new();
  for i in backward_iter_ones(&bitarray, 80, 200) {
    println!("Found a one: {}", i);
    vec.push(i);
  }
  assert_eq!([175usize, 150, 125, 100], vec);

  vec.clear();
  for i in backward_iter_ones(&bitarray, 0, 51) {
    println!("Found a one: {}", i);
    vec.push(i);
  }
  assert_eq!([50usize, 25, 0], vec);

  vec.clear();
  for i in backward_iter_ones(&bitarray, 74, 76) {
    println!("Found a one: {}", i);
    vec.push(i);
  }
  assert_eq!([75usize], vec);

  vec.clear();
  for i in backward_iter_ones(&bitarray, 128, 256) {
    println!("Found a one: {}", i);
    vec.push(i);
  }
  assert_eq!([201usize, 200, 175, 150], vec);
}
