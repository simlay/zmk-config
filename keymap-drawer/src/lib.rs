pub mod words;
pub mod typing;
pub mod highlight;

use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use web_sys::{window, Element, EventTarget, HtmlElement};

use crate::typing::TypingTest;
use crate::highlight::update_keys;

const SVG_CONTENT: &str = include_str!("../corne.svg");

// ─── Global state for typing test ───
thread_local! {
    static TYPING_TEST: RefCell<Option<TypingTest>> = RefCell::new(None);
    static TEXT_DISPLAY: RefCell<Option<Element>> = RefCell::new(None);
    static WPM_DISPLAY: RefCell<Option<HtmlElement>> = RefCell::new(None);
    static PASTE_MODAL: RefCell<Option<Element>> = RefCell::new(None);
    static PASTE_TEXTAREA: RefCell<Option<Element>> = RefCell::new(None);
}

fn init_typing_test(test: TypingTest, text_display: Element, wpm_display: HtmlElement, paste_modal: Option<Element>, paste_textarea: Option<Element>) {
    TYPING_TEST.with(|cell| {
        *cell.borrow_mut() = Some(test);
    });
    TEXT_DISPLAY.with(|cell| {
        *cell.borrow_mut() = Some(text_display);
    });
    WPM_DISPLAY.with(|cell| {
        *cell.borrow_mut() = Some(wpm_display);
    });
    PASTE_MODAL.with(|cell| {
        *cell.borrow_mut() = paste_modal;
    });
    PASTE_TEXTAREA.with(|cell| {
        *cell.borrow_mut() = paste_textarea;
    });
}

fn with_typing_test<F, R>(f: F) -> R
where
    F: FnOnce(&mut TypingTest, &Element, &HtmlElement) -> R,
{
    TEXT_DISPLAY.with(|td| {
        WPM_DISPLAY.with(|wp| {
            TYPING_TEST.with(|tt| {
                f(tt.borrow_mut().as_mut().unwrap(), td.borrow().as_ref().unwrap(), wp.borrow().as_ref().unwrap())
            })
        })
    })
}

fn get_paste_modal() -> Option<Element> {
    PASTE_MODAL.with(|cell| cell.borrow().clone())
}

fn get_paste_textarea() -> Option<Element> {
    PASTE_TEXTAREA.with(|cell| cell.borrow().clone())
}

pub fn init() {
    console_error_panic_hook::set_once();

    let document = window()
        .and_then(|win| win.document())
        .expect("Could not access the document");

    let keyboard = document
        .get_element_by_id("keyboard")
        .expect("Could not find #keyboard element");

    keyboard.set_inner_html(SVG_CONTENT);

    // Setup typing test
    let container = document.get_element_by_id("typing-test").expect("Could not find #typing-test");
    let text_display: Element = container.query_selector(".text-display").expect("Failed to query").expect("Could not find .text-display").unchecked_into();
    let wpm_display: HtmlElement = container.query_selector(".wpm-display").expect("Failed to query").expect("Could not find .wpm-display").unchecked_into();
    let input: HtmlElement = container.query_selector(".typing-input").expect("Failed to query").expect("Could not find .typing-input").unchecked_into();
    let reset_btn = container.query_selector(".reset-btn").expect("Failed to query").and_then(|el| el.dyn_into::<HtmlElement>().ok());
    let paste_btn = container.query_selector(".paste-btn").expect("Failed to query").and_then(|el| el.dyn_into::<HtmlElement>().ok());
    let paste_modal = container.query_selector(".paste-modal").expect("Failed to query");
    let paste_textarea = container.query_selector(".paste-textarea").expect("Failed to query");
    let paste_cancel = container.query_selector(".btn-cancel").expect("Failed to query").and_then(|el| el.dyn_into::<HtmlElement>().ok());
    let paste_submit = container.query_selector(".btn-paste").expect("Failed to query").and_then(|el| el.dyn_into::<HtmlElement>().ok());

    // Initialize global state
    init_typing_test(
        TypingTest::new(),
        text_display.clone(),
        wpm_display.clone(),
        paste_modal,
        paste_textarea,
    );

    // Start the typing test
    with_typing_test(|test, td, wp| {
        test.start_new_text(td, wp);
    });
    let _ = input.focus();

    // Reset button handler
    if let Some(reset_btn) = reset_btn {
        let input_reset = input.clone();
        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |_e: web_sys::MouseEvent| {
            with_typing_test(|test, td, wp| {
                test.start_new_text(td, wp);
            });
            let _ = input_reset.focus();
        }) as Box<dyn FnMut(_)>);
        let _ = reset_btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
        closure.forget();
    }

    // Paste button handler
    if let Some(paste_btn) = paste_btn {
        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |_e: web_sys::MouseEvent| {
            if let Some(paste_modal) = get_paste_modal() {
                let _ = paste_modal.set_class_name("paste-modal active");
            }
        }) as Box<dyn FnMut(_)>);
        let _ = paste_btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
        closure.forget();
    }

    // Paste cancel handler
    if let Some(paste_cancel) = paste_cancel {
        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |_e: web_sys::MouseEvent| {
            if let Some(paste_modal) = get_paste_modal() {
                let _ = paste_modal.set_class_name("paste-modal");
            }
        }) as Box<dyn FnMut(_)>);
        let _ = paste_cancel.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
        closure.forget();
    }

    // Paste submit handler
    if let Some(paste_submit) = paste_submit {
        let input_submit = input.clone();
        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |_e: web_sys::MouseEvent| {
            if let Some(textarea) = get_paste_textarea() {
                if let Some(textarea_el) = textarea.dyn_ref::<web_sys::HtmlInputElement>() {
                    let text = textarea_el.value();
                    let trimmed = text.trim();
                    if !trimmed.is_empty() {
                        with_typing_test(|test, td, wp| {
                            test.submit_custom_text(trimmed, td, wp);
                        });
                        let _ = input_submit.focus();
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);
        let _ = paste_submit.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
        closure.forget();
    }

    // Typing input keydown handler
    let input_keydown = input.clone();
    let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
        let key = e.key();

        if key == "Backspace" {
            e.prevent_default();
            with_typing_test(|test, td, wp| {
                test.handle_backspace(td, wp);
            });
        } else if key == "Escape" {
            e.prevent_default();
            with_typing_test(|test, td, wp| {
                test.start_new_text(td, wp);
            });
            let _ = input_keydown.focus();
        } else if key.len() == 1 {
            // Regular character key - let it pass through for keyboard highlighting
            with_typing_test(|test, td, wp| {
                test.handle_key_press(key.chars().next().unwrap(), td, wp);
            });
        }
    }) as Box<dyn FnMut(_)>);
    let _ = input.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
    closure.forget();

    // Set up keyboard event listeners for key highlighting
    let document = window().and_then(|win| win.document()).expect("Could not access document");
    let document: EventTarget = document.into();

    // Keydown handler
    {
        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            update_keys(&e, "#ff4444");
        }) as Box<dyn FnMut(_)>);
        document
            .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
            .expect("Failed to add keydown listener");
        closure.forget();
    }

    // Keyup handler
    {
        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
            update_keys(&e, "");
        }) as Box<dyn FnMut(_)>);
        document
            .add_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref())
            .expect("Failed to add keyup listener");
        closure.forget();
    }
}
