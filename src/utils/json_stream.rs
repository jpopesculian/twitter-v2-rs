use crate::{Error, Result};
use futures::prelude::*;
use pin_project_lite::pin_project;
use serde::de::DeserializeOwned;
use serde_json::de::{SliceRead, StreamDeserializer};
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

pin_project! {
#[project = JsonStreamProjection]
pub struct JsonStream<S, T> {
    #[pin]
    stream: S,
    buffer: Vec<u8>,
    item_ty: PhantomData<T>
}
}

impl<S, T> JsonStream<S, T> {
    pub fn new(stream: S) -> Self {
        Self {
            stream,
            buffer: Default::default(),
            item_ty: Default::default(),
        }
    }
}

impl<'a, S, T> JsonStreamProjection<'a, S, T>
where
    T: DeserializeOwned,
{
    pub fn next_item(&mut self) -> Option<Result<T>> {
        let mut deserializer =
            StreamDeserializer::<_, T>::new(SliceRead::new(self.buffer.as_ref()));
        match deserializer.next() {
            Some(Ok(item)) => {
                let read = deserializer.byte_offset();
                self.buffer.rotate_left(read);
                unsafe {
                    self.buffer.set_len(self.buffer.len() - read);
                }
                Some(Ok(item))
            }
            Some(Err(err)) => {
                if err.is_eof() {
                    None
                } else {
                    Some(Err(err.into()))
                }
            }
            None => None,
        }
    }
}

impl<S, B, E, T> Stream for JsonStream<S, T>
where
    S: Stream<Item = Result<B, E>>,
    B: AsRef<[u8]>,
    E: Into<Error>,
    T: DeserializeOwned,
{
    type Item = Result<T>;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();
        if let Some(item) = this.next_item() {
            return Poll::Ready(Some(item));
        }
        while let Poll::Ready(chunk) = this.stream.as_mut().poll_next(cx) {
            match chunk {
                Some(Ok(chunk)) => {
                    this.buffer.extend_from_slice(chunk.as_ref());
                    if let Some(item) = this.next_item() {
                        return Poll::Ready(Some(item));
                    }
                }
                Some(Err(err)) => return Poll::Ready(Some(Err(err.into()))),
                None => return Poll::Ready(None),
            }
        }
        Poll::Pending
    }
}
