use std::marker::PhantomData;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_observer(apply_damage::<RatDamage>)
        .add_observer(apply_damage::<SpectralDamage>)
        .add_observer(apply_damage::<CheeseyDamage>);
}

#[derive(Component, Reflect, Debug, Default)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

#[derive(Component, Reflect, Debug, Default)]
pub struct FlatArmor<T>(pub f32, PhantomData<T>);

#[derive(Component, Reflect, Debug, Default)]
pub struct PctArmor<T>(pub f32, PhantomData<T>);

// Damage types
pub struct RatDamage;
pub struct SpectralDamage;
pub struct CheeseyDamage;

#[derive(Event, Reflect, Debug)]
pub struct DamageEvent<D> {
    pub to: Entity,
    pub amount: f32,
    pub marker: PhantomData<D>,
}

pub fn apply_damage<D: Send + Sync + 'static>(
    dmg: On<DamageEvent<D>>,
    mut to: Query<(&mut Health, Option<&FlatArmor<D>>, Option<&PctArmor<D>>)>,
) {
    let Ok((mut health, flat, pct)) = to.get_mut(dmg.to) else {
        return;
    };

    if dmg.amount > 0.0 {
        health.current += dmg.amount;
    } else {
        let mut amount = dmg.amount;
        if let Some(pct) = pct {
            amount = amount * (1.0 - pct.0).clamp(0.0, 1.0);
        }
        if let Some(flat) = flat {
            amount -= flat.0;
        }

        health.current -= amount.max(0.0);
    }

    health.current = health.current.clamp(0.0, health.max);
}
