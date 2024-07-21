use bevy::math::NormedVectorSpace;

pub trait StableInterpolate: Clone {
    fn interpolate_stable(&self, other: &Self, t: f32) -> Self;

    fn interpolate_stable_assign(&mut self, other: &Self, t: f32) {
        *self = self.interpolate_stable(other, t);
    }

    fn smooth_nudge(&mut self, target: &Self, decay_rate: f32, delta: f32) {
        self.interpolate_stable_assign(target, 1.0 - f32::exp(-decay_rate * delta));
    }
}

impl<V> StableInterpolate for V
where
    V: NormedVectorSpace,
{
    #[inline]
    fn interpolate_stable(&self, other: &Self, t: f32) -> Self {
        self.lerp(*other, t)
    }
}
