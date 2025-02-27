// Implement NumberCollection struct here

struct Success();
struct NothingLeft();
trait ContainEadible {
    fn consume(&mut self) -> Result<Success,NothingLeft>;
    fn peek_name(&self) -> String;
    fn check_remaining(&self) -> usize;
}

struct FoodBag {
    
}
// End of implementation zone

// You can experiment your code in here!
fn main(){}

/// DO NOT CHANGE BELOW HERE ///
mod test;