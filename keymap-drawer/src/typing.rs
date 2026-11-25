use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlElement};

use crate::words::{SimpleRng, WORDS};

const TYPING_WINDOW_MS: u64 = 3 * 60 * 1000;
const WORD_TIME_LIMIT_MS: u64 = 60 * 1000;
const LINES_ON_SCREEN: usize = 3;

#[derive(Clone)]
pub(crate) struct Line {
    words: Vec<String>,
}

#[derive(Clone)]
pub struct TypingTest {
    lines: Vec<Line>,
    active_line: usize, // index into lines of the current visible line
    position: usize,    // position within active_line (character index in joined words)
    char_states: Vec<u8>, // 0=pending, 1=confirmed, 2=wrong
    start_time: Option<u64>,
    // Each completed word records (end_time, duration_ms, char_count)
    word_stats: Vec<(u64, u64, usize)>,
    current_word_start: Option<u64>,
    char_word_start_pos: usize, // character position where current word started
    is_finished: bool,
    custom_lines: Option<Vec<Vec<String>>>,
    rng: SimpleRng,
}

impl TypingTest {
    pub fn new() -> Self {
        // Use current time in ms as seed for different text each time
        let seed = (js_sys::Date::now() / 1000.0) as u64;
        Self {
            lines: Vec::new(),
            active_line: 0,
            position: 0,
            char_states: Vec::new(),
            start_time: None,
            word_stats: Vec::new(),
            current_word_start: None,
            char_word_start_pos: 0,
            is_finished: false,
            custom_lines: None,
            rng: SimpleRng::new(seed),
        }
    }

    fn generate_words(&mut self, max_width: usize) -> Vec<String> {
        let mut words = Vec::new();
        loop {
            let word_idx = self.rng.next_usize(WORDS.len());
            let word = WORDS[word_idx].to_string();
            let line_width: usize = words.iter().map(|w: &String| w.len() + 1).sum();
            if line_width + word.len() >= max_width {
                break;
            }
            words.push(word);
        }
        words
    }

    fn add_lines(&mut self, count: usize) {
        for _ in 0..count {
            let words = self.generate_words(44);
            self.lines.push(Line { words });
        }
    }

    pub fn generate_random_text(&mut self) {
        self.lines.clear();
        self.add_lines(LINES_ON_SCREEN + 2); // buffer
        self.custom_lines = None;
    }

    pub fn init_char_states(&mut self) {
        let line = &self.lines[self.active_line];
        self.char_states = vec![0u8; line.words.join(" ").len()];
    }

