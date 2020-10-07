#[macro_export]
macro_rules! column {
    ($($x:expr),* $(,)?) => {
        geng::ui::column(vec![$(Box::new($x)),*])
    };
}

#[macro_export]
macro_rules! stack {
    ($($x:expr),* $(,)?) => {
        geng::ui::stack(vec![$(Box::new($x)),*])
    };
}

#[macro_export]
macro_rules! row {
    ($($x:expr),* $(,)?) => {
        geng::ui::row(vec![$(Box::new($x)),*])
    };
}
