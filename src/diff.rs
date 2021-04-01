use super::*;

pub trait Diff:
    Debug + Serialize + for<'de> Deserialize<'de> + Sync + Send + Clone + PartialEq + 'static
{
    type Delta: Debug + Serialize + for<'de> Deserialize<'de> + Sync + Send + Clone + 'static;
    fn diff(&self, to: &Self) -> Self::Delta;
    fn update(&mut self, delta: &Self::Delta);
}

impl<
        T: Debug + Serialize + for<'de> Deserialize<'de> + Sync + Send + Copy + PartialEq + 'static,
    > Diff for T
{
    type Delta = Self;
    fn diff(&self, to: &Self) -> Self {
        *to
    }
    fn update(&mut self, new_value: &Self) {
        *self = *new_value;
    }
}
