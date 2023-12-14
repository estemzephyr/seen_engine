use crate::L_Presentation::stream_module::stream::IStream;
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct stream_service {
    pub(crate) stream: IStream,
}

impl stream_service {
    pub async fn create_new_stream() -> stream_service {
        // Create a new sender
        let (sender, receiver) = mpsc::unbounded_channel();

        // Create an instance of IStream with the correct sender
        let i_stream = IStream { sender, receiver };

        stream_service {
            stream: i_stream,
        }
    }
}
