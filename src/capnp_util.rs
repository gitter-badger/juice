//! Provides functionality for Cap'n Proto (de)serialization.

pub trait CapnpWrite<'a> {
    /// The Builder that was autogenerated by capnp.
    type Builder;

    /// Write the struct into the message that is being built by the Builder.
    fn write_capnp(&self, builder: &mut Self::Builder);
}
