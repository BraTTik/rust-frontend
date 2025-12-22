use crate::serializable::Serializer;

pub struct DebugSerializer {
    output: String,
}

impl DebugSerializer {
    pub fn new() -> Self {
        Self { output: String::new() }
    }
}

impl Serializer for DebugSerializer {
    type Output = String;
    type Error = ();

    fn visit_u32(&mut self, u: u32) -> Result<(), Self::Error> {
        self.output.push_str(&format!("{}, ", &u.to_string()));
        Ok(())
    }

    fn visit_string(&mut self, s: String) -> Result<(), Self::Error> {
        self.output.push_str(&format!("\"{}\", ", s));
        Ok(())
    }

    fn visit_bool(&mut self, b: bool) ->  Result<(), Self::Error> {
        self.output.push_str(&format!("{}, ", &b.to_string()));
        Ok(())
    }

    fn begin_struct(&mut self, name: &'static str) -> Result<(), Self::Error> {
        self.output.push_str(&format!("{} {{ ", name));
        Ok(())
    }

    fn visit_field(&mut self, name: &'static str) -> Result<(), Self::Error> {
        self.output.push_str(&format!("{}: ", name));
        Ok(())
    }

    fn end_struct(&mut self) -> Result<(), Self::Error> {
        self.output = self.output.trim().to_string();
        self.output.pop();
        self.output.push_str("}");
        Ok(())
    }

    fn finish(self) -> Result<Self::Output, Self::Error> {
        Ok(self.output)
    }
}