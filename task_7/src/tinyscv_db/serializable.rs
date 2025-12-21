pub trait Serializable {
    fn to_csv(&self) -> String;
    fn from_csv(csv: &str) -> Self;
}