    fn render_text(
        text_display: &Element,
        wpm_display: &HtmlElement,
        position: usize,
        char_states: &[u8],
        lines: &[Line],
        active_line: usize,
        word_stats: &[(u64, u64, usize)],
        start_time: Option<u64>,
    ) {
        // Clear and rebuild the text display
        let _ = text_display.set_inner_html("");
        
        let document = web_sys::window().unwrap().document().unwrap();
        
        // Render 3 lines: current active line + next 2 lines
        for i in 0..3.min(3) {
            let line_idx = active_line + i;
            if line_idx >= lines.len() {
                break;
            }
            let line = &lines[line_idx];
            let full_text = line.words.join(" ");
            
            // Wrap each line in a span with white-space: nowrap to keep it on one row
            let line_span: HtmlElement = document.create_element("span").unwrap().unchecked_into();
            line_span.set_class_name("typing-line");
            
            let mut pos = 0;
            for ch in full_text.chars() {
                let span: HtmlElement = document.create_element("span").unwrap().unchecked_into();
                span.set_class_name("char");
                span.set_text_content(Some(&ch.to_string()));

                // Only the first line shows typed state; preview lines are always fresh
                let class_name: &str = if i == 0 {
                    match char_states.get(pos) {
                        Some(&1) => "char correct",
                        Some(&2) => "char wrong",
                        Some(_) if pos == position => "char current",
                        _ => "char pending",
                    }
                } else {
                    "char preview"
                };
                span.set_class_name(&class_name);

                let _ = line_span.append_child(&span);
                pos += 1;
            }
            
            let _ = text_display.append_child(&line_span);
        }

        // Scroll current character into view
        if let Ok(nodes) = document.query_selector_all(".current") {
            if nodes.length() > 0 {
                if let Some(current_span) = nodes.get(0) {
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

        // Update WPM and CPM based on the time taken to type each word.
        let now = js_sys::Date::now() as u64;
        let valid_words: Vec<&(u64, u64, usize)> = word_stats
            .iter()
            .filter(|&&(end, duration, _)| {
                now - end < TYPING_WINDOW_MS && duration <= WORD_TIME_LIMIT_MS
            })
            .collect();

        if valid_words.is_empty() {
            let _ = wpm_display.set_text_content(Some("WPM: 0  CPM: 0"));
            return;
        }

        let elapsed_min = if let Some(start) = start_time {
            (now - start) as f64 / 60000.0
        } else {
            0.0
        };
        if elapsed_min <= 0.0 {
            let _ = wpm_display.set_text_content(Some("WPM: 0  CPM: 0"));
            return;
        }

        let wpm = (valid_words.len() as f64 / elapsed_min).round() as u32;
        let total_chars: usize = valid_words.iter().map(|&&(_, _, chars)| chars).sum();
        let cpm = (total_chars as f64 / elapsed_min).round() as u32;
        let _ = wpm_display.set_text_content(Some(&format!("WPM: {}  CPM: {}", wpm, cpm)));
    }

    /// Get the current line as a joined string of words
    fn get_line_text(&self) -> String {
        let line = &self.lines[self.active_line];
        line.words.join(" ")
    }

    /// Move to the next line, discarding the completed line and adding a new one
    fn advance_to_next_line(&mut self) {
        self.active_line += 1;
        
        // Add buffer lines if we're running low
        if self.lines.len() - self.active_line < LINES_ON_SCREEN {
            self.add_lines(2);
        }
        
        self.position = 0;
        self.init_char_states();
        // Clear any leftover states beyond the new line's length
        let line_len = self.get_line_text().len();
        for state in self.char_states.iter_mut().take(line_len) {
            *state = 0; // pending
        }
        self.current_word_start = None;
        self.char_word_start_pos = 0;
    }

    pub fn advance_position(&mut self) {
        let now = js_sys::Date::now() as u64;
        let p = self.position;
        let line_text = self.get_line_text();

        // A new word begins when typing the very first character or after a space
        let starts_word = p == 0 || line_text.chars().nth(p - 1) == Some(' ');
        if starts_word {
            self.current_word_start = Some(now);
            self.char_word_start_pos = p;
        }

        // A word finishes when followed by a space or end of text
        let word_finished = p + 1 >= line_text.len()
            || line_text.chars().nth(p + 1) == Some(' ');
        if word_finished {
            if let Some(start) = self.current_word_start {
                let word_char_count = self.position - self.char_word_start_pos;
                self.word_stats.push((now, now - start, word_char_count));
            }
            self.current_word_start = None;
        }

        self.position += 1;

        // If we've completed the entire line, advance to next line
        if self.position >= line_text.len() {
            self.advance_to_next_line();
        }
    }

    pub fn handle_key_press(&mut self, key: char, text_display: &Element, wpm_display: &HtmlElement) {
        if self.is_finished {
            return;
        }

        if self.start_time.is_none() {
            self.start_time = Some(js_sys::Date::now() as u64);
        }

        let line_text = self.get_line_text();
        if self.position >= line_text.len() {
            return;
        }

        let expected = line_text.chars().nth(self.position).unwrap();

        if key == expected {
            self.char_states[self.position] = 1; // confirmed
        } else {
            self.char_states[self.position] = 2; // wrong
        }

        self.advance_position();
        Self::render_text(text_display, wpm_display, self.position, &self.char_states, &self.lines, self.active_line, &self.word_stats, self.start_time);
    }

    pub fn handle_backspace(&mut self, text_display: &Element, wpm_display: &HtmlElement) {
        if self.position == 0 {
            return;
        }

        self.position -= 1;

        let line_text = self.get_line_text();
        if line_text.chars().nth(self.position) == Some(' ') && !self.word_stats.is_empty() {
            self.word_stats.pop();
        }

        self.char_states[self.position] = 0; // pending

        Self::render_text(text_display, wpm_display, self.position, &self.char_states, &self.lines, self.active_line, &self.word_stats, self.start_time);
    }

    pub fn start_new_text(&mut self, text_display: &Element, wpm_display: &HtmlElement) {
        self.active_line = 0;
        
        if let Some(ref custom) = self.custom_lines {
            self.lines.clear();
            for words in custom {
                self.lines.push(Line {
                    words: words.clone(),
                });
            }
        } else {
            self.generate_random_text();
        }
        
        self.init_char_states();
        self.position = 0;
        self.start_time = None;
        self.word_stats.clear();
        self.is_finished = false;
        Self::render_text(text_display, wpm_display, self.position, &self.char_states, &self.lines, self.active_line, &self.word_stats, self.start_time);
    }

    pub fn submit_custom_text(&mut self, text: &str, text_display: &Element, wpm_display: &HtmlElement) {
        let trimmed = text.trim();
        if trimmed.is_empty() {
            return;
        }
        
        // Split text into lines that fit within 44 characters
        let mut custom_lines = Vec::new();
        let words: Vec<String> = trimmed.split_whitespace().map(|s| s.to_string()).collect();
        
        let mut current_line = Vec::new();
        for word in words {
            let line_width: usize = current_line.iter().map(|w: &String| w.len() + 1).sum();
            if line_width + word.len() >= 44 && !current_line.is_empty() {
                custom_lines.push(current_line);
                current_line = Vec::new();
            }
            current_line.push(word);
        }
        if !current_line.is_empty() {
            custom_lines.push(current_line);
        }
        
        self.custom_lines = Some(custom_lines);
        self.start_new_text(text_display, wpm_display);
    }
}
