use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Sellable {
    pub base: Product,
    pub name: String,
    pub effects: HashSet<Effect>,
    pub ingredients: Vec<Ingredient>,
}

impl Sellable {
    pub fn sell_price(&self) -> f32 {
        if self.ingredients.is_empty() {
            return self.base.sell_price();
        }

        let multiplier_sum = self
            .effects
            .iter()
            .map(|effect| {
                if self.base.effects().contains(effect) {
                    0.
                } else {
                    effect.multiplier()
                }
            })
            .sum::<f32>();
        self.base.sell_price() * (1. + multiplier_sum)
    }
    pub fn from_product(product: Product) -> Self {
        let effects = product.effects();
        Sellable {
            base: product,
            effects,
            ingredients: Vec::new(),
            name: format!("{:?}", product),
        }
    }

    pub fn apply_effects(&mut self, remove: Effect, add: Effect, condition: Option<bool>) {
        if self.effects.contains(&remove)
            && !self.effects.contains(&add)
            && condition.unwrap_or(true)
        {
            self.effects.remove(&remove);
            self.effects.insert(add);
        }
    }

    pub fn add_ingredient(&mut self, ingredient: Ingredient) -> Self {
        let effects = self.effects.clone();
        match ingredient {
            Ingredient::Cuke => {
                self.apply_effects(Effect::Munchies, Effect::Athletic, None);
                self.apply_effects(
                    Effect::Slippery,
                    Effect::Athletic,
                    Some(self.effects.contains(&Effect::Munchies)),
                );
                self.apply_effects(Effect::Foggy, Effect::Cyclopean, None);
                self.apply_effects(Effect::Euphoric, Effect::Laxative, None);
                self.apply_effects(Effect::Toxic, Effect::Euphoric, None);
                self.apply_effects(Effect::Slippery, Effect::Munchies, None);
                self.apply_effects(Effect::Sneaky, Effect::Paranoia, None);
                self.apply_effects(Effect::Gingeritis, Effect::ThoughtProvoking, None);
                self.effects.insert(Effect::Energizing);
            }
            Ingredient::Banana => {
                self.apply_effects(Effect::Smelly, Effect::AntiGravity, None);
                self.apply_effects(Effect::Focused, Effect::SeizureInducing, None);
                self.apply_effects(Effect::Disorienting, Effect::Focused, None);
                self.apply_effects(Effect::Paranoia, Effect::Jennerising, None);
                self.apply_effects(Effect::LongFaced, Effect::Refreshing, None);
                self.apply_effects(Effect::Toxic, Effect::Smelly, None);
                self.apply_effects(Effect::Calming, Effect::Sneaky, None);
                self.apply_effects(Effect::Cyclopean, Effect::ThoughtProvoking, None);
                self.apply_effects(
                    Effect::Energizing,
                    Effect::ThoughtProvoking,
                    Some(!self.effects.contains(&Effect::Cyclopean)),
                );
                self.effects.insert(Effect::Gingeritis);
            }
            Ingredient::Paracetamol => {
                self.apply_effects(Effect::Munchies, Effect::AntiGravity, None);
                self.apply_effects(Effect::Electrifying, Effect::Athletic, None);
                self.apply_effects(Effect::Paranoia, Effect::Balding, None);
                self.apply_effects(
                    Effect::Energizing,
                    Effect::Paranoia,
                    Some(!self.effects.contains(&Effect::Munchies)),
                );
                self.apply_effects(
                    Effect::Energizing,
                    Effect::Balding,
                    Some(!self.effects.contains(&Effect::Paranoia)),
                );
                self.apply_effects(Effect::Spicy, Effect::BrightEyed, None);
                self.apply_effects(Effect::Calming, Effect::Slippery, None);
                self.apply_effects(Effect::Foggy, Effect::Calming, None);
                self.apply_effects(Effect::Focused, Effect::Gingeritis, None);
                self.apply_effects(Effect::Toxic, Effect::TropicThunder, None);
                self.apply_effects(Effect::Glowing, Effect::Toxic, None);
                self.effects.insert(Effect::Sneaky);
            }
            Ingredient::Donut => {
                self.apply_effects(Effect::Shrinking, Effect::Energizing, None);
                self.apply_effects(Effect::Focused, Effect::Euphoric, None);
                self.apply_effects(Effect::CalorieDense, Effect::Explosive, None);
                self.apply_effects(Effect::Jennerising, Effect::Gingeritis, None);
                self.apply_effects(Effect::AntiGravity, Effect::Slippery, None);
                self.apply_effects(Effect::Balding, Effect::Sneaky, None);
                self.effects.insert(Effect::CalorieDense);
            }
            Ingredient::Viagra => {
                self.apply_effects(Effect::Euphoric, Effect::BrightEyed, None);
                self.apply_effects(Effect::Laxative, Effect::Calming, None);
                self.apply_effects(Effect::Athletic, Effect::Sneaky, None);
                self.apply_effects(Effect::Disorienting, Effect::Toxic, None);
                self.effects.insert(Effect::TropicThunder);
            }
            Ingredient::MouthWash => {
                self.apply_effects(Effect::Calming, Effect::AntiGravity, None);
                self.apply_effects(Effect::Focused, Effect::Jennerising, None);
                self.apply_effects(Effect::Explosive, Effect::Sedating, None);
                self.apply_effects(Effect::CalorieDense, Effect::Sneaky, None);
                self.effects.insert(Effect::Balding);
            }
            Ingredient::FluMedicine => {
                self.apply_effects(Effect::Calming, Effect::BrightEyed, None);
                self.apply_effects(Effect::Focused, Effect::Calming, None);
                self.apply_effects(Effect::Euphoric, Effect::Toxic, None);
                self.apply_effects(Effect::Laxative, Effect::Euphoric, None);
                self.apply_effects(Effect::Cyclopean, Effect::Foggy, None);
                self.apply_effects(Effect::ThoughtProvoking, Effect::Gingeritis, None);
                self.apply_effects(Effect::Munchies, Effect::Slippery, None);
                self.apply_effects(Effect::Athletic, Effect::Munchies, None);
                self.apply_effects(Effect::Shrinking, Effect::Paranoia, None);
                self.apply_effects(Effect::Electrifying, Effect::Refreshing, None);
                self.effects.insert(Effect::Sedating);
            }
            Ingredient::Gasoline => {
                self.apply_effects(Effect::Paranoia, Effect::Calming, None);
                self.apply_effects(Effect::Disorienting, Effect::Glowing, None);
                self.apply_effects(Effect::Electrifying, Effect::Disorienting, None);
                self.apply_effects(Effect::Shrinking, Effect::Focused, None);
                self.apply_effects(Effect::Laxative, Effect::Foggy, None);
                self.apply_effects(Effect::Munchies, Effect::Sedating, None);
                self.apply_effects(Effect::Gingeritis, Effect::Smelly, None);
                self.apply_effects(Effect::Sneaky, Effect::TropicThunder, None);
                self.apply_effects(Effect::Jennerising, Effect::Sneaky, None);
                self.apply_effects(
                    Effect::Euphoric,
                    Effect::Spicy,
                    Some(!self.effects.contains(&Effect::Energizing)),
                );
                self.apply_effects(Effect::Energizing, Effect::Euphoric, None);
                self.effects.insert(Effect::Toxic);
            }
            Ingredient::EnergyDrink => {
                self.apply_effects(Effect::Schizophrenia, Effect::Balding, None);
                self.apply_effects(Effect::Disorienting, Effect::Electrifying, None);
                self.apply_effects(Effect::Glowing, Effect::Disorienting, None);
                self.apply_effects(Effect::Euphoric, Effect::Energizing, None);
                self.apply_effects(Effect::Spicy, Effect::Euphoric, None);
                self.apply_effects(Effect::Foggy, Effect::Laxative, None);
                self.apply_effects(Effect::Sedating, Effect::Munchies, None);
                self.apply_effects(Effect::Focused, Effect::Shrinking, None);
                self.apply_effects(Effect::TropicThunder, Effect::Sneaky, None);
                self.effects.insert(Effect::Athletic);
            }
            Ingredient::MotorOil => {
                self.apply_effects(Effect::Paranoia, Effect::AntiGravity, None);
                self.apply_effects(
                    Effect::Munchies,
                    Effect::Schizophrenia,
                    Some(!self.effects.contains(&Effect::Energizing)),
                );
                self.apply_effects(Effect::Energizing, Effect::Munchies, None);
                self.apply_effects(Effect::Euphoric, Effect::Sedating, None);
                self.apply_effects(Effect::Foggy, Effect::Toxic, None);
                self.effects.insert(Effect::Slippery);
            }
            Ingredient::MegaBean => {
                self.apply_effects(Effect::Calming, Effect::Glowing, None);
                self.apply_effects(Effect::Sneaky, Effect::Calming, None);
                self.apply_effects(Effect::ThoughtProvoking, Effect::Cyclopean, None);
                self.apply_effects(
                    Effect::Energizing,
                    Effect::Cyclopean,
                    Some(!self.effects.contains(&Effect::ThoughtProvoking)),
                );
                self.apply_effects(Effect::Focused, Effect::Disorienting, None);
                self.apply_effects(Effect::Shrinking, Effect::Electrifying, None);
                self.apply_effects(Effect::SeizureInducing, Effect::Focused, None);
                self.apply_effects(Effect::Athletic, Effect::Laxative, None);
                self.apply_effects(Effect::Jennerising, Effect::Paranoia, None);
                self.apply_effects(Effect::Slippery, Effect::Toxic, None);
                self.effects.insert(Effect::Foggy);
            }
            Ingredient::Chili => {
                self.apply_effects(Effect::Sneaky, Effect::BrightEyed, None);
                self.apply_effects(Effect::Athletic, Effect::Euphoric, None);
                self.apply_effects(Effect::Laxative, Effect::LongFaced, None);
                self.apply_effects(Effect::Shrinking, Effect::Refreshing, None);
                self.apply_effects(Effect::Munchies, Effect::Toxic, None);
                self.apply_effects(Effect::AntiGravity, Effect::TropicThunder, None);
                self.effects.insert(Effect::Spicy);
            }
            Ingredient::Battery => {
                self.apply_effects(Effect::Laxative, Effect::CalorieDense, None);
                self.apply_effects(
                    Effect::Euphoric,
                    Effect::Zombifying,
                    Some(!self.effects.contains(&Effect::Electrifying)),
                );
                self.apply_effects(
                    Effect::Electrifying,
                    Effect::Euphoric,
                    Some(!self.effects.contains(&Effect::Zombifying)),
                );
                self.apply_effects(Effect::Cyclopean, Effect::Glowing, None);
                self.apply_effects(Effect::Munchies, Effect::TropicThunder, None);
                self.apply_effects(Effect::Shrinking, Effect::Munchies, None);
                self.effects.insert(Effect::BrightEyed);
            }
            Ingredient::Iodine => {
                self.apply_effects(Effect::CalorieDense, Effect::Gingeritis, None);
                self.apply_effects(Effect::Foggy, Effect::Paranoia, None);
                self.apply_effects(Effect::Calming, Effect::Balding, None);
                self.apply_effects(Effect::Euphoric, Effect::SeizureInducing, None);
                self.apply_effects(Effect::Toxic, Effect::Sneaky, None);
                self.apply_effects(Effect::Refreshing, Effect::ThoughtProvoking, None);
                self.effects.insert(Effect::Jennerising);
            }
            Ingredient::Addy => {
                self.apply_effects(Effect::LongFaced, Effect::Electrifying, None);
                self.apply_effects(Effect::Foggy, Effect::Energizing, None);
                self.apply_effects(Effect::Explosive, Effect::Euphoric, None);
                self.apply_effects(Effect::Sedating, Effect::Gingeritis, None);
                self.apply_effects(Effect::Glowing, Effect::Refreshing, None);
                self.effects.insert(Effect::ThoughtProvoking);
            }
            Ingredient::HorseSemen => {
                self.apply_effects(Effect::AntiGravity, Effect::Calming, None);
                self.apply_effects(Effect::ThoughtProvoking, Effect::Electrifying, None);
                self.apply_effects(Effect::Gingeritis, Effect::Refreshing, None);
                self.effects.insert(Effect::LongFaced);
            }
        }

        if effects == self.effects {
            // if there are no changes, return self
            return self.clone();
        }

        let mut ingredients = self.ingredients.clone();
        ingredients.push(ingredient);
        Sellable {
            base: self.base,
            ingredients,
            effects: self.effects.clone(),
            name: format!("{} + {:?}", self.name, ingredient),
        }
    }
    
