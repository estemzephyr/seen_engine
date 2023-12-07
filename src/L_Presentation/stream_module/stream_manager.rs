use crate::L_Presentation::stream_module::stream::IStream;
#[derive(Debug)]
pub struct stream_service{
    stream:IStream
}

impl stream_service{
    pub async fn create_new_stream() -> stream_service{
        stream_service{
            stream: IStream {},
        }
    }
}