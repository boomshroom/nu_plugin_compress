use crate::CompressPlugin;
use bzip2::bufread::BzDecoder;
use nu_plugin::PluginCommand;
use nu_protocol::{
    ByteStream, ByteStreamType, Category, LabeledError, PipelineData, Signature, Type, Value,
};
use xz2::bufread::XzDecoder;

pub enum DecompressCommand {
    Gzip,
    Zstd,
    Xz,
    Bzip2,
}

impl PluginCommand for DecompressCommand {
    type Plugin = CompressPlugin;

    fn name(&self) -> &str {
        match self {
            DecompressCommand::Gzip => "from gz",
            DecompressCommand::Zstd => "from zst",
            DecompressCommand::Xz => "from xz",
            DecompressCommand::Bzip2 => "from bz2",
        }
    }

    fn signature(&self) -> Signature {
        Signature::build(PluginCommand::name(self))
            .allow_variants_without_examples(true)
            .input_output_types(vec![(Type::Binary, Type::Any)])
            .category(Category::Experimental)
            .filter()
    }

    fn description(&self) -> &str {
        match self {
            DecompressCommand::Gzip => "Decompress with gzip.",
            DecompressCommand::Zstd => "Decompress with zstd.",
            DecompressCommand::Xz => "Decompress with xz.",
            DecompressCommand::Bzip2 => "Decompress with bzip2.",
        }
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        engine: &nu_plugin::EngineInterface,
        call: &nu_plugin::EvaluatedCall,
        input: nu_protocol::PipelineData,
    ) -> Result<nu_protocol::PipelineData, nu_protocol::LabeledError> {
        let (byte_stream, span, pipeline_metadata) = match input {
            PipelineData::ByteStream(byte_stream, pipeline_metadata) => {
                let span = byte_stream.span();
                (byte_stream, span, pipeline_metadata)
            }
            PipelineData::Value(Value::Binary { val, internal_span }, pipeline_metadata) => {
                let byte_stream =
                    ByteStream::read_binary(val, internal_span, engine.signals().clone());

                (byte_stream, internal_span, pipeline_metadata)
            }
            v => {
                return Err(LabeledError::new(format!(
                    "requires binary input, got {}",
                    v.get_type()
                ))
                .with_label("Expected binary from pipeline", call.head))
            }
        };

        let reader = byte_stream.reader().ok_or_else(|| {
            LabeledError::new("Failed to get reader from byte stream")
                .with_label("Expected a valid reader", span)
        })?;

        let stream = match self {
            DecompressCommand::Gzip => ByteStream::read(
                flate2::read::MultiGzDecoder::new(reader),
                span,
                engine.signals().clone(),
                ByteStreamType::Binary,
            ),
            DecompressCommand::Zstd => ByteStream::read(
                zstd::stream::Decoder::new(reader).map_err(|e| LabeledError::new(e.to_string()))?,
                span,
                engine.signals().clone(),
                ByteStreamType::Binary,
            ),
            DecompressCommand::Xz => ByteStream::read(
                XzDecoder::new(reader),
                span,
                engine.signals().clone(),
                ByteStreamType::Binary,
            ),
            DecompressCommand::Bzip2 => ByteStream::read(
                BzDecoder::new(reader),
                span,
                engine.signals().clone(),
                ByteStreamType::Binary,
            ),
        };
        Ok(PipelineData::ByteStream(stream, pipeline_metadata))
    }
}
