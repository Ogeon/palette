//! Module containing the errors that can be
//! produced by the library.

error_chain!{
    errors {
        /// Error representing an intensity sample being less than zero.
        SpectrumIntensityOutOfRange {
            description("Spectrum intensity value is out of range.")
            display("Spectrum intensity value must be >= 0.")
        }
    }
}