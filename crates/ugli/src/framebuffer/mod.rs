use super::*;

mod fbo;
mod read;

pub(crate) use fbo::*;
pub use read::*;

pub enum ColorAttachmentRead<'a> {
    None,
    Texture(&'a Texture),
}

pub enum DepthAttachmentRead<'a> {
    None,
    Renderbuffer(&'a Renderbuffer<DepthComponent>),
    RenderbufferWithStencil(&'a Renderbuffer<DepthStencilValue>),
}

pub struct FramebufferRead<'a> {
    pub(crate) fbo: Fbo,
    color: ColorAttachmentRead<'a>,
    depth: DepthAttachmentRead<'a>,
    size: vec2<usize>,
}

impl<'a> FramebufferRead<'a> {
    pub fn new(
        ugli: &Ugli,
        color: ColorAttachmentRead<'a>,
        depth: DepthAttachmentRead<'a>,
    ) -> Self {
        let gl = &ugli.inner.raw;
        let fbo = Fbo::new(ugli);
        fbo.bind();
        let mut size = None;
        match color {
            ColorAttachmentRead::None => {}
            ColorAttachmentRead::Texture(texture) => {
                gl.framebuffer_texture_2d(
                    raw::FRAMEBUFFER,
                    raw::COLOR_ATTACHMENT0,
                    raw::TEXTURE_2D,
                    Some(&texture.handle),
                    0,
                );
                size = Some(texture.size());
            }
        }
        match depth {
            DepthAttachmentRead::None => {}
            DepthAttachmentRead::Renderbuffer(renderbuffer) => {
                gl.framebuffer_renderbuffer(
                    raw::FRAMEBUFFER,
                    raw::DEPTH_ATTACHMENT,
                    raw::RENDERBUFFER,
                    Some(&renderbuffer.handle),
                );
                // TODO: update/check size
            }
            DepthAttachmentRead::RenderbufferWithStencil(renderbuffer) => {
                gl.framebuffer_renderbuffer(
                    raw::FRAMEBUFFER,
                    raw::DEPTH_STENCIL_ATTACHMENT,
                    raw::RENDERBUFFER,
                    Some(&renderbuffer.handle),
                );
            }
        }
        fbo.check();
        ugli.debug_check();
        Self {
            fbo,
            color,
            depth,
            size: size.unwrap(),
        }
    }
    pub fn new_color(ugli: &Ugli, color: ColorAttachmentRead<'a>) -> Self {
        Self::new(ugli, color, DepthAttachmentRead::None)
    }
    pub fn size(&self) -> vec2<usize> {
        self.size
    }

    pub fn color_attachment(&self) -> &ColorAttachmentRead {
        &self.color
    }
    pub fn depth_attachment(&self) -> &DepthAttachmentRead {
        &self.depth
    }
    pub fn destruct(self) -> (ColorAttachmentRead<'a>, DepthAttachmentRead<'a>) {
        (self.color, self.depth)
    }
}

pub enum ColorAttachment<'a> {
    None,
    Texture(&'a mut Texture),
}

pub enum DepthAttachment<'a> {
    None,
    Renderbuffer(&'a mut Renderbuffer<DepthComponent>),
    RenderbufferWithStencil(&'a mut Renderbuffer<DepthStencilValue>),
}

#[derive(Deref)]
pub struct Framebuffer<'a> {
    #[deref]
    read: FramebufferRead<'a>,
}

impl<'a> Framebuffer<'a> {
    pub fn new(ugli: &Ugli, color: ColorAttachment<'a>, depth: DepthAttachment<'a>) -> Self {
        Self {
            read: FramebufferRead::new(
                ugli,
                match color {
                    ColorAttachment::None => ColorAttachmentRead::None,
                    ColorAttachment::Texture(texture) => ColorAttachmentRead::Texture(texture),
                },
                match depth {
                    DepthAttachment::None => DepthAttachmentRead::None,
                    DepthAttachment::Renderbuffer(renderbuffer) => {
                        DepthAttachmentRead::Renderbuffer(renderbuffer)
                    }
                    DepthAttachment::RenderbufferWithStencil(renderbuffer) => {
                        DepthAttachmentRead::RenderbufferWithStencil(renderbuffer)
                    }
                },
            ),
        }
    }
    pub fn new_color(ugli: &Ugli, color: ColorAttachment<'a>) -> Self {
        Self::new(ugli, color, DepthAttachment::None)
    }
    pub fn destruct(self) -> (ColorAttachmentRead<'a>, DepthAttachmentRead<'a>) {
        self.read.destruct()
    }
}

impl<'a> Framebuffer<'a> {
    pub fn default(ugli: &Ugli) -> Self {
        Self {
            read: FramebufferRead {
                fbo: Fbo::default(ugli),
                color: ColorAttachmentRead::None,
                depth: DepthAttachmentRead::None,
                size: ugli.size(),
            },
        }
    }
}
