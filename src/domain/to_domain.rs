/// Convert vendor/API types to domain types.
pub trait ToDomain<T> {
    fn to_domain(&self) -> T;
}
