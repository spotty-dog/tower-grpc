pub use generic::server::unary::Once;

use Body;
use codec::{Encode, Encoder, Decoder};
use generic::Streaming;
use generic::server::{UnaryService, unary};

use {http, prost};
use futures::{Future, Poll};

use std::fmt;

pub struct ResponseFuture<T, B, R>
where T: UnaryService<R>,
      R: prost::Message + Default,
      T::Response: prost::Message,
      B: Body,
{
    inner: Inner<T, T::Response, R, B>,
}

type Inner<T, U, V, B> =
    unary::ResponseFuture<T, Encoder<U>, Streaming<Decoder<V>, B>>;

impl<T, B, R> ResponseFuture<T, B, R>
where T: UnaryService<R>,
      R: prost::Message + Default,
      T::Response: prost::Message,
      B: Body,
{
    pub(crate) fn new(inner: Inner<T, T::Response, R, B>) -> Self {
        ResponseFuture { inner }
    }
}

impl<T, B, R> Future for ResponseFuture<T, B, R>
where T: UnaryService<R>,
      R: prost::Message + Default,
      T::Response: prost::Message,
      B: Body,
{
    type Item = http::Response<Encode<Once<T::Response>>>;
    type Error = ::error::Never;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let response = try_ready!(self.inner.poll());
        let response = response.map(Encode::new);
        Ok(response.into())
    }
}

impl<T, B, R> fmt::Debug for ResponseFuture<T, B, R>
where T: UnaryService<R> + fmt::Debug,
      R: prost::Message + Default + fmt::Debug,
      T::Response: prost::Message + fmt::Debug,
      T::Future: fmt::Debug,
      B: Body + fmt::Debug,
      B::Item: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("unary::ResponseFuture")
            .field("inner", &self.inner)
            .finish()
    }
}
