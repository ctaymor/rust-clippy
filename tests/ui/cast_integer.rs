#![allow(unused)]
#![warn(clippy::cast_integer)]

fn main() {
  let max32 = u32::MAX;
  max32 as u16;
  let maxUsize = usize::MAX;
  maxUsize as u64;
}
