use tonic_web_wasm_client::Client;

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct CardegoDataServiceWasm {
    pub(crate) s: String,
}

#[wasm_bindgen]
impl CardegoDataServiceWasm {
    pub fn greet() {
        let _ = cardego_grpc::generated::cardego_data::cardego_data_service_client::CardegoDataServiceClient::new(
            Client::new("http://localhost:8080".to_string())
        );
    }
}

