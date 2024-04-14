use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{LabeledError, PipelineData, Signature, Type, Value};
use qr2term::{qr, render};
use qr2term::render::Renderer;
use crate::QrPlugin;

pub struct ToQr ;
impl ToQr {
    pub fn new() -> ToQr {
        ToQr {}
    }
    fn generate_qr(val: String) -> Result<String, qr2term::QrError> {
        let mut matrix = qr::Qr::from(val)?.to_matrix();
        // Padding and theme
        matrix.surround(2, render::QrLight);

        // Render QR code to a String
        let mut buf = Vec::new();
        Renderer::default()
            .render(&matrix, &mut buf)
            .expect("failed to generate QR code string");
        Ok(String::from_utf8(buf).unwrap())
    }
}
impl PluginCommand for ToQr {
    type Plugin = QrPlugin;

    fn name(&self) -> &str {
        "to qr"
    }

    fn signature(&self) -> Signature {
        Signature::build("to qr",)
            .input_output_type(Type::String, Type::String)
    }

    fn usage(&self) -> &str {
        "create qr code from given text. (to convert records into qr you must use `to json` before using `to qr`)"
    }

    fn run(&self, _plugin: &Self::Plugin, _engine: &EngineInterface, call: &EvaluatedCall, input: PipelineData) -> Result<PipelineData, LabeledError> {
        match input {
           PipelineData::Value(Value::String { val, .. }, ..) => return match Self::generate_qr(val) {
               Ok(res) => Ok(PipelineData::Value(Value::string(res, call.head), None)),
               Err(qr_err) => {
                   Err(LabeledError::new(format!("error when tried to create qr code: {}", qr_err)))
               }
           },
            _ => {}
        }
        return Err(LabeledError::new("please provide string in input of you want to export structured data convert them to json before converting to qr code"));

    }
}