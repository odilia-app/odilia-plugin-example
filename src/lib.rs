use wai_bindgen_rust::*;

export!("odilia-test-plugin.wai");

pub struct OdiliaTestPlugin;

impl odilia_test_plugin::OdiliaTestPlugin for OdiliaTestPlugin {
	fn init() -> bool {
		false
	}
	fn addone(input: i32) -> i32 {
		input+1
	}
}
