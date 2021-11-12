uniform mat3 u_projection_matrix;
uniform mat3 u_view_matrix;
uniform mat3 u_model_matrix;
varying vec2 v_quad_pos;
uniform ivec2 u_framebuffer_size;

#ifdef VERTEX_SHADER
attribute vec2 a_quad_pos;
void main() {
    v_quad_pos = a_quad_pos;
    vec3 pos = u_projection_matrix * u_view_matrix * u_model_matrix * vec3(a_quad_pos, 1.0);
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
    mat3 im = inverse(u_projection_matrix * u_view_matrix * u_model_matrix);
    vec3 pixelP1 = im * vec3(vec2(0.0, 0.0) * 2.0 / vec2(u_framebuffer_size), 1.0);
    pixelP1.xy /= pixelP1.z;
    vec3 pixelP2 = im * vec3(vec2(1.0, 1.0) * 2.0 / vec2(u_framebuffer_size), 1.0);
    pixelP2.xy /= pixelP2.z;
    float pixelLength = length((pixelP2 - pixelP1).xy);
    gl_FragColor.w *= min((1.0 - length(v_quad_pos)) / pixelLength, 1.0);
#endif
}
#endif