use std::io::Cursor;

use crate::{frame, Frame, Result};
use bytes::{Buf, BytesMut};
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

#[derive(Debug)]
pub struct Connection {
    stream: TcpStream,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            stream,
            buffer: BytesMut::with_capacity(4096),
        }
    }

    /// Read a frame from the connection.
    ///
    /// Returns `None` if EOF is reached.
    pub async fn read_frame(&mut self) -> Result<Option<Frame>> {
        loop {
            // Attempt to parse a frame from the buffered data. If enough data
            // has been buffered, the frame is returned.
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }

            // There is not enough buffered data to read a frame. Attempt to
            // read more data from the socket.
            //
            // On success, the number of bytes is returned. `0` indicates
            // "end of stream".
            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                // The remote closed the connection. For this to be a clean
                // shutdown, there should be no data in the read buffer. If
                // there is, this means that the peer closed the socket while
                // sending a frame.
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err("connection reset by peer".into());
                }
            }
        }
    }

    /// Write a frame to the connection
    pub async fn write_frame(&mut self, _frame: &Frame) -> Result<()> {
        Ok(())
    }

    fn parse_frame(&mut self) -> crate::Result<Option<Frame>> {
        let mut buf = Cursor::new(&self.buffer[..]);

        match Frame::check(&mut buf) {
            Ok(_) => {
                // Get byte length of the frame.
                let len = buf.position() as usize;

                // Reset the internal cursor for the call to `parse`.
                buf.set_position(0);

                // Parse the frame.
                let frame = Frame::parse(&mut buf)?;

                // Discard the frame from the buffer.
                self.buffer.advance(len);

                Ok(Some(frame))
            }
            Err(frame::Error::Incomplete) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}
