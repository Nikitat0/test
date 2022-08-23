use derive_more::*;

#[derive(Debug, Index, IndexMut, IntoIterator)]
pub struct MotorsBunch<Motor>(Vec<Motor>);

impl<Motor> MotorsBunch<Motor> {
    pub fn new<I: IntoIterator<Item = Motor>>(iter: I) -> Self {
        iter.into_iter().collect()
    }

    pub fn exec<F, T, U>(&mut self, f: F) -> anyhow::Result<U>
    where
        F: FnMut(&mut Motor) -> anyhow::Result<T>,
        U: FromIterator<T>,
    {
        self.0.iter_mut().map(f).collect()
    }
}

impl<Motor> FromIterator<Motor> for MotorsBunch<Motor> {
    fn from_iter<T: IntoIterator<Item = Motor>>(iter: T) -> Self {
        MotorsBunch(iter.into_iter().collect())
    }
}
