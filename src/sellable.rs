use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Sellable {
    pub base: Product,
    pub effects: HashSet<Effect>,
    pub ingredients: Vec<Ingredient>,
}

impl Sellable {
    pub fn sell_price(&self) -> f32 {
        let multiplier_sum = self
            .effects
            .iter()
            .map(|effect| effect.multiplier())
            .sum::<f32>();
        self.base.sell_price() * (1. + multiplier_sum)
    }
    pub fn from_product(product: Product) -> Self {
        let effects = product.effects();
        Sellable {
            base: product,
            effects,
            ingredients: Vec::new(),
        }
    }
    pub fn add_ingredient(&self, ingredient: Ingredient) -> Self {
        let (removals, additions) = ingredient.effect_changes(&self.effects);
        let mut effects = self.effects.clone();
        for effect in removals {
            effects.remove(&effect);
        }
        for effect in additions {
            effects.insert(effect);
        }
        let mut ingredients = self.ingredients.clone();
        ingredients.push(ingredient);
        Sellable {
            base: self.base,
            ingredients,
            effects,
        }
    }
    pub fn price(&self) -> f32 {
        let mut price = self.base.price();
        for ingredient in &self.ingredients {
            price += ingredient.price();
        }
        price
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

    pub fn effect_changes(&self, effects: &HashSet<Effect>) -> (HashSet<Effect>, HashSet<Effect>) {
        let mut removals = HashSet::new();
        let mut additions = HashSet::new();
        match self {
            Ingredient::Cuke => {
                additions.insert(Effect::Energizing);
                if effects.contains(&Effect::Munchies) {
                    removals.insert(Effect::Munchies);
                    additions.insert(Effect::Athletic);
                }
                if effects.contains(&Effect::Slippery) && effects.contains(&Effect::Munchies) {
                    removals.insert(Effect::Slippery);
                    additions.insert(Effect::Athletic);
                }
                if effects.contains(&Effect::Foggy) {
                    removals.insert(Effect::Foggy);
                    additions.insert(Effect::Cyclopean);
                }
                if effects.contains(&Effect::Toxic) {
                    removals.insert(Effect::Toxic);
                    additions.insert(Effect::Euphoric);
                }
                if effects.contains(&Effect::Euphoric) {
                    removals.insert(Effect::Euphoric);
                    additions.insert(Effect::Laxative);
                }
                if effects.contains(&Effect::Slippery) {
                    removals.insert(Effect::Slippery);
                    additions.insert(Effect::Munchies);
                }
                if effects.contains(&Effect::Sneaky) {
                    removals.insert(Effect::Sneaky);
                    additions.insert(Effect::Paranoia);
                }
                if effects.contains(&Effect::Gingeritis) {
                    removals.insert(Effect::Gingeritis);
                    additions.insert(Effect::ThoughtProvoking);
                }
            }
            Ingredient::Banana => {
                additions.insert(Effect::Gingeritis);
                if effects.contains(&Effect::Smelly) {
                    removals.insert(Effect::Smelly);
                    additions.insert(Effect::AntiGravity);
                }
                if effects.contains(&Effect::Disorienting) {
                    removals.insert(Effect::Disorienting);
                    additions.insert(Effect::Focused);
                }
                if effects.contains(&Effect::Paranoia) {
                    removals.insert(Effect::Paranoia);
                    additions.insert(Effect::Jennerising);
                }
                if effects.contains(&Effect::LongFaced) {
                    removals.insert(Effect::LongFaced);
                    additions.insert(Effect::Refreshing);
                }
                if effects.contains(&Effect::Focused) {
                    removals.insert(Effect::Focused);
                    additions.insert(Effect::SeizureInducing);
                }
                if effects.contains(&Effect::Toxic) {
                    removals.insert(Effect::Toxic);
                    additions.insert(Effect::Smelly);
                }
                if effects.contains(&Effect::Calming) {
                    removals.insert(Effect::Calming);
                    additions.insert(Effect::Sneaky);
                }
                if effects.contains(&Effect::Cyclopean) {
                    removals.insert(Effect::Cyclopean);
                    additions.insert(Effect::ThoughtProvoking);
                }
                if effects.contains(&Effect::Energizing) && !effects.contains(&Effect::Cyclopean) {
                    removals.insert(Effect::Energizing);
                    additions.insert(Effect::ThoughtProvoking);
                }
            }
            Ingredient::Paracetamol => {
                additions.insert(Effect::Sneaky);
                if effects.contains(&Effect::Munchies) {
                    removals.insert(Effect::Munchies);
                    additions.insert(Effect::AntiGravity);
                }
                if effects.contains(&Effect::Electrifying) {
                    removals.insert(Effect::Electrifying);
                    additions.insert(Effect::Athletic);
                }
                if effects.contains(&Effect::Paranoia) {
                    removals.insert(Effect::Paranoia);
                    additions.insert(Effect::Balding);
                }
                if effects.contains(&Effect::Energizing) && !effects.contains(&Effect::Paranoia) {
                    removals.insert(Effect::Energizing);
                    additions.insert(Effect::Balding);
                }
                if effects.contains(&Effect::Spicy) {
                    removals.insert(Effect::Spicy);
                    additions.insert(Effect::BrightEyed);
                }
                if effects.contains(&Effect::Foggy) {
                    removals.insert(Effect::Foggy);
                    additions.insert(Effect::Calming);
                }
                if effects.contains(&Effect::Focused) {
                    removals.insert(Effect::Focused);
                    additions.insert(Effect::Gingeritis);
                }
                if effects.contains(&Effect::Energizing) && !effects.contains(&Effect::Munchies) {
                    removals.insert(Effect::Energizing);
                    additions.insert(Effect::Paranoia);
                }
                if effects.contains(&Effect::Calming) {
                    removals.insert(Effect::Calming);
                    additions.insert(Effect::Slippery);
                }
                if effects.contains(&Effect::Glowing) {
                    removals.insert(Effect::Glowing);
                    additions.insert(Effect::Toxic);
                }
                if effects.contains(&Effect::Toxic) {
                    removals.insert(Effect::Toxic);
                    additions.insert(Effect::TropicThunder);
                }
            }
            Ingredient::Donut => {
                additions.insert(Effect::CalorieDense);
                if effects.contains(&Effect::Shrinking) {
                    removals.insert(Effect::Shrinking);
                    additions.insert(Effect::Energizing);
                }
                if effects.contains(&Effect::Focused) {
                    removals.insert(Effect::Focused);
                    additions.insert(Effect::Euphoric);
                }
                if effects.contains(&Effect::CalorieDense) && !effects.contains(&Effect::Explosive) {
                    removals.insert(Effect::CalorieDense);
                    additions.insert(Effect::Explosive);
                }
                if effects.contains(&Effect::Jennerising) {
                    removals.insert(Effect::Jennerising);
                    additions.insert(Effect::Gingeritis);
                }
                if effects.contains(&Effect::AntiGravity) {
                    removals.insert(Effect::AntiGravity);
                    additions.insert(Effect::Slippery);
                }
                if effects.contains(&Effect::Balding) {
                    removals.insert(Effect::Balding);
                    additions.insert(Effect::Sneaky);
                }
            }
            Ingredient::Viagra => {
                additions.insert(Effect::TropicThunder);
                if effects.contains(&Effect::Euphoric) {
                    removals.insert(Effect::Euphoric);
                    additions.insert(Effect::BrightEyed);
                }
                if effects.contains(&Effect::Laxative) {
                    removals.insert(Effect::Laxative);
                    additions.insert(Effect::Calming);
                }
                if effects.contains(&Effect::Athletic) {
                    removals.insert(Effect::Athletic);
                    additions.insert(Effect::Sneaky);
                }
                if effects.contains(&Effect::Disorienting) {
                    removals.insert(Effect::Disorienting);
                    additions.insert(Effect::Toxic);
                }
            }
            Ingredient::MouthWash => {
                additions.insert(Effect::Balding);
                if effects.contains(&Effect::Calming) {
                    removals.insert(Effect::Calming);
                    additions.insert(Effect::AntiGravity);
                }
                if effects.contains(&Effect::Focused) {
                    removals.insert(Effect::Focused);
                    additions.insert(Effect::Jennerising);
                }
                if effects.contains(&Effect::Explosive) {
                    removals.insert(Effect::Explosive);
                    additions.insert(Effect::Sedating);
                }
                if effects.contains(&Effect::CalorieDense) {
                    removals.insert(Effect::CalorieDense);
                    additions.insert(Effect::Sneaky);
                }
            }
            Ingredient::FluMedicine => {
                additions.insert(Effect::Sedating);
                if effects.contains(&Effect::Calming) {
                    removals.insert(Effect::Calming);
                    additions.insert(Effect::BrightEyed);
                }
                if effects.contains(&Effect::Focused) {
                    removals.insert(Effect::Focused);
                    additions.insert(Effect::Calming);
                }
                if effects.contains(&Effect::Laxative) {
                    removals.insert(Effect::Laxative);
                    additions.insert(Effect::Euphoric);
                }
                if effects.contains(&Effect::Cyclopean) {
                    removals.insert(Effect::Cyclopean);
                    additions.insert(Effect::Foggy);
                }
                if effects.contains(&Effect::ThoughtProvoking) {
                    removals.insert(Effect::ThoughtProvoking);
                    additions.insert(Effect::Gingeritis);
                }
                if effects.contains(&Effect::Athletic) {
                    removals.insert(Effect::Athletic);
                    additions.insert(Effect::Munchies);
                }
                if effects.contains(&Effect::Shrinking) {
                    removals.insert(Effect::Shrinking);
                    additions.insert(Effect::Paranoia);
                }
                if effects.contains(&Effect::Electrifying) {
                    removals.insert(Effect::Electrifying);
                    additions.insert(Effect::Refreshing);
                }
                if effects.contains(&Effect::Munchies) {
                    removals.insert(Effect::Munchies);
                    additions.insert(Effect::Slippery);
                }
                if effects.contains(&Effect::Euphoric) {
                    removals.insert(Effect::Euphoric);
                    additions.insert(Effect::Toxic);
                }
            }
            Ingredient::Gasoline => {
                additions.insert(Effect::Toxic);
                if effects.contains(&Effect::Paranoia) {
                    removals.insert(Effect::Paranoia);
                    additions.insert(Effect::Calming);
                }
                if effects.contains(&Effect::Electrifying) {
                    removals.insert(Effect::Electrifying);
                    additions.insert(Effect::Disorienting);
                }
                if effects.contains(&Effect::Energizing) {
                    removals.insert(Effect::Energizing);
                    additions.insert(Effect::Euphoric);
                }
                if effects.contains(&Effect::Shrinking) {
                    removals.insert(Effect::Shrinking);
                    additions.insert(Effect::Focused);
                }
                if effects.contains(&Effect::Laxative) {
                    removals.insert(Effect::Laxative);
                    additions.insert(Effect::Foggy);
                }
                if effects.contains(&Effect::Disorienting) {
                    removals.insert(Effect::Disorienting);
                    additions.insert(Effect::Glowing);
                }
                if effects.contains(&Effect::Munchies) {
                    removals.insert(Effect::Munchies);
                    additions.insert(Effect::Sedating);
                }
                if effects.contains(&Effect::Gingeritis) {
                    removals.insert(Effect::Gingeritis);
                    additions.insert(Effect::Smelly);
                }
                if effects.contains(&Effect::Jennerising) {
                    removals.insert(Effect::Jennerising);
                    additions.insert(Effect::Sneaky);
                }
                if effects.contains(&Effect::Energizing) {
                    removals.insert(Effect::Energizing);
                    additions.insert(Effect::Spicy);
                }
                if effects.contains(&Effect::Euphoric) && !effects.contains(&Effect::Energizing) {
                    removals.insert(Effect::Euphoric);
                    additions.insert(Effect::Spicy);
                }
                if effects.contains(&Effect::Sneaky) {
                    removals.insert(Effect::Sneaky);
                    additions.insert(Effect::TropicThunder);
                }
            }
            Ingredient::EnergyDrink => {
                additions.insert(Effect::Athletic);
                if effects.contains(&Effect::Schizophrenia) {
                    removals.insert(Effect::Schizophrenia);
                    additions.insert(Effect::Balding);
                }
                if effects.contains(&Effect::Glowing) {
                    removals.insert(Effect::Glowing);
                    additions.insert(Effect::Disorienting);
                }
                if effects.contains(&Effect::Disorienting) {
                    removals.insert(Effect::Disorienting);
                    additions.insert(Effect::Electrifying);
                }
                if effects.contains(&Effect::Euphoric) {
                    removals.insert(Effect::Euphoric);
                    additions.insert(Effect::Energizing);
                }
                if effects.contains(&Effect::Spicy) {
                    removals.insert(Effect::Spicy);
                    additions.insert(Effect::Euphoric);
                }
                if effects.contains(&Effect::Foggy) {
                    removals.insert(Effect::Foggy);
                    additions.insert(Effect::Laxative);
                }
                if effects.contains(&Effect::Sedating) {
                    removals.insert(Effect::Sedating);
                    additions.insert(Effect::Munchies);
                }
                if effects.contains(&Effect::Focused) {
                    removals.insert(Effect::Focused);
                    additions.insert(Effect::Shrinking);
                }
                if effects.contains(&Effect::TropicThunder) {
                    removals.insert(Effect::TropicThunder);
                    additions.insert(Effect::Sneaky);
                }
            }
            Ingredient::MotorOil => {
                additions.insert(Effect::Slippery);
                if effects.contains(&Effect::Paranoia) {
                    removals.insert(Effect::Paranoia);
                    additions.insert(Effect::AntiGravity);
                }
                if effects.contains(&Effect::Energizing) {
                    removals.insert(Effect::Energizing);
                    additions.insert(Effect::Munchies);
                }
                if effects.contains(&Effect::Energizing) {
                    removals.insert(Effect::Energizing);
                    additions.insert(Effect::Schizophrenia);
                }
                if effects.contains(&Effect::Munchies) && !effects.contains(&Effect::Energizing) {
                    removals.insert(Effect::Munchies);
                    additions.insert(Effect::Schizophrenia);
                }
                if effects.contains(&Effect::Euphoric) {
                    removals.insert(Effect::Euphoric);
                    additions.insert(Effect::Sedating);
                }
                if effects.contains(&Effect::Foggy) {
                    removals.insert(Effect::Foggy);
                    additions.insert(Effect::Toxic);
                }
            }
            Ingredient::MegaBean => {
                additions.insert(Effect::Foggy);
                if effects.contains(&Effect::Sneaky) {
                    removals.insert(Effect::Sneaky);
                    additions.insert(Effect::Calming);
                }
                if effects.contains(&Effect::ThoughtProvoking) {
                    removals.insert(Effect::ThoughtProvoking);
                    additions.insert(Effect::Cyclopean);
                }
                if effects.contains(&Effect::Energizing) && !effects.contains(&Effect::ThoughtProvoking) {
                    removals.insert(Effect::Energizing);
                    additions.insert(Effect::Cyclopean);
                }
                if effects.contains(&Effect::Focused) {
                    removals.insert(Effect::Focused);
                    additions.insert(Effect::Disorienting);
                }
                if effects.contains(&Effect::Shrinking) {
                    removals.insert(Effect::Shrinking);
                    additions.insert(Effect::Electrifying);
                }
                if effects.contains(&Effect::ThoughtProvoking) {
                    removals.insert(Effect::ThoughtProvoking);
                    additions.insert(Effect::Energizing);
                }
                if effects.contains(&Effect::SeizureInducing) {
                    removals.insert(Effect::SeizureInducing);
                    additions.insert(Effect::Focused);
                }
                if effects.contains(&Effect::Calming) {
                    removals.insert(Effect::Calming);
                    additions.insert(Effect::Glowing);
                }
                if effects.contains(&Effect::Sneaky) {
                    removals.insert(Effect::Sneaky);
                    additions.insert(Effect::Glowing);
                }
                if effects.contains(&Effect::Athletic) {
                    removals.insert(Effect::Athletic);
                    additions.insert(Effect::Laxative);
                }
                if effects.contains(&Effect::Jennerising) {
                    removals.insert(Effect::Jennerising);
                    additions.insert(Effect::Paranoia);
                }
                if effects.contains(&Effect::Slippery) {
                    removals.insert(Effect::Slippery);
                    additions.insert(Effect::Toxic);
                }
            }
            Ingredient::Chili => {
                additions.insert(Effect::Spicy);
                if effects.contains(&Effect::Sneaky) {
                    removals.insert(Effect::Sneaky);
                    additions.insert(Effect::BrightEyed);
                }
                if effects.contains(&Effect::Athletic) {
                    removals.insert(Effect::Athletic);
                    additions.insert(Effect::Euphoric);
                }
                if effects.contains(&Effect::Laxative) {
                    removals.insert(Effect::Laxative);
                    additions.insert(Effect::LongFaced);
                }
                if effects.contains(&Effect::Shrinking) {
                    removals.insert(Effect::Shrinking);
                    additions.insert(Effect::Refreshing);
                }
                if effects.contains(&Effect::Munchies) {
                    removals.insert(Effect::Munchies);
                    additions.insert(Effect::Toxic);
                }
                if effects.contains(&Effect::AntiGravity) {
                    removals.insert(Effect::AntiGravity);
                    additions.insert(Effect::TropicThunder);
                }
            }
            Ingredient::Battery => {
                additions.insert(Effect::BrightEyed);
                if effects.contains(&Effect::Laxative) {
                    removals.insert(Effect::Laxative);
                    additions.insert(Effect::CalorieDense);
                }
                if effects.contains(&Effect::Electrifying) && !effects.contains(&Effect::Zombifying) {
                    removals.insert(Effect::Electrifying);
                    additions.insert(Effect::Euphoric);
                }
                if effects.contains(&Effect::Cyclopean) {
                    removals.insert(Effect::Cyclopean);
                    additions.insert(Effect::Glowing);
                }
                if effects.contains(&Effect::Shrinking) {
                    removals.insert(Effect::Shrinking);
                    additions.insert(Effect::Munchies);
                }
                if effects.contains(&Effect::Munchies) {
                    removals.insert(Effect::Munchies);
                    additions.insert(Effect::TropicThunder);
                }
                if effects.contains(&Effect::Euphoric) && !effects.contains(&Effect::Electrifying) {
                    removals.insert(Effect::Euphoric);
                    additions.insert(Effect::Zombifying);
                }
            }
            Ingredient::Iodine => {
                additions.insert(Effect::Jennerising);
                if effects.contains(&Effect::CalorieDense) {
                    removals.insert(Effect::CalorieDense);
                    additions.insert(Effect::Gingeritis);
                }
                if effects.contains(&Effect::Foggy) {
                    removals.insert(Effect::Foggy);
                    additions.insert(Effect::Paranoia);
                }
                if effects.contains(&Effect::Calming) {
                    removals.insert(Effect::Calming);
                    additions.insert(Effect::Sedating);
                }
                if effects.contains(&Effect::Euphoric) {
                    removals.insert(Effect::Euphoric);
                    additions.insert(Effect::SeizureInducing);
                }
                if effects.contains(&Effect::Toxic) {
                    removals.insert(Effect::Toxic);
                    additions.insert(Effect::Sneaky);
                }
                if effects.contains(&Effect::Refreshing) {
                    removals.insert(Effect::Refreshing);
                    additions.insert(Effect::ThoughtProvoking);
                }
            }
            Ingredient::Addy => {
                additions.insert(Effect::ThoughtProvoking);
                if effects.contains(&Effect::LongFaced) {
                    removals.insert(Effect::LongFaced);
                    additions.insert(Effect::Electrifying);
                }
                if effects.contains(&Effect::Foggy) {
                    removals.insert(Effect::Foggy);
                    additions.insert(Effect::Energizing);
                }
                if effects.contains(&Effect::Explosive) {
                    removals.insert(Effect::Explosive);
                    additions.insert(Effect::Euphoric);
                }
                if effects.contains(&Effect::Sedating) {
                    removals.insert(Effect::Sedating);
                    additions.insert(Effect::Gingeritis);
                }
                if effects.contains(&Effect::Glowing) {
                    removals.insert(Effect::Glowing);
                    additions.insert(Effect::Refreshing);
                }
            }
            Ingredient::HorseSemen => {
                additions.insert(Effect::LongFaced);
                if effects.contains(&Effect::AntiGravity) {
                    removals.insert(Effect::AntiGravity);
                    additions.insert(Effect::Calming);
                }
                if effects.contains(&Effect::ThoughtProvoking) {
                    removals.insert(Effect::ThoughtProvoking);
                    additions.insert(Effect::Electrifying);
                }
                if effects.contains(&Effect::Gingeritis) {
                    removals.insert(Effect::Gingeritis);
                    additions.insert(Effect::Refreshing);
                }
            }
        }
        (removals, additions)
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
