use nu_plugin::{self, EvaluatedCall, LabeledError};
use nu_protocol::{ Category, PluginExample, PluginSignature,  Type, Value};
use qr2term::{qr, render::{self, Renderer}};

pub struct Plugin;

impl nu_plugin::Plugin for Plugin {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![PluginSignature::build("to qr")
            .usage("create qr code from given text. (to convert records into qr you must use `to json` before using `to qr`)")
            .input_output_type(Type::String, Type::String)
            .plugin_examples(
                vec![
                    PluginExample{
                        example: "\"https://google.com\" | to qr".to_string(),
                        description : "to create qr code just use `to qr`".to_string(),
                        result: None,
                    },
                    
                ],)
            .category(Category::Conversions)]
    }

    fn run(
        &mut self,
        _name: &str,
        _config: &Option<Value>,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        match input {
            Value::String { val, .. } => match generate_qr(val) {
                Ok(res) => return Ok(Value::string(res, call.head)),
                Err(qr_err) => {
                    return Err(LabeledError {
                        label: "error when tried to create qr code".to_string(),
                        msg: qr_err.to_string(),
                        span: Some(call.head),
                    })
                }
            },
            _ => {}
        }
        return Err(LabeledError {
            label: "Unsupported input format".to_string(),
            msg: "please provide string in input of you want to export structured data convert them to json before converting to qr code".to_string(),
            span: Some(call.head),
        });
    }
}

fn generate_qr(val: &String) -> Result<String, qr2term::QrError> {
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

fn main() {
    nu_plugin::serve_plugin(&mut Plugin {}, nu_plugin::MsgPackSerializer {})
}

