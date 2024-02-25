pub fn add(left: usize, right: usize) -> usize {
    left + right
}

use my1::My1;

pub struct My2{

pub my1: My1,

}

impl My2{
   pub fn new () ->Self{
      Self{
        my1: My1::new(),
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
