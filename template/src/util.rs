use std::fs::File;
use std::io;
use tauri::Window;

pub fn zoom_webview(window: &Window, factor: f64) {
    window
        .with_webview(move |webview| {
            #[cfg(target_os = "linux")]
            {
                // see https://docs.rs/webkit2gtk/0.18.2/webkit2gtk/struct.WebView.html
                // and https://docs.rs/webkit2gtk/0.18.2/webkit2gtk/trait.WebViewExt.html
                use webkit2gtk::traits::WebViewExt;
                webview.inner().set_zoom_level(factor);
            }

            #[cfg(windows)]
            unsafe {
                // see https://docs.rs/webview2-com/0.19.1/webview2_com/Microsoft/Web/WebView2/Win32/struct.ICoreWebView2Controller.html
                webview.controller().SetZoomFactor(factor).unwrap();
            }
            #[cfg(target_os = "macos")]
            unsafe {
                let () = msg_send![webview.inner(), setPageZoom: factor];
                // let () = msg_send![webview.controller(), removeAllUserScripts];
                // let bg_color: cocoa::base::id = msg_send![class!(NSColor), colorWithDeviceRed:0.5 green:0.2 blue:0.4 alpha:1.];
                // let () = msg_send![webview.ns_window(), setBackgroundColor: bg_color];
            }
        })
        .expect("error while setting webview");
}

// alphanumeric filters a string to only contain alphanumeric characters
// and replaces all other characters with the given character
pub fn alphanumeric(s: &str, replace: char) -> String {
    s.chars()
        .map(|c| if c.is_alphanumeric() { c } else { replace })
        .collect()
}

pub fn download_file(url: &str, path: &str) {
    let resp = reqwest::blocking::get(url).expect("request failed");
    let body = resp.text().expect("body invalid");
    let mut out = File::create(path).expect("failed to create file");
    io::copy(&mut body.as_bytes(), &mut out).expect("failed to copy content");
}
