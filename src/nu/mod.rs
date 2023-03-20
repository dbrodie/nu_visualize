use crate::Visualize;
use nu_plugin::{EvaluatedCall, LabeledError, Plugin};
use nu_protocol::{Category, PluginSignature, SyntaxShape, Value};

impl Plugin for Visualize {
    fn signature(&self) -> Vec<PluginSignature> {
        eprintln!("HELP!");
        // It is possible to declare multiple signature in a plugin
        // Each signature will be converted to a command declaration once the
        // plugin is registered to nushell
        vec![
            PluginSignature::build("viz")
        ]
    }

    fn run(
        &mut self,
        name: &str,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
            if name != "viz" {
                return Ok(Value::Nothing { span: call.head });
            }

            self.viz(call.head, input)
    }
}

