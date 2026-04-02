use bevy::prelude::*;

pub fn plugin(app: &mut App) {}

#[derive(Component, Reflect, Debug, Default)]
pub struct Health(pub f32);

#[derive(Component, Reflect, Debug, Default)]
pub struct FlatArmor<T>(pub f32, PhantomData<T>);

#[derive(Component, Reflect, Debug, Default)]
pub struct PctArmor<T>(pub f32, PhantomData<T>);

// Damage types
pub struct RatDamage;
pub struct SpectralDamage;
pub struct CheeseyDamage;

#[derive(Event, Reflect, Debug, Default)]
pub struct DamageEvent<D> {
    pub to: Entity,
    pub amount: f32,
}

pub fn apply_damage<D>(
    dmg: On<DamageEvent<D>>,
    to: Query<Option<&mut Health>, Option<&FlatArmor<D>>, Option<&PctArmor<D>>>,
) {
}
