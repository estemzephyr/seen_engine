use std::pin::Pin;
use std::task::{Context, Poll};
use futures_util::stream::Stream;
use tokio::sync::mpsc;
use crate::L_Data::IDataObj::IData::IData;

#[derive(Debug)]
pub struct IStream {
    pub sender:mpsc::UnboundedSender<IData>,
    pub(crate) receiver: mpsc::UnboundedReceiver<IData>,
}

impl IStream {
    pub fn new() -> IStream {
        let (sender, receiver) = mpsc::unbounded_channel();
        let i_stream = IStream { sender, receiver };
        (i_stream)
    }
    pub fn send_data(&mut self, data: IData) {
        // Veriyi gönder
        self.sender.send(data).expect("Stream value isn't send");
    }
}
    impl Stream for IStream {
        type Item = IData;

        fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            // Poll the underlying receiver for new data
            match Pin::new(&mut self.receiver).poll_recv(cx) {
                Poll::Ready(Some(value)) => Poll::Ready(Some(value)),
                Poll::Ready(None) => Poll::Ready(None),
                Poll::Pending => Poll::Pending,
            }
        }
    }

#[cfg(test)]
mod tests {
    use futures_util::stream::StreamExt;
    use tokio::time::timeout;
    use crate::L_Data::IDataObj::IData::IData;
    use super::IStream;
    #[tokio::test]
    async fn test_i_stream() {
        // Create a new IStream
        let mut stream = IStream::new();
        let ex_data = IData {
            id: 123,
            name: "Car".to_string(),
            value: "Mustang".to_string(),
        };

        // Use the send_data method to send some data
        stream.send_data(ex_data.clone());

        // Poll for the next item in the stream with a timeout
        let item = timeout(std::time::Duration::from_secs(5), stream.next()).await;

        // Assert that the item is Some and contains the expected value
            assert!(item.is_ok());
            let received_data = item.unwrap().unwrap();
            assert_eq!(received_data, ex_data);

            // Print the received and expected data
            println!("Received Data: {:?}", received_data);
            println!("Expected Data: {:?}", ex_data);
    }
}
