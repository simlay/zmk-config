use wasm_bindgen::JsCast;
use web_sys::{window, Element, KeyboardEvent};

pub fn get_key_label(key: &str, e: &KeyboardEvent) -> Option<String> {
    match key {
        " " => Some("SPACE".to_string()),
        "Backspace" => Some("BSPC".to_string()),
        "Tab" => Some("TAB".to_string()),
        "Enter" => Some("RET".to_string()),
        "Escape" => Some("ESC".to_string()),
        "ArrowLeft" => Some("LEFT".to_string()),
        "ArrowUp" => Some("UP".to_string()),
        "ArrowDown" => Some("DOWN".to_string()),
        "ArrowRight" => Some("RIGHT".to_string()),
        "Control" => {
            let code = e.code();
            Some(if code.contains("Left") { "LCTRL".to_string() } else { "RCTRL".to_string() })
        }
        "Shift" => {
            let code = e.code();
            Some(if code.contains("Left") { "LSHFT".to_string() } else { "RSHFT".to_string() })
        }
        "Meta" => {
            let code = e.code();
            Some(if code.contains("Left") { "LGUI".to_string() } else { "RGUI".to_string() })
        }
        _ => None,
    }
}

pub fn find_keys_by_label(label: &str) -> Vec<Element> {
    let document = window().and_then(|win| win.document()).expect("Could not access document");
    
    let keys = document
        .query_selector_all(".key")
        .expect("Failed to query selectors");
    
    let mut result = Vec::new();
    
    for i in 0..keys.length() {
        let node = keys.get(i).expect("Failed to get key element");
        let key_el: Element = node.unchecked_into();
        
        if let Ok(Some(tap_text)) = key_el.query_selector("text.tap") {
            if let Some(text_content) = tap_text.text_content() {
                if text_content == label {
                    result.push(key_el);
                }
            }
        }
    }
    
    result
}

pub fn set_key_fill(key_el: &Element, fill: &str) {
    if let Ok(Some(rect)) = key_el.query_selector("rect.key") {
        use wasm_bindgen::JsValue;
        if let Ok(style) = js_sys::Reflect::get(&rect, &JsValue::from_str("style")) {
            let _ = js_sys::Reflect::set(&style, &JsValue::from_str("fill"), &JsValue::from_str(fill));
        }
    }
}

pub fn update_keys(e: &KeyboardEvent, fill: &str) {
    let key = e.key();
    
    let label = if let Some(custom_label) = get_key_label(&key, e) {
        custom_label
    } else if key.is_empty() {
        return;
    } else if key.len() == 1 {
        key.to_uppercase()
    } else {
        return;
    };
    
    let keys = find_keys_by_label(&label);
    for key_el in keys {
        set_key_fill(&key_el, fill);
    }
}
