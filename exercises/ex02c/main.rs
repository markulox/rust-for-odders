// Declare your Agent struct and implement all required method here

// End of declaration zone

// DO NOT MAKE CHANGE ANYTHING ON FUNCTION: `dont_mess_my_work`!
fn dont_mess_my_work() -> String {
    let noob = Agent::new("Noob".into(), 12, 3, 25, 1.1, 4000.23, false);
    let anby = Agent::new("Anby".into(), 89, 2400, 1200, 92.45, 50.25, false);

    let (winner, loser) = if noob.action_attack(&anby) {
        (noob.name, anby.name)
    } else {
        (anby.name, noob.name)
    };
    format!("{} wins over {}", winner, loser)
}
// DO NOT MAKE CHANGE ANYTHING ON FUNCTION: `dont_mess_my_work`!

// You can experiment your code in here!
fn main() {}

/// DO NOT CHANGE BELOW HERE ///
mod test;