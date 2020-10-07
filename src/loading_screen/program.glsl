varying vec2 v_pos;

#ifdef VERTEX_SHADER
attribute vec2 a_pos;
uniform ivec2 u_framebuffer_size;
uniform float u_time;
void main() {
    v_pos = a_pos;
    float angle = 3.0 * u_time;
    float cs = cos(angle);
    float sn = sin(angle);
    vec2 pos = mat2(cs, sn, -sn, cs) * a_pos / 20.0;
    gl_Position = vec4(pos.x * float(u_framebuffer_size.y) / float(u_framebuffer_size.x), pos.y, 0.0, 1.0);
}
#endif

#ifdef FRAGMENT_SHADER
void main() {
    gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
}
#endif