//! Experimental pipeline.

/// Experimental pipeline trait.
pub trait Pipeline<I> {
    /// The output type of the pipeline operation.
    type Output;

    /// Process some input with the pipeline.
    fn apply(&self, input: I) -> Self::Output;
}

impl<T: Fn(I) -> O, I, O> Pipeline<I> for T {
    type Output = O;

    fn apply(&self, input: I) -> Self::Output {
        self(input)
    }
}
