use super::*;

pub trait Config<T> {
    fn get(&self) -> T;
    fn ui<'a>(&'a mut self, cx: &'a Controller) -> Box<dyn Widget + 'a>;
}

pub trait Configurable: Sized {
    type Config: Config<Self>;
    fn config(theme: &Rc<Theme>, value: Self) -> Self::Config;
}

pub struct ShowValue<T: ToString + Clone> {
    theme: Rc<Theme>,
    value: T,
    text: Option<String>,
}

impl<T: ToString + Clone> Config<T> for ShowValue<T> {
    fn get(&self) -> T {
        self.value.clone()
    }
    fn ui<'a>(&'a mut self, _cx: &'a Controller) -> Box<dyn Widget + 'a> {
        if self.text.is_none() {
            self.text = Some(self.value.to_string());
        }
        Box::new(Text::new(
            self.text.as_ref().unwrap(),
            &self.theme.font,
            16.0,
            self.theme.text_color,
        ))
    }
}

impl<T: ToString + Clone> Configurable for T {
    type Config = ShowValue<T>;
    fn config(theme: &Rc<Theme>, value: T) -> ShowValue<T> {
        ShowValue {
            theme: theme.clone(),
            value,
            text: None,
        }
    }
}
