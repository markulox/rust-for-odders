enum Snack {
    BunBun,
    Lay,
    RollerCoaster,
    Taro
}

impl Snack {
    fn get_snack_name(&self) -> String{
        match &self {
            Snack::BunBun => "BunBun".into(),
            Snack::Lay => "Lay".into(),
            Snack::RollerCoaster => "RollerCoaster".into(),
            Snack::Taro => "Taro".into(),
        }
    }
}
// Implement your SnackBasket struct here
todo!()
// End of implementation zone


// You can experiment your code in here!
fn main(){
    
}

/// DO NOT CHANGE BELOW HERE ///
mod test;