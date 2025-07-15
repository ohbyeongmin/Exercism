pub struct Allergies {
    allergies: Vec<Allergen>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

// 255

impl Allergies {
    pub fn new(score: u32) -> Self {
        let allergies: Vec<Allergen> = (0..8)
            .filter_map(|i| {
                if score & 2_u32.pow(i) == 0 {
                    return None;
                }
                if i == 0 {
                    Some(Allergen::Eggs)
                } else if i == 1 {
                    Some(Allergen::Peanuts)
                } else if i == 2 {
                    Some(Allergen::Shellfish)
                } else if i == 3 {
                    Some(Allergen::Strawberries)
                } else if i == 4 {
                    Some(Allergen::Tomatoes)
                } else if i == 5 {
                    Some(Allergen::Chocolate)
                } else if i == 6 {
                    Some(Allergen::Pollen)
                } else {
                    Some(Allergen::Cats)
                }
            })
            .collect();
        Self { allergies }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies.iter().any(|a| a == allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergies.clone()
    }
}