    pub fn with_name(&self, name: String) -> Self {
        let mut new = self.clone();
        new.name = name;
        new
    }

    pub fn price(&self) -> f32 {
        let mut price = self.base.price();
        for ingredient in &self.ingredients {
            price += ingredient.price();
        }
        price
    }

    pub fn addictiveness(&self) -> f32 {
        self.base.addictiveness()
    }

    pub fn key(&self) -> String {
        let mut key = format!("{:?}", self.base);
        for ingredient in &self.ingredients {
            key.push_str(&format!("{:?}", ingredient));
        }

        key
    }

    pub fn save(&self) -> (String, Self) {
        (self.key(), self.clone())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Quality {
    Low,
    Medium,
    High,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Product {
    OGKush,
    SourDiesel,
    GreenCrack,
    GranddaddyPurple,
    Meth(Quality),
    Cocaine,
}

impl Product {
    pub fn price(&self) -> f32 {
        match self {
            Product::OGKush => 30.,
            Product::SourDiesel => 35.,
            Product::GreenCrack => 40.,
            Product::GranddaddyPurple => 45.,
            Product::Meth(quality) => match quality {
                Quality::Low => 60.,
                Quality::Medium => 80.,
                Quality::High => 110.,
            },
            Product::Cocaine => 80.,
        }
    }

    pub fn effects(&self) -> HashSet<Effect> {
        match self {
            Product::OGKush => HashSet::from([Effect::Calming]),
            Product::SourDiesel => HashSet::from([Effect::Refreshing]),
            Product::GreenCrack => HashSet::from([Effect::Energizing]),
            Product::GranddaddyPurple => HashSet::from([Effect::Sedating]),
            Product::Meth(_) => HashSet::new(),
            Product::Cocaine => HashSet::new(),
        }
    }

    pub fn sell_price(&self) -> f32 {
        match self {
            Product::OGKush => 38.,
            Product::SourDiesel => 40.,
            Product::GreenCrack => 43.,
            Product::GranddaddyPurple => 44.,
            Product::Meth(_) => 70.,
            Product::Cocaine => 150.,
        }
    }

    pub fn addictiveness(&self) -> f32 {
        match self {
            Product::OGKush => 0.,
            Product::SourDiesel => 0.1,
            Product::GreenCrack => 0.34,
            Product::GranddaddyPurple => 0.,
            Product::Meth(_) => 0.6,
            Product::Cocaine => 0.4,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Ingredient {
    Cuke,
    Banana,
    Paracetamol,
    Donut,
    Viagra,
    MouthWash,
    FluMedicine,
    Gasoline,
    EnergyDrink,
    MotorOil,
    MegaBean,
    Chili,
    Battery,
    Iodine,
    Addy,
    HorseSemen,
}

impl Ingredient {
    pub fn price(&self) -> f32 {
        match self {
            Ingredient::Cuke => 2.,
            Ingredient::Banana => 2.,
            Ingredient::Paracetamol => 3.,
            Ingredient::Donut => 3.,
            Ingredient::Viagra => 4.,
            Ingredient::MouthWash => 4.,
            Ingredient::FluMedicine => 5.,
            Ingredient::Gasoline => 5.,
            Ingredient::EnergyDrink => 6.,
            Ingredient::MotorOil => 6.,
            Ingredient::MegaBean => 7.,
            Ingredient::Chili => 7.,
            Ingredient::Battery => 8.,
            Ingredient::Iodine => 8.,
            Ingredient::Addy => 9.,
            Ingredient::HorseSemen => 9.,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Effect {
    AntiGravity,
    Athletic,
    Balding,
    BrightEyed,
    Calming,
    CalorieDense,
    Cyclopean,
    Disorienting,
    Electrifying,
    Energizing,
    Euphoric,
    Explosive,
    Focused,
    Foggy,
    Gingeritis,
    Glowing,
    Jennerising,
    Laxative,
    LongFaced,
    Munchies,
    Paranoia,
    Refreshing,
    Schizophrenia,
    Sedating,
    SeizureInducing,
    Shrinking,
    Slippery,
    Smelly,
    Sneaky,
    Spicy,
    ThoughtProvoking,
    Toxic,
    TropicThunder,
    Zombifying,
}

impl Effect {
    pub fn multiplier(&self) -> f32 {
        match self {
            Effect::AntiGravity => 0.54,
            Effect::Athletic => 0.32,
            Effect::Balding => 0.30,
            Effect::BrightEyed => 0.40,
            Effect::Calming => 0.10,
            Effect::CalorieDense => 0.28,
            Effect::Cyclopean => 0.56,
            Effect::Disorienting => 0.00,
            Effect::Electrifying => 0.50,
            Effect::Energizing => 0.22,
            Effect::Euphoric => 0.18,
            Effect::Explosive => 0.00,
            Effect::Focused => 0.16,
            Effect::Foggy => 0.36,
            Effect::Gingeritis => 0.20,
            Effect::Glowing => 0.48,
            Effect::Jennerising => 0.42,
            Effect::Laxative => 0.00,
            Effect::LongFaced => 0.52,
            Effect::Munchies => 0.12,
            Effect::Paranoia => 0.00,
            Effect::Refreshing => 0.14,
            Effect::Schizophrenia => 0.00,
            Effect::Sedating => 0.26,
            Effect::SeizureInducing => 0.00,
            Effect::Shrinking => 0.60,
            Effect::Slippery => 0.34,
            Effect::Smelly => 0.00,
            Effect::Sneaky => 0.24,
            Effect::Spicy => 0.38,
            Effect::ThoughtProvoking => 0.44,
            Effect::Toxic => 0.00,
            Effect::TropicThunder => 0.46,
            Effect::Zombifying => 0.58,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix_1() {
        let mut mix = Sellable::from_product(Product::Meth(Quality::Medium));
        assert!(mix.effects.is_empty());
        mix = mix.add_ingredient(Ingredient::Cuke);
        assert_eq!(mix.effects, HashSet::from([Effect::Energizing]));
        mix = mix.add_ingredient(Ingredient::Banana);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Gingeritis, Effect::ThoughtProvoking])
        );
        mix = mix.add_ingredient(Ingredient::Paracetamol);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Gingeritis, Effect::ThoughtProvoking, Effect::Sneaky])
        );
        mix = mix.add_ingredient(Ingredient::Donut);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Gingeritis,
                Effect::ThoughtProvoking,
                Effect::Sneaky,
                Effect::CalorieDense,
            ])
        );
        mix = mix.add_ingredient(Ingredient::Donut);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Gingeritis,
                Effect::ThoughtProvoking,
                Effect::Sneaky,
                Effect::CalorieDense,
                Effect::Explosive,
            ])
        );
        mix = mix.add_ingredient(Ingredient::Battery);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Gingeritis,
                Effect::ThoughtProvoking,
                Effect::Sneaky,
                Effect::CalorieDense,
                Effect::Explosive,
                Effect::BrightEyed,
            ])
        );
        mix = mix.add_ingredient(Ingredient::Iodine);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Gingeritis,
                Effect::ThoughtProvoking,
                Effect::Sneaky,
                Effect::CalorieDense,
                Effect::Explosive,
                Effect::BrightEyed,
                Effect::Jennerising,
            ])
        );
        mix = mix.add_ingredient(Ingredient::Addy);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::Gingeritis,
                Effect::ThoughtProvoking,
                Effect::Sneaky,
                Effect::CalorieDense,
                Effect::BrightEyed,
                Effect::Jennerising,
                Effect::Euphoric,
            ])
        );
        mix = mix.add_ingredient(Ingredient::Gasoline);
        assert_eq!(
            mix.effects,
            HashSet::from([
                Effect::ThoughtProvoking,
                Effect::Sneaky,
                Effect::CalorieDense,
                Effect::BrightEyed,
                Effect::Smelly,
                Effect::Spicy,
                Effect::Toxic,
                Effect::TropicThunder,
            ])
        );
    }

    #[test]
    fn test_og_addy() {
        let mix = Sellable::from_product(Product::OGKush).add_ingredient(Ingredient::Addy);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Calming, Effect::ThoughtProvoking]),
        );
        // assert_eq!(mix.sell_price().round(), 54.);
        // assert_eq!(mix.addictiveness(), 0.42);
    }
    #[test]
    fn test_sour_addy() {
        let mix = Sellable::from_product(Product::SourDiesel).add_ingredient(Ingredient::Addy);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Refreshing, Effect::ThoughtProvoking]),
        );
        // assert_eq!(mix.sell_price().round(), 55.);
        // assert_eq!(mix.addictiveness(), 0.52);
    }
    #[test]
    fn test_green_addy() {
        let mix = Sellable::from_product(Product::GreenCrack).add_ingredient(Ingredient::Addy);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Energizing, Effect::ThoughtProvoking]),
        );
        // assert_eq!(mix.sell_price().round(), 58.);
        // assert_eq!(mix.addictiveness(), 0.76);
    }
    #[test]
    fn test_meth_addy() {
        let mix =
            Sellable::from_product(Product::Meth(Quality::High)).add_ingredient(Ingredient::Addy);
        assert_eq!(mix.effects, HashSet::from([Effect::ThoughtProvoking]),);
        // assert_eq!(mix.sell_price().round(), 101.);
        // assert_eq!(mix.addictiveness(), 0.97);
    }
    #[test]
    fn test_cocaine_addy() {
        let mix = Sellable::from_product(Product::Cocaine).add_ingredient(Ingredient::Addy);
        assert_eq!(mix.effects, HashSet::from([Effect::ThoughtProvoking]),);
        // assert_eq!(mix.sell_price().round(), 216.);
        // assert_eq!(mix.addictiveness(), 0.77);
    }
    #[test]
    fn test_og_banana() {
        let mix = Sellable::from_product(Product::OGKush).add_ingredient(Ingredient::Banana);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Gingeritis, Effect::Sneaky]),
        );
        // assert_eq!(mix.sell_price().round(), 50.);
        // assert_eq!(mix.addictiveness(), 0.37);
    }
    #[test]
    fn test_sour_banana() {
        let mix = Sellable::from_product(Product::SourDiesel).add_ingredient(Ingredient::Banana);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Refreshing, Effect::Gingeritis]),
        );
        // assert_eq!(mix.sell_price().round(), 47.);
        // assert_eq!(mix.addictiveness(), 0.15);
    }
    #[test]
    fn test_green_banana() {
        let mix = Sellable::from_product(Product::GreenCrack).add_ingredient(Ingredient::Banana);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Gingeritis, Effect::ThoughtProvoking]),
        );
        // assert_eq!(mix.sell_price().round(), 57.);
        // assert_eq!(mix.addictiveness(), 0.42);
    }
    #[test]
    fn test_purple_banana() {
        let mix =
            Sellable::from_product(Product::GranddaddyPurple).add_ingredient(Ingredient::Banana);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Sedating, Effect::Gingeritis]),
        );
        // assert_eq!(mix.sell_price().round(), 51.);
        // assert_eq!(mix.addictiveness(), 0.05);
    }
    #[test]
    fn test_meth_banana() {
        let mix =
            Sellable::from_product(Product::Meth(Quality::High)).add_ingredient(Ingredient::Banana);
        assert_eq!(mix.effects, HashSet::from([Effect::Gingeritis]),);
        // assert_eq!(mix.sell_price().round(), 85.);
        // assert_eq!(mix.addictiveness(), 0.94);
    }
    #[test]
    fn test_cocaine_banana() {
        let mix = Sellable::from_product(Product::Cocaine).add_ingredient(Ingredient::Banana);
        assert_eq!(mix.effects, HashSet::from([Effect::Gingeritis]),);
        // assert_eq!(mix.sell_price().round(), 180.);
        // assert_eq!(mix.addictiveness(), 0.4);
    }
    #[test]
    fn test_og_battery() {
        let mix = Sellable::from_product(Product::OGKush).add_ingredient(Ingredient::Battery);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Calming, Effect::BrightEyed]),
        );
        // assert_eq!(mix.sell_price().round(), 52.);
        // assert_eq!(mix.addictiveness(), 0.25);
    }
    #[test]
    fn test_sour_battery() {
        let mix = Sellable::from_product(Product::SourDiesel).add_ingredient(Ingredient::Battery);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Refreshing, Effect::BrightEyed]),
        );
        // assert_eq!(mix.sell_price().round(), 54.);
        // assert_eq!(mix.addictiveness(), 0.35);
    }
    #[test]
    fn test_green_battery() {
        let mix = Sellable::from_product(Product::GreenCrack).add_ingredient(Ingredient::Battery);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Energizing, Effect::BrightEyed]),
        );
        // assert_eq!(mix.sell_price().round(), 57.);
        // assert_eq!(mix.addictiveness(), 0.59);
    }
    #[test]
    fn test_purple_battery() {
        let mix =
            Sellable::from_product(Product::GranddaddyPurple).add_ingredient(Ingredient::Battery);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Sedating, Effect::BrightEyed]),
        );
        // assert_eq!(mix.sell_price().round(), 58.);
        // assert_eq!(mix.addictiveness(), 0.25);
    }
    #[test]
    fn test_meth_battery() {
        let mix = Sellable::from_product(Product::Meth(Quality::High))
            .add_ingredient(Ingredient::Battery);
        assert_eq!(mix.effects, HashSet::from([Effect::BrightEyed]),);
        // assert_eq!(mix.sell_price().round(), 98.);
        // assert_eq!(mix.addictiveness(), 0.8);
    }
    #[test]
    fn test_cocaine_battery() {
        let mix = Sellable::from_product(Product::Cocaine).add_ingredient(Ingredient::Battery);
        assert_eq!(mix.effects, HashSet::from([Effect::BrightEyed]),);
        // assert_eq!(mix.sell_price().round(), 210.);
        // assert_eq!(mix.addictiveness(), 0.60);
    }
    #[test]
    fn test_og_chili() {
        let mix = Sellable::from_product(Product::OGKush).add_ingredient(Ingredient::Chili);
        assert_eq!(mix.effects, HashSet::from([Effect::Calming, Effect::Spicy]),);
        // assert_eq!(mix.sell_price().round(), 52.);
        // assert_eq!(mix.addictiveness(), 0.71);
    }
    #[test]
    fn test_sour_chili() {
        let mix = Sellable::from_product(Product::SourDiesel).add_ingredient(Ingredient::Chili);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Refreshing, Effect::Spicy]),
        );
        // assert_eq!(mix.sell_price().round(), 53.);
        // assert_eq!(mix.addictiveness(), 0.81);
    }
    #[test]
    fn test_green_chili() {
        let mix = Sellable::from_product(Product::GreenCrack).add_ingredient(Ingredient::Chili);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Energizing, Effect::Spicy]),
        );
        // assert_eq!(mix.sell_price().round(), 56.);
        // assert_eq!(mix.addictiveness(), 1.);
    }
    #[test]
    fn test_purple_chili() {
        let mix =
            Sellable::from_product(Product::GranddaddyPurple).add_ingredient(Ingredient::Chili);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Sedating, Effect::Spicy]),
        );
        // assert_eq!(mix.sell_price().round(), 57.);
        // assert_eq!(mix.addictiveness(), 0.71);
    }
    #[test]
    fn test_meth_chili() {
        let mix =
            Sellable::from_product(Product::Meth(Quality::High)).add_ingredient(Ingredient::Chili);
        assert_eq!(mix.effects, HashSet::from([Effect::Spicy]),);
        // assert_eq!(mix.sell_price().round(), 97.);
        // assert_eq!(mix.addictiveness(), 1.);
    }
    #[test]
    fn test_cocaine_chili() {
        let mix = Sellable::from_product(Product::Cocaine).add_ingredient(Ingredient::Chili);
        assert_eq!(mix.effects, HashSet::from([Effect::Spicy]),);
        // assert_eq!(mix.sell_price().round(), 207.);
        // assert_eq!(mix.addictiveness(), 1.);
    }
    #[test]
    fn test_og_cuke() {
        let mix = Sellable::from_product(Product::OGKush).add_ingredient(Ingredient::Cuke);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Calming, Effect::Energizing]),
        );
        // assert_eq!(mix.sell_price().round(), 46.);
        // assert_eq!(mix.addictiveness(), 0.39);
    }
    #[test]
    fn test_sour_cuke() {
        let mix = Sellable::from_product(Product::SourDiesel).add_ingredient(Ingredient::Cuke);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Refreshing, Effect::Energizing]),
        );
        // assert_eq!(mix.sell_price().round(), 48.);
        // assert_eq!(mix.addictiveness(), 0.49);
    }
    #[test]
    fn test_purple_cuke() {
        let mix =
            Sellable::from_product(Product::GranddaddyPurple).add_ingredient(Ingredient::Cuke);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Sedating, Effect::Energizing]),
        );
        // assert_eq!(mix.sell_price().round(), 52.);
        // assert_eq!(mix.addictiveness(), 0.39);
    }
    #[test]
    fn test_meth_cuke() {
        let mix =
            Sellable::from_product(Product::Meth(Quality::High)).add_ingredient(Ingredient::Cuke);
        assert_eq!(mix.effects, HashSet::from([Effect::Energizing]),);
        // assert_eq!(mix.sell_price().round(), 84.);
        // assert_eq!(mix.addictiveness(), 0.60);
    }
    #[test]
    fn test_cocaine_cuke() {
        let mix = Sellable::from_product(Product::Cocaine).add_ingredient(Ingredient::Cuke);
        assert_eq!(mix.effects, HashSet::from([Effect::Energizing]),);
        // assert_eq!(mix.sell_price().round(), 183.);
        // assert_eq!(mix.addictiveness(), 0.74);
    }
    #[test]
    fn test_og_donut() {
        let mix = Sellable::from_product(Product::OGKush).add_ingredient(Ingredient::Donut);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Calming, Effect::CalorieDense]),
        );
        // assert_eq!(mix.sell_price().round(), 48.);
        // assert_eq!(mix.addictiveness(), 0.15);
    }
    #[test]
    fn test_sour_donut() {
        let mix = Sellable::from_product(Product::SourDiesel).add_ingredient(Ingredient::Donut);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Refreshing, Effect::CalorieDense]),
        );
        // assert_eq!(mix.sell_price().round(), 50.);
        // assert_eq!(mix.addictiveness(), 0.25);
    }
    #[test]
    fn test_green_donut() {
        let mix = Sellable::from_product(Product::GreenCrack).add_ingredient(Ingredient::Donut);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Energizing, Effect::CalorieDense]),
        );
        // assert_eq!(mix.sell_price().round(), 52.);
        // assert_eq!(mix.addictiveness(), 0.49);
    }
    #[test]
    fn test_purple_donut() {
        let mix =
            Sellable::from_product(Product::GranddaddyPurple).add_ingredient(Ingredient::Donut);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Sedating, Effect::CalorieDense]),
        );
        // assert_eq!(mix.sell_price().round(), 54.);
        // assert_eq!(mix.addictiveness(), 0.15);
    }
    #[test]
    fn test_meth_donut() {
        let mix =
            Sellable::from_product(Product::Meth(Quality::High)).add_ingredient(Ingredient::Donut);
        assert_eq!(mix.effects, HashSet::from([Effect::CalorieDense]),);
        // assert_eq!(mix.sell_price().round(), 90.);
        // assert_eq!(mix.addictiveness(), 0.92);
    }
    #[test]
    fn test_cocaine_donut() {
        let mix = Sellable::from_product(Product::Cocaine).add_ingredient(Ingredient::Donut);
        assert_eq!(mix.effects, HashSet::from([Effect::CalorieDense]),);
        // assert_eq!(mix.sell_price().round(), 192.);
        // assert_eq!(mix.addictiveness(), 0.50);
    }
    #[test]
    fn test_og_energydrink() {
        let mix = Sellable::from_product(Product::OGKush).add_ingredient(Ingredient::EnergyDrink);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Calming, Effect::Athletic]),
        );
        // assert_eq!(mix.sell_price().round(), 50.);
        // assert_eq!(mix.addictiveness(), 0.65);
    }
    #[test]
    fn test_sour_energydrink() {
        let mix =
            Sellable::from_product(Product::SourDiesel).add_ingredient(Ingredient::EnergyDrink);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Refreshing, Effect::Athletic]),
        );
        // assert_eq!(mix.sell_price().round(), 51.);
        // assert_eq!(mix.addictiveness(), 0.76);
    }
    #[test]
    fn test_green_energydrink() {
        let mix =
            Sellable::from_product(Product::GreenCrack).add_ingredient(Ingredient::EnergyDrink);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Energizing, Effect::Athletic]),
        );
        // assert_eq!(mix.sell_price().round(), 54.);
        // assert_eq!(mix.addictiveness(), 0.99);
    }
    #[test]
    fn test_purple_energydrink() {
        let mix = Sellable::from_product(Product::GranddaddyPurple)
            .add_ingredient(Ingredient::EnergyDrink);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Munchies, Effect::Athletic]),
        );
        // assert_eq!(mix.sell_price().round(), 50.);
        // assert_eq!(mix.addictiveness(), 0.75);
    }
    #[test]
    fn test_meth_energydrink() {
        let mix = Sellable::from_product(Product::Meth(Quality::High))
            .add_ingredient(Ingredient::EnergyDrink);
        assert_eq!(mix.effects, HashSet::from([Effect::Athletic]),);
        // assert_eq!(mix.sell_price().round(), 92.);
        // assert_eq!(mix.addictiveness(), 1.);
    }
    #[test]
    fn test_cocaine_energydrink() {
        let mix = Sellable::from_product(Product::Cocaine).add_ingredient(Ingredient::EnergyDrink);
        assert_eq!(mix.effects, HashSet::from([Effect::Athletic]),);
        // assert_eq!(mix.sell_price().round(), 198.);
        // assert_eq!(mix.addictiveness(), 1.);
    }
    #[test]
    fn test_og_flumedicine() {
        let mix = Sellable::from_product(Product::OGKush).add_ingredient(Ingredient::FluMedicine);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Sedating, Effect::BrightEyed]),
        );
        // assert_eq!(mix.sell_price().round(), 58.);
        // assert_eq!(mix.addictiveness(), 0.25);
    }
    #[test]
    fn test_sour_flumedicine() {
        let mix =
            Sellable::from_product(Product::SourDiesel).add_ingredient(Ingredient::FluMedicine);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Refreshing, Effect::Sedating]),
        );
        // assert_eq!(mix.sell_price().round(), 49.);
        // assert_eq!(mix.addictiveness(), 0.15);
    }
    #[test]
    fn test_green_flumedicine() {
        let mix =
            Sellable::from_product(Product::GreenCrack).add_ingredient(Ingredient::FluMedicine);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Energizing, Effect::Sedating]),
        );
        // assert_eq!(mix.sell_price().round(), 52.);
        // assert_eq!(mix.addictiveness(), 0.39);
    }
    #[test]
    fn test_meth_flumedicine() {
        let mix = Sellable::from_product(Product::Meth(Quality::High))
            .add_ingredient(Ingredient::FluMedicine);
        assert_eq!(mix.effects, HashSet::from([Effect::Sedating]),);
        // assert_eq!(mix.sell_price().round(), 88.);
        // assert_eq!(mix.addictiveness(), 0.60);
    }
    #[test]
    fn test_cocaine_flumedicine() {
        let mix = Sellable::from_product(Product::Cocaine).add_ingredient(Ingredient::FluMedicine);
        assert_eq!(mix.effects, HashSet::from([Effect::Sedating]),);
        // assert_eq!(mix.sell_price().round(), 189.);
        // assert_eq!(mix.addictiveness(), 0.40);
    }
    #[test]
    fn test_og_gasoline() {
        let mix = Sellable::from_product(Product::OGKush).add_ingredient(Ingredient::Gasoline);
        assert_eq!(mix.effects, HashSet::from([Effect::Calming, Effect::Toxic]),);
        // assert_eq!(mix.sell_price().round(), 38.);
        // assert_eq!(mix.addictiveness(), 0.05);
    }
    #[test]
    fn test_sour_gasoline() {
        let mix = Sellable::from_product(Product::SourDiesel).add_ingredient(Ingredient::Gasoline);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Refreshing, Effect::Toxic]),
        );
        // assert_eq!(mix.sell_price().round(), 40.);
        // assert_eq!(mix.addictiveness(), 0.15);
    }
    #[test]
    fn test_green_gasoline() {
        let mix = Sellable::from_product(Product::GreenCrack).add_ingredient(Ingredient::Gasoline);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Euphoric, Effect::Toxic]),
        );
        // assert_eq!(mix.sell_price().round(), 40.);
        // assert_eq!(mix.addictiveness(), 0.28);
    }
    #[test]
    fn test_purple_gasoline() {
        let mix =
            Sellable::from_product(Product::GranddaddyPurple).add_ingredient(Ingredient::Gasoline);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Sedating, Effect::Toxic]),
        );
        // assert_eq!(mix.sell_price().round(), 44.);
        // assert_eq!(mix.addictiveness(), 0.05);
    }
    #[test]
    fn test_meth_gasoline() {
        let mix = Sellable::from_product(Product::Meth(Quality::High))
            .add_ingredient(Ingredient::Gasoline);
        assert_eq!(mix.effects, HashSet::from([Effect::Toxic]),);
        // assert_eq!(mix.sell_price().round(), 70.);
        // assert_eq!(mix.addictiveness(), 0.60);
    }
    #[test]
    fn test_cocaine_gasoline() {
        let mix = Sellable::from_product(Product::Cocaine).add_ingredient(Ingredient::Gasoline);
        assert_eq!(mix.effects, HashSet::from([Effect::Toxic]),);
        // assert_eq!(mix.sell_price().round(), 150.);
        // assert_eq!(mix.addictiveness(), 0.4);
    }
    #[test]
    fn test_og_horsesemen() {
        let mix = Sellable::from_product(Product::OGKush).add_ingredient(Ingredient::HorseSemen);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Calming, Effect::LongFaced]),
        );
        // assert_eq!(mix.sell_price().round(), 57.);
        // assert_eq!(mix.addictiveness(), 0.65);
    }
    #[test]
    fn test_sour_horsesemen() {
        let mix =
            Sellable::from_product(Product::SourDiesel).add_ingredient(Ingredient::HorseSemen);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Refreshing, Effect::LongFaced]),
        );
        // assert_eq!(mix.sell_price().round(), 61.);
        // assert_eq!(mix.addictiveness(), 0.99);
    }
    #[test]
    fn test_green_horsesemen() {
        let mix =
            Sellable::from_product(Product::GreenCrack).add_ingredient(Ingredient::HorseSemen);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Energizing, Effect::LongFaced]),
        );
        // assert_eq!(mix.sell_price().round(), 58.);
        // assert_eq!(mix.addictiveness(), 0.76);
    }
    #[test]
    fn test_purple_horsesemen() {
        let mix = Sellable::from_product(Product::GranddaddyPurple)
            .add_ingredient(Ingredient::HorseSemen);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Sedating, Effect::LongFaced]),
        );
        // assert_eq!(mix.sell_price().round(), 62.);
        // assert_eq!(mix.addictiveness(), 0.65);
    }
    #[test]
    fn test_meth_horsesemen() {
        let mix = Sellable::from_product(Product::Meth(Quality::High))
            .add_ingredient(Ingredient::HorseSemen);
        assert_eq!(mix.effects, HashSet::from([Effect::LongFaced]),);
        // assert_eq!(mix.sell_price().round(), 106.);
        // assert_eq!(mix.addictiveness(), 1.);
    }
    #[test]
    fn test_cocaine_horsesemen() {
        let mix = Sellable::from_product(Product::Cocaine).add_ingredient(Ingredient::HorseSemen);
        assert_eq!(mix.effects, HashSet::from([Effect::LongFaced]),);
        // assert_eq!(mix.sell_price().round(), 228.);
        // assert_eq!(mix.addictiveness(), 1.);
    }
    #[test]
    fn test_og_iodine() {
        let mix = Sellable::from_product(Product::OGKush).add_ingredient(Ingredient::Iodine);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Balding, Effect::Jennerising]),
        );
        // assert_eq!(mix.sell_price().round(), 60.);
        // assert_eq!(mix.addictiveness(), 0.39);
    }
    #[test]
    fn test_sour_iodine() {
        let mix = Sellable::from_product(Product::SourDiesel).add_ingredient(Ingredient::Iodine);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::ThoughtProvoking, Effect::Jennerising]),
        );
        // assert_eq!(mix.sell_price().round(), 65.);
        // assert_eq!(mix.addictiveness(), 0.76);
    }
    #[test]
    fn test_green_iodine() {
        let mix = Sellable::from_product(Product::GreenCrack).add_ingredient(Ingredient::Iodine);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Energizing, Effect::Jennerising]),
        );
        // assert_eq!(mix.sell_price().round(), 57.);
        // assert_eq!(mix.addictiveness(), 0.73);
    }
    #[test]
    fn test_purple_iodine() {
        let mix =
            Sellable::from_product(Product::GranddaddyPurple).add_ingredient(Ingredient::Iodine);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Sedating, Effect::Jennerising]),
        );
        // assert_eq!(mix.sell_price().round(), 59.);
        // assert_eq!(mix.addictiveness(), 0.39);
    }
    #[test]
    fn test_meth_iodine() {
        let mix =
            Sellable::from_product(Product::Meth(Quality::High)).add_ingredient(Ingredient::Iodine);
        assert_eq!(mix.effects, HashSet::from([Effect::Jennerising]),);
        // assert_eq!(mix.sell_price().round(), 99.);
        // assert_eq!(mix.addictiveness(), 0.94);
    }
    #[test]
    fn test_cocaine_iodine() {
        let mix = Sellable::from_product(Product::Cocaine).add_ingredient(Ingredient::Iodine);
        assert_eq!(mix.effects, HashSet::from([Effect::Jennerising]),);
        // assert_eq!(mix.sell_price().round(), 213.);
        // assert_eq!(mix.addictiveness(), 0.74);
    }
    #[test]
    fn test_og_megabean() {
        let mix = Sellable::from_product(Product::OGKush).add_ingredient(Ingredient::MegaBean);
        assert_eq!(mix.effects, HashSet::from([Effect::Foggy, Effect::Glowing]),);
        // assert_eq!(mix.sell_price().round(), 64.);
        // assert_eq!(mix.addictiveness(), 0.62);
    }
    #[test]
    fn test_sour_megabean() {
        let mix = Sellable::from_product(Product::SourDiesel).add_ingredient(Ingredient::MegaBean);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Refreshing, Effect::Foggy]),
        );
        // assert_eq!(mix.sell_price().round(), 52.);
        // assert_eq!(mix.addictiveness(), 0.25);
    }
    #[test]
    fn test_green_megabean() {
        let mix = Sellable::from_product(Product::GreenCrack).add_ingredient(Ingredient::MegaBean);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Foggy, Effect::Cyclopean]),
        );
        // assert_eq!(mix.sell_price().round(), 67.);
        // assert_eq!(mix.addictiveness(), 0.25);
    }
    #[test]
    fn test_purple_megabean() {
        let mix =
            Sellable::from_product(Product::GranddaddyPurple).add_ingredient(Ingredient::MegaBean);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Sedating, Effect::Foggy]),
        );
        // assert_eq!(mix.sell_price().round(), 57.);
        // assert_eq!(mix.addictiveness(), 0.15);
    }
    #[test]
    fn test_meth_megabean() {
        let mix = Sellable::from_product(Product::Meth(Quality::High))
            .add_ingredient(Ingredient::MegaBean);
        assert_eq!(mix.effects, HashSet::from([Effect::Foggy]),);
        // assert_eq!(mix.sell_price().round(), 95.);
        // assert_eq!(mix.addictiveness(), 0.70);
    }
    #[test]
    fn test_cocaine_megabean() {
        let mix = Sellable::from_product(Product::Cocaine).add_ingredient(Ingredient::MegaBean);
        assert_eq!(mix.effects, HashSet::from([Effect::Foggy]),);
        // assert_eq!(mix.sell_price().round(), 204.);
        // assert_eq!(mix.addictiveness(), 0.50);
    }
    #[test]
    fn test_og_motoroil() {
        let mix = Sellable::from_product(Product::OGKush).add_ingredient(Ingredient::MotorOil);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Calming, Effect::Slippery]),
        );
        // assert_eq!(mix.sell_price().round(), 50.);
        // assert_eq!(mix.addictiveness(), 0.35);
    }
    #[test]
    fn test_sour_motoroil() {
        let mix = Sellable::from_product(Product::SourDiesel).add_ingredient(Ingredient::MotorOil);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Refreshing, Effect::Slippery]),
        );
        // assert_eq!(mix.sell_price().round(), 52.);
        // assert_eq!(mix.addictiveness(), 0.46);
    }
    #[test]
    fn test_green_motoroil() {
        let mix = Sellable::from_product(Product::GreenCrack).add_ingredient(Ingredient::MotorOil);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Munchies, Effect::Slippery]),
        );
        // assert_eq!(mix.sell_price().round(), 51.);
        // assert_eq!(mix.addictiveness(), 0.45);
    }
    #[test]
    fn test_purple_motoroil() {
        let mix =
            Sellable::from_product(Product::GranddaddyPurple).add_ingredient(Ingredient::MotorOil);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Sedating, Effect::Slippery]),
        );
        // assert_eq!(mix.sell_price().round(), 56.);
        // assert_eq!(mix.addictiveness(), 0.35);
    }
    #[test]
    fn test_meth_motoroil() {
        let mix = Sellable::from_product(Product::Meth(Quality::High))
            .add_ingredient(Ingredient::MotorOil);
        assert_eq!(mix.effects, HashSet::from([Effect::Slippery]),);
        // assert_eq!(mix.sell_price().round(), 94.);
        // assert_eq!(mix.addictiveness(), 0.90);
    }
    #[test]
    fn test_cocaine_motoroil() {
        let mix = Sellable::from_product(Product::Cocaine).add_ingredient(Ingredient::MotorOil);
        assert_eq!(mix.effects, HashSet::from([Effect::Slippery]),);
        // assert_eq!(mix.sell_price().round(), 201.);
        // assert_eq!(mix.addictiveness(), 0.70);
    }
    #[test]
    fn test_og_mouthwash() {
        let mix = Sellable::from_product(Product::OGKush).add_ingredient(Ingredient::MouthWash);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Balding, Effect::AntiGravity]),
        );
        // assert_eq!(mix.sell_price().round(), 64.);
        // assert_eq!(mix.addictiveness(), 0.66);
    }
    #[test]
    fn test_sour_mouthwash() {
        let mix = Sellable::from_product(Product::SourDiesel).add_ingredient(Ingredient::MouthWash);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Balding, Effect::Refreshing]),
        );
        // assert_eq!(mix.sell_price().round(), 50.);
        // assert_eq!(mix.addictiveness(), 0.15);
    }
    #[test]
    fn test_green_mouthwash() {
        let mix = Sellable::from_product(Product::GreenCrack).add_ingredient(Ingredient::MouthWash);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Energizing, Effect::Balding]),
        );
        // assert_eq!(mix.sell_price().round(), 53.);
        // assert_eq!(mix.addictiveness(), 0.39);
    }
    #[test]
    fn test_purple_mouthwash() {
        let mix =
            Sellable::from_product(Product::GranddaddyPurple).add_ingredient(Ingredient::MouthWash);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Sedating, Effect::Balding]),
        );
        // assert_eq!(mix.sell_price().round(), 55.);
        // assert_eq!(mix.addictiveness(), 0.05);
    }
    #[test]
    fn test_meth_mouthwash() {
        let mix = Sellable::from_product(Product::Meth(Quality::High))
            .add_ingredient(Ingredient::MouthWash);
        assert_eq!(mix.effects, HashSet::from([Effect::Balding]),);
        // assert_eq!(mix.sell_price().round(), 91.);
        // assert_eq!(mix.addictiveness(), 0.60);
    }
    #[test]
    fn test_cocaine_mouthwash() {
        let mix = Sellable::from_product(Product::Cocaine).add_ingredient(Ingredient::MouthWash);
        assert_eq!(mix.effects, HashSet::from([Effect::Balding]),);
        // assert_eq!(mix.sell_price().round(), 195.);
        // assert_eq!(mix.addictiveness(), 0.40);
    }
    #[test]
    fn test_og_paracetamol() {
        let mix = Sellable::from_product(Product::OGKush).add_ingredient(Ingredient::Paracetamol);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Sneaky, Effect::Slippery]),
        );
        // assert_eq!(mix.sell_price().round(), 55.);
        // assert_eq!(mix.addictiveness(), 0.68);
    }
    #[test]
    fn test_sour_paracetamol() {
        let mix =
            Sellable::from_product(Product::SourDiesel).add_ingredient(Ingredient::Paracetamol);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Refreshing, Effect::Sneaky]),
        );
        // assert_eq!(mix.sell_price().round(), 48.);
        // assert_eq!(mix.addictiveness(), 0.48);
    }
    #[test]
    fn test_green_paracetamol() {
        let mix =
            Sellable::from_product(Product::GreenCrack).add_ingredient(Ingredient::Paracetamol);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Paranoia, Effect::Sneaky]),
        );
        // assert_eq!(mix.sell_price().round(), 43.);
        // assert_eq!(mix.addictiveness(), 0.37);
    }
    #[test]
    fn test_purple_paracetamol() {
        let mix = Sellable::from_product(Product::GranddaddyPurple)
            .add_ingredient(Ingredient::Paracetamol);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Sedating, Effect::Sneaky]),
        );
        // assert_eq!(mix.sell_price().round(), 52.);
        // assert_eq!(mix.addictiveness(), 0.37);
    }
    #[test]
    fn test_meth_paracetamol() {
        let mix = Sellable::from_product(Product::Meth(Quality::High))
            .add_ingredient(Ingredient::Paracetamol);
        assert_eq!(mix.effects, HashSet::from([Effect::Sneaky]),);
        // assert_eq!(mix.sell_price().round(), 87.);
        // assert_eq!(mix.addictiveness(), 0.70);
    }
    #[test]
    fn test_cocaine_paracetamol() {
        let mix = Sellable::from_product(Product::Cocaine).add_ingredient(Ingredient::Paracetamol);
        assert_eq!(mix.effects, HashSet::from([Effect::Sneaky]),);
        // assert_eq!(mix.sell_price().round(), 186.);
        // assert_eq!(mix.addictiveness(), 0.72);
    }
    #[test]
    fn test_og_viagra() {
        let mix = Sellable::from_product(Product::OGKush).add_ingredient(Ingredient::Viagra);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Calming, Effect::TropicThunder]),
        );
        // assert_eq!(mix.sell_price().round(), 55.);
        // assert_eq!(mix.addictiveness(), 0.85);
    }
    #[test]
    fn test_sour_viagra() {
        let mix = Sellable::from_product(Product::SourDiesel).add_ingredient(Ingredient::Viagra);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Refreshing, Effect::TropicThunder]),
        );
        // assert_eq!(mix.sell_price().round(), 56.);
        // assert_eq!(mix.addictiveness(), 0.95);
    }
    #[test]
    fn test_green_viagra() {
        let mix = Sellable::from_product(Product::GreenCrack).add_ingredient(Ingredient::Viagra);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Energizing, Effect::TropicThunder]),
        );
        // assert_eq!(mix.sell_price().round(), 59.);
        // assert_eq!(mix.addictiveness(), 0.95);
    }
    #[test]
    fn test_purple_viagra() {
        let mix =
            Sellable::from_product(Product::GranddaddyPurple).add_ingredient(Ingredient::Viagra);
        assert_eq!(
            mix.effects,
            HashSet::from([Effect::Sedating, Effect::TropicThunder]),
        );
        // assert_eq!(mix.sell_price().round(), 60.);
        // assert_eq!(mix.addictiveness(), 0.85);
    }
    #[test]
    fn test_meth_viagra() {
        let mix =
            Sellable::from_product(Product::Meth(Quality::High)).add_ingredient(Ingredient::Viagra);
        assert_eq!(mix.effects, HashSet::from([Effect::TropicThunder]),);
        // assert_eq!(mix.sell_price().round(), 102.);
        // assert_eq!(mix.addictiveness(), 1.);
    }
    #[test]
    fn test_cocaine_viagra() {
        let mix = Sellable::from_product(Product::Cocaine).add_ingredient(Ingredient::Viagra);
        assert_eq!(mix.effects, HashSet::from([Effect::TropicThunder]),);
        // assert_eq!(mix.sell_price().round(), 219.);
        // assert_eq!(mix.addictiveness(), 1.);
    }
}
