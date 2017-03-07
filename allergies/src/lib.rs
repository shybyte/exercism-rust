
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Allergen {
    Eggs = 0,
    Peanuts = 1,
    Shellfish = 2,
    Strawberries = 3,
    Tomatoes = 4,
    Chocolate = 5,
    Pollen = 6,
    Cats = 7,
}

use Allergen::*;

static ALLERGENS: &'static [Allergen] =
    &[Eggs, Peanuts, Shellfish, Strawberries, Tomatoes, Chocolate, Pollen, Cats];

pub struct Allergies {
    bit_set: u8,
}

impl Allergies {
    pub fn new(bit_set: u8) -> Allergies {
        Allergies { bit_set: bit_set }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        (self.bit_set & (1 << *allergen as u8)) > 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        ALLERGENS.iter()
            .filter(|allergen| self.is_allergic_to(allergen))
            .cloned()
            .collect()
    }
}
