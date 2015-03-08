/*
  I think, we need to declare these as modules somewhere or they don't actually exist in the module space. We do that here. We don't bother declaring these pub because we'll republish all the important bits.
*/
mod bit_blit;
mod bit_search;
mod bit_iter;

/*
  We republish all the bits we want this module to provide. This is how we avoid putting everything in one file.
*/
pub use self::bit_blit::blit;
pub use self::bit_search::{find_first_one, find_last_one};
pub use self::bit_iter::{ForwardOnesIterator, BackwardOnesIterator, iter_ones, backward_iter_ones};
