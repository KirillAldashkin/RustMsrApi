//! [Reqwest](reqwest)-based client implementation.

use alloc::borrow::ToOwned;

use crate::{API_ROOT, ClientFutureSend, action::Action, model};

/// Error when processing a request.
#[derive(Debug)]
pub enum Error {
    /// Request error.
    Reqwest(reqwest::Error),
    /// API error.
    Api(model::Error),
}

impl ClientFutureSend for reqwest::Client {
    type Error = Error;

    fn msr<'this, A: Action>(
        &'this self,
        action: A,
    ) -> impl Future<Output = Result<A::Data, Self::Error>> + Send + 'this {
        let mut url = API_ROOT.to_owned();
        action.path(&mut url).expect("Couldn't format URL");

        async move {
            let data = self
                .get(url)
                .send()
                .await
                .map_err(Error::Reqwest)?
                .error_for_status()
                .map_err(Error::Reqwest)?
                .json::<model::Response<A::Data>>()
                .await
                .map_err(Error::Reqwest)?;
            Result::from(data).map_err(Error::Api)
        }
    }
}
