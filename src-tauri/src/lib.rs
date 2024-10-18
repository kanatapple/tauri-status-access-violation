use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};
use url::Url;

const INITIALIZATION_SCRIPT: &str = include_str!("./initialization_script.js");

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            _ = WebviewWindowBuilder::new(app.app_handle(), "main", WebviewUrl::External(Url::parse("https://st.id.softbank.jp/sbid_auth/type1/2.0/authorization.php?response_type=code&client_id=yPXPPaM2ccrUxdLtHV5VEgbJ3jGeDGic&redirect_uri=https://bff-top.crth.st.bene-st.jp/v1/top/sb-pre-auth-start&state=36eaf7a1-5103-45cf-93ba-97cb642519f2&scope=openid+cdp&nonce=0797f0f2-c821-4f19-8502-ccd789673704&acr_value=1").unwrap())).initialization_script(INITIALIZATION_SCRIPT).build();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
