mod to_qr;
use crate::to_qr::ToQr;
pub struct QrPlugin;

impl nu_plugin::Plugin for QrPlugin {
    fn commands(&self) -> Vec<Box<dyn nu_plugin::PluginCommand<Plugin = Self>>> {
        vec![Box::new(ToQr::new())]
    }
}



fn main() {
    nu_plugin::serve_plugin(&mut QrPlugin {}, nu_plugin::MsgPackSerializer {})
}

