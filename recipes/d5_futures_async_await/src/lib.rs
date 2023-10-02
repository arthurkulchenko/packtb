use futures::stream::Stream;
// use futures::stream::StreamExt;
// use futures::io::{AsyncRead};
use tokio::io::AsyncRead;
use tokio::io::ReadBuf;
use futures::task::Context;
use futures::task::Poll;
use std::pin::Pin;

// pub mod simple;

// pub struct ReadStream<A: AsyncRead + Unpin> {
//     reader: A,
//     buffer: BytesMut,
// }

pub struct ReadStream<A: AsyncRead + Unpin> {
    reader: A,
    buffer: [u8; 100],
}

impl<A: AsyncRead + Unpin> ReadStream<A> {
    pub fn new(reader: A) -> Self {
        ReadStream { reader, buffer: [0; 100], }
    }
}

impl <A: AsyncRead + Unpin> Stream for ReadStream<A> {
    type Item = std::io::Result<String>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        let pinned_reader = Pin::new(&mut this.reader);

        let mut read_buf = ReadBuf::new(&mut this.buffer);
        // match pinned_reader.poll_read(cx, &mut this.buffer) {
        match pinned_reader.poll_read(cx, &mut read_buf) {
            Poll::Ready(Ok(num_bytes_read)) => {


                let num_bytes_read = read_buf.filled().len();
                let result = String::from_utf8_lossy(read_buf.filled()).to_string();
                Poll::Ready(Some(Ok(result)))


                // let result = String::from_utf8_lossy(&this.buffer[..num_bytes_read]).to_string();
                // Poll::Ready(Some(Ok(result)))
            }
            Poll::Ready(Err(e)) => Poll::Ready(Some(Err(e))),
            Poll::Pending => Poll::Pending,
        }
    }
}

// impl <A: AsyncRead + Unpin> ReadStream<A> {
//     type Item = String;

//     fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<<ReadStream<A> as IntoIterator>::Item>> {
//         let up = self.get_mut();
//         let r = Pin::new(&mut up.reader);
//         match r.poll_read(cx, &mut up.buffer) {
//             Poll::Ready(Ok(len)) => Poll::Ready(Some(String::from_utf8_lossy(&up.buffer[..len]).to_string())),
//             Poll::Ready(Err(_e)) => Poll::Ready(None),
//             Poll::Pending => Poll::Pending,
//         }
//     }
// }
