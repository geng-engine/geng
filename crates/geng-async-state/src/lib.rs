use futures::prelude::*;

scoped_tls::scoped_thread_local! { static mut WITH_FRAMEBUFFER: for<'a> &'a mut (dyn 'a + FnMut(&mut dyn FnMut(&mut ugli::Framebuffer))) }

pub fn with_current_framebuffer<T>(
    window: &geng_window::Window,
    f: impl FnOnce(&mut ugli::Framebuffer<'_>) -> T,
) -> T {
    if WITH_FRAMEBUFFER.is_set() {
        let mut value = None::<T>;
        let mut f = Some(f);
        WITH_FRAMEBUFFER.with(|with_framebuffer| {
            with_framebuffer(&mut |framebuffer| value = Some(f.take().unwrap()(framebuffer)))
        });
        value.expect("LUL")
    } else {
        window.with_framebuffer(f)
    }
}

pub trait ActiveState {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer);
}

impl<'a, T: ActiveState + ?Sized> ActiveState for &'a mut T {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        (**self).draw(framebuffer)
    }
}

impl<T: ActiveState + ?Sized> ActiveState for Box<T> {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        (**self).draw(framebuffer)
    }
}

pub trait Transition {
    fn finished(&self) -> bool;
    fn draw(
        &mut self,
        from: &mut dyn FnMut(&mut ugli::Framebuffer),
        to: &mut dyn FnMut(&mut ugli::Framebuffer),
        framebuffer: &mut ugli::Framebuffer,
    );
}

pub struct StateResult<'a, T> {
    pub value: T,
    pub active_state: Box<dyn ActiveState + 'a>,
}

pub async fn transition<T>(
    window: &geng_window::Window,
    from: &mut dyn ActiveState,
    transition: &mut dyn Transition,
    into: impl Future<Output = T>,
) -> T {
    let mut into = std::pin::pin!(into);
    std::future::poll_fn(|cx| {
        if transition.finished() {
            into.as_mut().poll(cx)
        } else {
            with_current_framebuffer(window, |actual_framebuffer| {
                WITH_FRAMEBUFFER.set(
                    &mut |with_framebuffer: &mut dyn FnMut(&mut ugli::Framebuffer)| {
                        transition.draw(
                            &mut |f| from.draw(f),
                            with_framebuffer,
                            actual_framebuffer,
                        );
                    },
                    || into.as_mut().poll(cx),
                )
            })
        }
    })
    .await
}
