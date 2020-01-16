use crate::*;

pub trait Diff:
    Serialize + for<'de> Deserialize<'de> + Trans + Sync + Send + Clone + 'static
{
    type Delta: Serialize + for<'de> Deserialize<'de> + Trans + Sync + Send + Clone + 'static;
    fn diff(&self, to: &Self) -> Self::Delta;
    fn update(&mut self, delta: &Self::Delta);
}
