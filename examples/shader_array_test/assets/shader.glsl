// https://github.com/bwasty/gltf-viewer/blob/master/src/shaders

varying vec2 v_uv;
varying vec2 v_mr_uv;
varying vec4 v_color;
varying vec3 v_normal;
varying vec3 v_world_pos;

vec3 vec4_to_3(vec4 v) {
    return v.xyz / v.w;
}

#ifdef VERTEX_SHADER

attribute vec2 a_uv;
attribute vec2 a_mr_uv;
attribute vec3 a_pos;
attribute vec3 a_normal;
attribute vec4 a_color;

uniform mat4 u_projection_matrix;
uniform mat4 u_view_matrix;
uniform mat4 u_model_matrix[2];

void main() {
    v_uv = a_uv;
    v_mr_uv = a_mr_uv;
    v_color = a_color;
    v_world_pos = vec4_to_3(u_model_matrix[1] * vec4(a_pos, 1.0));
    v_normal = normalize(vec3(u_model_matrix[1] * vec4(a_normal, 0.0)));
    gl_Position = u_projection_matrix * u_view_matrix * vec4(v_world_pos, 1.0);
}
#endif

#ifdef FRAGMENT_SHADER

uniform sampler2D u_base_color_texture;
uniform vec4 u_base_color_factor;
uniform sampler2D u_metallic_roughness_texture;
uniform float u_metallic_factor;
uniform float u_roughness_factor;

uniform vec3 u_eye_pos;
uniform vec3 u_light_dir;
uniform vec4 u_light_color;
uniform vec4 u_ambient_light_color;
uniform float u_ambient_light_intensity;

// Encapsulate the various inputs used by the various functions in the shading equation
// We store values in this struct to simplify the integration of alternative implementations
// of the shading terms, outlined in the Readme.MD Appendix.
struct PBRInfo
{
    float NdotL;                  // cos angle between normal and light direction
    float NdotV;                  // cos angle between normal and view direction
    float NdotH;                  // cos angle between normal and half vector
    float LdotH;                  // cos angle between light direction and half vector
    float VdotH;                  // cos angle between view direction and half vector
    float perceptualRoughness;    // roughness value, as authored by the model creator (input to shader)
    float metalness;              // metallic value at the surface
    vec3 reflectance0;            // full reflectance color (normal incidence angle)
    vec3 reflectance90;           // reflectance color at grazing angle
    float alphaRoughness;         // roughness mapped to a more linear change in the roughness (proposed by [2])
    vec3 diffuseColor;            // color contribution from diffuse lighting
    vec3 specularColor;           // color contribution from specular lighting
};

const float M_PI = 3.141592653589793;
const float c_MinRoughness = 0.04;

// Basic Lambertian diffuse
// Implementation from Lambert's Photometria https://archive.org/details/lambertsphotome00lambgoog
// See also [1], Equation 1
vec3 diffuse(PBRInfo pbrInputs)
{
    return pbrInputs.diffuseColor / M_PI;
}

// The following equation models the Fresnel reflectance term of the spec equation (aka F())
// Implementation of fresnel from [4], Equation 15
vec3 specularReflection(PBRInfo pbrInputs)
{
    return pbrInputs.reflectance0 + (pbrInputs.reflectance90 - pbrInputs.reflectance0) * pow(clamp(1.0 - pbrInputs.VdotH, 0.0, 1.0), 5.0);
}

// This calculates the specular geometric attenuation (aka G()),
// where rougher material will reflect less light back to the viewer.
// This implementation is based on [1] Equation 4, and we adopt their modifications to
// alphaRoughness as input as originally proposed in [2].
float geometricOcclusion(PBRInfo pbrInputs)
{
    float NdotL = pbrInputs.NdotL;
    float NdotV = pbrInputs.NdotV;
    float r = pbrInputs.alphaRoughness;

    float attenuationL = 2.0 * NdotL / (NdotL + sqrt(r * r + (1.0 - r * r) * (NdotL * NdotL)));
    float attenuationV = 2.0 * NdotV / (NdotV + sqrt(r * r + (1.0 - r * r) * (NdotV * NdotV)));
    return attenuationL * attenuationV;
}

