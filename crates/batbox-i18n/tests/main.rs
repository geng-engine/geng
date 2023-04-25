batbox_i18n::gen!(mod i18n: "tests/translations.toml");

#[test]
fn main() {
    assert_eq!(i18n::EN.hello(), "Hello");
    assert_eq!(i18n::RU.hello(), "Привет");

    assert_eq!(i18n::get_or_en("ru").world(), "Мир");
    assert_eq!(i18n::get_or_en("es").world(), "World");
}
