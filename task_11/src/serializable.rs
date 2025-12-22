pub trait Serializer {
    type Output;
    type Error;

    fn visit_u32(&mut self, u: u32) -> Result<(), Self::Error>;
    fn visit_string(&mut self, s: String) -> Result<(), Self::Error>;
    fn visit_bool(&mut self, b: bool) ->  Result<(), Self::Error>;

    fn begin_struct(&mut self, name: &'static str) -> Result<(), Self::Error>;
    fn visit_field(&mut self, name: &'static str) -> Result<(), Self::Error>;
    fn end_struct(&mut self) -> Result<(), Self::Error>;

    fn finish(self) -> Result<Self::Output, Self::Error>;
}

pub trait Serializable {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Output, S::Error>;
}