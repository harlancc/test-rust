pub fn add(left: usize, right: usize) -> usize {
    left + right
}

//test cargo-smart-release

pub struct My1{

pub data: u8, //test version

}

impl My1{
   pub fn new () ->Self{
      Self{
      data:0
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
