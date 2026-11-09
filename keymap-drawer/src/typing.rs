use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlElement};

use crate::words::{SimpleRng, WORDS};

const TYPING_WINDOW_MS: u64 = 3 * 60 * 1000;
const TYPING_WORD_COUNT: usize = 100;

#[derive(Clone)]
pub struct TypingTest {
    current_text: String,
    position: usize,
    char_states: Vec<u8>, // 0=pending, 1=confirmed, 2=wrong
    start_time: Option<u64>,
    word_times: Vec<u64>,
    is_finished: bool,
    custom_text: Option<String>,
    rng: SimpleRng,
}

impl TypingTest {
    pub fn new() -> Self {
        Self {
            current_text: String::new(),
            position: 0,
            char_states: Vec::new(),
            start_time: None,
            word_times: Vec::new(),
            is_finished: false,
            custom_text: None,
            rng: SimpleRng::new(42),
        }
    }

    pub fn generate_random_text(&mut self) {
        let mut text = String::with_capacity(TYPING_WORD_COUNT * 5);
        for i in 0..TYPING_WORD_COUNT {
            if i > 0 {
                text.push(' ');
            }
            let word_idx = self.rng.next_usize(WORDS.len());
            text.push_str(WORDS[word_idx]);
        }
        self.current_text = text;
        self.custom_text = None;
    }

    pub fn init_char_states(&mut self) {
        self.char_states = vec![0u8; self.current_text.len()];
    }

    pub fn render_text(
        text_display: &Element,
        wpm_display: &HtmlElement,
        position: usize,
        char_states: &[u8],
        current_text: &str,
        word_times: &[u64],
    ) {
        // Clear and rebuild the text display
        let _ = text_display.set_inner_html("");
        
        let document = web_sys::window().unwrap().document().unwrap();
        
        for (i, ch) in current_text.chars().enumerate() {
            let span: HtmlElement = document.create_element("span").unwrap().unchecked_into();
            span.set_class_name("char");
            span.set_text_content(Some(&ch.to_string()));

            let class_name = match char_states[i] {
                1 => "char correct",
                2 => "char wrong",
                _ if i == position => "char current",
                _ => "char pending",
            };
            span.set_class_name(class_name);

            let _ = text_display.append_child(&span);
        }

        // Scroll current character into view
        if let Ok(nodes) = document.query_selector_all(".current") {
            if nodes.length() > 0 {
                if let Some(current_span) = nodes.get(0) {
                    // Use js_sys to call scrollIntoView directly
                    use wasm_bindgen::JsValue;
                    let _ = js_sys::Reflect::get(&current_span, &JsValue::from_str("scrollIntoView"))
                        .and_then(|scroll_into_view| {
                            if let Some(func) = scroll_into_view.dyn_ref::<js_sys::Function>() {
                                func.call0(&JsValue::NULL).map(|_| ())
                            } else {
                                Err(scroll_into_view)
                            }
                        });
                }
            }
        }

        // Update WPM
        let now = js_sys::Date::now() as u64;
        let mut filtered_times: Vec<u64> = word_times.iter().filter(|&&t| now - t < TYPING_WINDOW_MS).copied().collect();
        
        if filtered_times.is_empty() {
            let _ = wpm_display.set_text_content(Some("WPM: 0"));
            return;
        }

        filtered_times.sort();
        let elapsed_min = (now - filtered_times[0]) as f64 / 60000.0;
        if elapsed_min <= 0.0 {
            let _ = wpm_display.set_text_content(Some("WPM: 0"));
            return;
        }

        let wpm = (filtered_times.len() as f64 / elapsed_min).round() as u32;
        let _ = wpm_display.set_text_content(Some(&format!("WPM: {}", wpm)));
    }

    pub fn advance_position(&mut self) {
        if self.current_text.chars().nth(self.position) == Some(' ') {
            self.word_times.push(js_sys::Date::now() as u64);
        }
        self.position += 1;
        if self.position >= self.current_text.len() {
            self.is_finished = true;
        }
    }

    pub fn handle_key_press(&mut self, key: char, text_display: &Element, wpm_display: &HtmlElement) {
        if self.is_finished || self.position >= self.current_text.len() {
            return;
        }

        if self.start_time.is_none() {
            self.start_time = Some(js_sys::Date::now() as u64);
        }

        let expected = self.current_text.chars().nth(self.position).unwrap();

        if key == expected {
            self.char_states[self.position] = 1; // confirmed
        } else {
            self.char_states[self.position] = 2; // wrong
        }

        self.advance_position();
        Self::render_text(text_display, wpm_display, self.position, &self.char_states, &self.current_text, &self.word_times);
    }

    pub fn handle_backspace(&mut self, text_display: &Element, wpm_display: &HtmlElement) {
        if self.position == 0 {
            return;
        }

        self.position -= 1;

        if self.current_text.chars().nth(self.position) == Some(' ') && !self.word_times.is_empty() {
            self.word_times.pop();
        }

        self.char_states[self.position] = 0; // pending

        Self::render_text(text_display, wpm_display, self.position, &self.char_states, &self.current_text, &self.word_times);
    }

    pub fn start_new_text(&mut self, text_display: &Element, wpm_display: &HtmlElement) {
        if let Some(ref custom) = self.custom_text {
            self.current_text = custom.clone();
        } else {
            self.generate_random_text();
        }
        self.position = 0;
        self.start_time = None;
        self.word_times.clear();
        self.is_finished = false;
        self.init_char_states();
        Self::render_text(text_display, wpm_display, self.position, &self.char_states, &self.current_text, &self.word_times);
    }

    pub fn submit_custom_text(&mut self, text: &str, text_display: &Element, wpm_display: &HtmlElement) {
        let trimmed = text.trim();
        if trimmed.is_empty() {
            return;
        }
        self.custom_text = Some(trimmed.to_string());
        self.start_new_text(text_display, wpm_display);
    }
}
