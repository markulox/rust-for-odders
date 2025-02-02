

////////// DO NOT CHANGE BELOW HERE /////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_produce_juice_failed() {
        match produce_juice(vec![Ingredient::Salmon]) {
            Ok(_) => panic!("Salmon shouldn't be able to convert to juice"),
            Err(e) => assert_eq!(e, "This is not a fruit".to_string()),
        }

        match produce_juice(vec![Ingredient::Chilli]) {
            Ok(_) => panic!("Chili shouldn't be able to convert to juice"),
            Err(e) => assert_eq!(e, "This is not a fruit".to_string()),
        }

        match produce_juice(vec![Ingredient::Banana, Ingredient::Salmon]) {
            Ok(_) => panic!("Eventhough there is a banana, Salmon still shoudn't be convertable"),
            Err(e) => assert_eq!(e, "This is not a fruit".to_string()),
        }
    }

    #[test]
    fn test_produce_juice_pass(){
        if let Ok(v_j) = produce_juice(
            vec![
                Ingredient::Kiwi,
                Ingredient::Strawberry,
                Ingredient::PineApple
                ]
        ) {
            assert_eq!(v_j, vec![
                FruitDrink::KiwiSmoothie,
                FruitDrink::StrawberrySmoothie,
                FruitDrink::PineAppleJuice
                ])
        }

        if let Ok(v_j) = produce_juice(
            vec![
                Ingredient::Banana,
                ]
        ) {
            assert_eq!(v_j, vec![FruitDrink::BananaSmoothie])
        }
    }

    #[test]
    fn test_factory_producing_all_pass() {
        let reports = factory_producing(vec![
            vec![Ingredient::Strawberry, Ingredient::Kiwi],
            vec![Ingredient::Banana, Ingredient::PineApple]
        ]);
        assert_eq!(reports[0], "Lot no.: 1\n\t- Strawberry Smoothie\n\t- Kiwi Smoothie\n");
        assert_eq!(reports[1], "Lot no.: 2\n\t- Banana Smoothie\n\t- Pine Apple Juice\n")
    }

    #[test]
    fn test_factory_producing_some_failed(){
        let reports = factory_producing(vec![
            vec![Ingredient::Salmon, Ingredient::Kiwi],
            vec![Ingredient::Banana, Ingredient::Chilli]
        ]);
        assert_eq!(reports[0], "Lot no.: 1\n\t[X] Some ingredients are not fruit\n");
        assert_eq!(reports[1], "Lot no.: 2\n\t[X] Some ingredients are not fruit\n")
    }

}

fn main() {
    let reports = factory_producing(vec![
        vec![Ingredient::Salmon, Ingredient::Kiwi],
        vec![Ingredient::Banana, Ingredient::Chilli]
    ]);
    for e_report in reports{
        println!("{}", e_report)
    }
}
