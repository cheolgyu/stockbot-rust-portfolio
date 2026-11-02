#[derive(Debug)]
pub struct ItemReaderError {
    pub source: String,
}
#[derive(Debug)]
pub struct ItemWriterError {
    pub source: String,
}
#[derive(Debug)]
pub struct ItemProcessorError {
    pub source: String,
}
#[derive(Debug)]
pub struct StepError {
    pub source: String,
}

impl From<ItemReaderError> for StepError {
    fn from(value: ItemReaderError) -> Self {
        Self {
            source: value.source,
        }
    }
}

impl From<ItemWriterError> for StepError {
    fn from(value: ItemWriterError) -> Self {
        Self {
            source: value.source,
        }
    }
}

impl From<ItemProcessorError> for StepError {
    fn from(value: ItemProcessorError) -> Self {
        Self {
            source: value.source,
        }
    }
}
#[derive(Debug)]
pub struct JobError {
    pub source: String,
}
impl From<StepError> for JobError {
    fn from(value: StepError) -> Self {
        Self {
            source: value.source,
        }
    }
}
