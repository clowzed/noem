use crate::adapter::Adapter;

pub struct Chain<A, B> {
    first: A,
    second: B,
}

impl<A, B> Chain<A, B> {
    pub fn new(first: A, second: B) -> Self {
        Chain { first, second }
    }
}

impl<A, B> Adapter for Chain<A, B>
where
    A: Adapter,
    B: Adapter<Input = A::Output>,
    A::Error: 'static,
    B::Error: 'static,
    <A as Adapter>::Error: std::error::Error,
    <B as Adapter>::Error: std::error::Error,
{
    type Input = A::Input;
    type Output = B::Output;
    type Error = Box<dyn std::error::Error>;

    fn convert(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        let intermediate = self
            .first
            .convert(input)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        self.second
            .convert(intermediate)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }
}
