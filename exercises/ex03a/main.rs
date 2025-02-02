struct Dmg(i32);
struct CritRate(f32);
struct CritDmg(f32);
struct Stamina(i16);

enum Equipment {
    Weapon(Dmg),
    Locket(CritRate, Stamina),
    Braclet(CritDmg, Dmg),
    Boots(Stamina)
}

fn total_stat() -> (Dmg, CritRate, CritDmg, Stamina) {
    let equipment_bag: [Equipment; 7] = [
        Equipment::Weapon(Dmg(1200)),
        Equipment::Locket(CritRate(13.50), Stamina(15)),
        Equipment::Braclet(CritDmg(70.2), Dmg(300)),
        Equipment::Boots(Stamina(50)),
        Equipment::Weapon(Dmg(780)),
        Equipment::Locket(CritRate(16.23), Stamina(10)),
        Equipment::Braclet(CritDmg(50.25), Dmg(200)),
    ];
    let mut total_damage:i32 = 0;
    let mut total_crit_rate:f32 = 0.0;
    let mut total_crit_damage:f32 = 0.0;
    let mut total_stamina:i16 = 0;
    
    todo!()
}
// End of implementation zone

// You can experiment your code in here!
fn main(){

}

/// DO NOT CHANGE BELOW HERE ///
mod test;