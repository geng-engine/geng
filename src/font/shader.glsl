varying vec2 v_vt;

#ifdef VERTEX_SHADER
uniform ivec2 u_framebuffer_size;

attribute vec2 a_vt;
attribute vec2 a_pos;
void main() {
    v_vt = a_vt;
    vec2 pos = 2.0 * a_pos / vec2(u_framebuffer_size);
    gl_Position = vec4(pos.x - 1.0, 1.0 - pos.y, 0.0, 1.0);
}
#endif

#ifdef FRAGMENT_SHADER
uniform vec4 u_color;
uniform sampler2D u_cache_texture;
void main() {
    gl_FragColor = u_color;
    gl_FragColor.w *= texture2D(u_cache_texture, v_vt).w;
}
#endif