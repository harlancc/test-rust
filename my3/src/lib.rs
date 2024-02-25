pub fn add(left: usize, right: usize) -> usize {
    left + right
}

use my2::My2;

pub struct My3{

pub my2: My2,

}

impl My3{
   pub fn new () ->Self{
      Self{
        my2: My2::new(),
      }
   }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
