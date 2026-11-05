#![doc = include_str!("../README.md")]
#![no_std]

extern crate alloc;

pub mod action;
pub mod model;
#[cfg(feature = "reqwest")]
pub mod reqwest;

/// Base API URL of the official service.
pub const API_ROOT: &str = "https://monster-siren.hypergryph.com/api/";

/// Client that performs actions as a [Future].
pub trait ClientFuture {
    /// Type that represents a request failure.
    type Error;

    /// Perform a given action, returning its result.
    fn msr<'this, A: action::Action>(
        &'this self,
        action: A,
    ) -> impl Future<Output = Result<A::Data, Self::Error>> + 'this;
}

/// Client that performs actions as a [Send]able [Future].
pub trait ClientFutureSend {
    /// Type that represents a request failure.
    type Error;

    /// Perform a given action, returning its result.
    fn msr<'this, A: action::Action>(
        &'this self,
        action: A,
    ) -> impl Future<Output = Result<A::Data, Self::Error>> + Send + 'this;
}

impl<T: ClientFutureSend> ClientFuture for T {
    type Error = T::Error;

    fn msr<'this, A: action::Action>(
        &'this self,
        action: A,
    ) -> impl Future<Output = Result<A::Data, Self::Error>> + 'this {
        <Self as ClientFutureSend>::msr(self, action)
    }
}
