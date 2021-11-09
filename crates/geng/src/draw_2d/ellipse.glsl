uniform mat3 u_projection_matrix;
uniform mat3 u_view_matrix;
varying vec2 v_quad_pos;
uniform ivec2 u_framebuffer_size;

#ifdef VERTEX_SHADER
attribute vec2 a_quad_pos;
uniform vec2 u_pos;
uniform vec2 u_radius;
void main() {
    v_quad_pos = a_quad_pos;
    vec3 pos = u_projection_matrix * u_view_matrix * vec3(u_pos + u_radius * a_quad_pos, 1.0);
    gl_Position = vec4(pos.xy, 0.0, pos.z);
}
#endif

#ifdef FRAGMENT_SHADER
uniform vec4 u_color;
void main() {
    if (length(v_quad_pos) > 1.0) {
        discard;
    }
    gl_FragColor = u_color;
#ifdef GENG_ANTIALIAS
    vec3 pixelV = inverse(u_projection_matrix * u_view_matrix) * vec3(vec2(1.0, 1.0) / vec2(u_framebuffer_size), 0.0);
    float pixelR = length(pixelV.xy);
    gl_FragColor.w *= min((1.0 - length(v_quad_pos)) / (pixelR / 1.0), 1.0);
#endif
}
#endif