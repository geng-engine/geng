#ifdef VERTEX_SHADER
attribute vec2 a_pos;
uniform ivec2 u_framebuffer_size;
void main() {
    gl_Position = vec4(a_pos.xy / vec2(u_framebuffer_size) * 2.0 - 1.0, 0.0, 1.0);
}
#endif

#ifdef FRAGMENT_SHADER
void main() {
    gl_FragColor = vec4(1.0);
}
#endif