// The following equation(s) model the distribution of microfacet normals across the area being drawn (aka D())
// Implementation from "Average Irregularity Representation of a Roughened Surface for Ray Reflection" by T. S. Trowbridge, and K. P. Reitz
// Follows the distribution function recommended in the SIGGRAPH 2013 course notes from EPIC Games [1], Equation 3.
float microfacetDistribution(PBRInfo pbrInputs)
{
    float roughnessSq = pbrInputs.alphaRoughness * pbrInputs.alphaRoughness;
    float f = (pbrInputs.NdotH * roughnessSq - pbrInputs.NdotH) * pbrInputs.NdotH + 1.0;
    return roughnessSq / (M_PI * f * f);
}

void main() {
    vec4 mr_sample = texture2D(u_metallic_roughness_texture, v_mr_uv);
    float perceptualRoughness = mr_sample.g * u_roughness_factor;
    float metallic = mr_sample.b * u_metallic_factor;
    vec4 base_color = texture2D(u_base_color_texture, v_uv) * u_base_color_factor * v_color;

    float alphaRoughness = perceptualRoughness * perceptualRoughness;

    vec3 f0 = vec3(0.04);
    vec3 diffuseColor = base_color.rgb * (vec3(1.0) - f0);
    diffuseColor *= 1.0 - metallic;
    vec3 specularColor = mix(f0, base_color.rgb, metallic);

    // Compute reflectance.
    float reflectance = max(max(specularColor.r, specularColor.g), specularColor.b);

    // For typical incident reflectance range (between 4% to 100%) set the grazing reflectance to 100% for typical fresnel effect.
    // For very low reflectance range on highly diffuse objects (below 4%), incrementally reduce grazing reflecance to 0%.
    float reflectance90 = clamp(reflectance * 25.0, 0.0, 1.0);
    vec3 specularEnvironmentR0 = specularColor.rgb;
    vec3 specularEnvironmentR90 = vec3(1.0, 1.0, 1.0) * reflectance90;

    vec3 n = v_normal; /*getNormal();*/                             // normal at surface point
    vec3 v = normalize(u_eye_pos - v_world_pos);        // Vector from surface point to camera
    vec3 l = normalize(u_light_dir);             // Vector from surface point to light
    vec3 h = normalize(l+v);                          // Half vector between both l and v
    vec3 reflection = -normalize(reflect(v, n));

    float NdotL = clamp(dot(n, l), 0.001, 1.0);
    float NdotV = clamp(abs(dot(n, v)), 0.001, 1.0);
    float NdotH = clamp(dot(n, h), 0.0, 1.0);
    float LdotH = clamp(dot(l, h), 0.0, 1.0);
    float VdotH = clamp(dot(v, h), 0.0, 1.0);

    PBRInfo pbrInputs = PBRInfo(
        NdotL,
        NdotV,
        NdotH,
        LdotH,
        VdotH,
        perceptualRoughness,
        metallic,
        specularEnvironmentR0,
        specularEnvironmentR90,
        alphaRoughness,
        diffuseColor,
        specularColor
    );

    // Calculate the shading terms for the microfacet specular shading model
    vec3 F = specularReflection(pbrInputs);
    float G = geometricOcclusion(pbrInputs);
    float D = microfacetDistribution(pbrInputs);

    // Calculation of analytical lighting contribution
    vec3 diffuseContrib = (1.0 - F) * diffuse(pbrInputs);
    vec3 specContrib = F * G * D / (4.0 * NdotL * NdotV);
    vec3 color = NdotL * u_light_color.rgb * (diffuseContrib + specContrib);

    color += u_ambient_light_color.rgb * u_ambient_light_intensity * base_color.xyz;

    // NOTE: the spec mandates to ignore any alpha value in 'OPAQUE' mode
    // float alpha = mix(1.0, base_color.a, u_AlphaBlend);
    // if (u_AlphaCutoff > 0.0) {
    //     alpha = step(u_AlphaCutoff, base_color.a);
    // }

    // if (alpha == 0.0) {
    //     discard;
    // }
    float alpha = 1.0;

    // TODO!: apply fix from reference shader:
    // https://github.com/KhronosGroup/glTF-WebGL-PBR/pull/55/files#diff-f7232333b020880432a925d5a59e075d
    gl_FragColor = vec4(color, alpha);
}
#endif