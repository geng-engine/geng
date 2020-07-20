use crate::*;

use once_cell::sync::Lazy;

static TRANSLATIONS: Lazy<Mutex<HashMap<String, HashMap<&str, &str>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
static LOCALE: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(detect_locale()));
static REPORTED_UNTRANSLATED: Lazy<Mutex<HashMap<String, HashSet<String>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

fn detect_locale() -> String {
    #[cfg(target_arch = "wasm32")]
    let locale = web_sys::window().unwrap().navigator().language();
    #[cfg(not(target_arch = "wasm32"))]
    let locale = unsafe {
        let locale = libc::setlocale(
            libc::LC_COLLATE,
            b"\0" as *const _ as *const std::os::raw::c_char,
        ) as *const _;
        if locale == std::ptr::null() {
            None
        } else {
            std::ffi::CStr::from_ptr(locale)
                .to_str()
                .ok()
                .map(|s| s.to_owned())
        }
    };
    trace!("Detected locale: {:?}", locale);
    let mut locale = match locale {
        Some(locale) => locale,
        None => String::from("en"),
    };
    if locale.len() > 2 {
        locale.split_off(2);
    }
    let locale = locale.to_lowercase();
    trace!("Using locale: {:?}", locale);
    locale
}

pub fn set_locale(locale: &str) {
    *LOCALE.lock().unwrap() = locale.to_owned();
}

pub fn add_translation(locale: &str, src: &'static str, translation: &'static str) {
    let mut translations = TRANSLATIONS.lock().unwrap();
    if !translations.contains_key(locale) {
        translations.insert(locale.to_owned(), HashMap::new());
    }
    let locale_translations = translations.get_mut(locale).unwrap();
    locale_translations.insert(src, translation);
}

pub fn add_translations(src: &'static str) {
    let current = RefCell::new(HashMap::<&'static str, &'static str>::new());
    let add = || {
        let mut current = current.borrow_mut();
        if current.is_empty() {
            return;
        }
        let src = current
            .get("en")
            .expect("Expected english source for translation");
        for (locale, translation) in current.iter() {
            if locale == &"en" {
                continue;
            }
            add_translation(locale, src, translation);
        }
        current.clear();
    };
    for line in src.lines() {
        if line.is_empty() {
            add();
        } else {
            let index = line.find('=').expect("Failed to parse translations");
            current
                .borrow_mut()
                .insert(&line[..index], &line[index + 1..]);
        }
    }
    add();
}

pub fn translate(text: &str) -> &str {
    let translations = TRANSLATIONS.lock().unwrap();
    let locale = LOCALE.lock().unwrap();
    if *locale == "en" {
        return text;
    }
    let locale_translations = translations.get(&**locale);
    if let Some(translations) = locale_translations {
        if let Some(translation) = translations.get(text) {
            return translation;
        }
    }
    if log_enabled!(log::Level::Debug) {
        let mut reported = REPORTED_UNTRANSLATED.lock().unwrap();
        if !reported.contains_key(&**locale) {
            reported.insert(locale.clone(), HashSet::new());
        }
        if reported.get_mut(&**locale).unwrap().insert(text.to_owned()) {
            debug!("{:?} not translated to {:?}", text, &**locale);
        }
    }
    text
}
