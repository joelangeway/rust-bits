
use bits::bit_search::{find_first_one, find_last_one};

pub struct ForwardOnesIterator <'a> {
  pub words : &'a[u64],
  pub offset : usize,
  pub limit : usize,
}

impl <'a> Iterator for ForwardOnesIterator<'a> {
  type Item = usize;

  fn next(&mut self) -> Option<usize> {
    match find_first_one(&self.words, self.offset, self.limit) {
      None => None,
      Some(next_offset) => {
        self.offset = next_offset + 1;
        Some(next_offset)
      }
    }
  }
}

pub fn iter_ones<'a>(words : &'a[u64], offset : usize, limit : usize) -> ForwardOnesIterator<'a> {
  ForwardOnesIterator { words: words, offset: offset, limit: limit }
}

#[test]
pub fn test_iter_ones_1() {
  let bitarray = [0x0004000002000001u64, 0x2000001000000800u64,0x0000800000400000u64,0x0000000000000300u64 ];
  let onesarray = [ 0usize, 25, 50, 75, 100, 125, 150, 175, 200, 201 ];

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

  vec.clear();
  for i in iter_ones(&bitarray, 74, 76) {
    println!("Found a one: {}", i);
    vec.push(i);
  }
  assert_eq!([75usize], vec);

  vec.clear();
  for i in iter_ones(&bitarray, 128, 256) {
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
