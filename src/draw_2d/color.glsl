varying vec4 v_color;

#ifdef VERTEX_SHADER
attribute vec2 a_pos;
attribute vec4 a_color;
uniform ivec2 u_framebuffer_size;
void main() {
    v_color = a_color;
    vec2 pos = a_pos / vec2(u_framebuffer_size) * 2.0 - 1.0;
    gl_Position = vec4(pos, 0.0, 1.0);
}
#endif

#ifdef FRAGMENT_SHADER
uniform vec4 u_color;
void main() {
    gl_FragColor = v_color * u_color;
}
#endif