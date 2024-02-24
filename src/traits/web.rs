pub trait Send {}
impl<T> Send for T {}

pub trait Sync {}
impl<T> Sync for T {}
