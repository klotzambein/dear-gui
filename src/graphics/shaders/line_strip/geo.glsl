layout(lines) in;
layout(triangle_strip, max_vertices = 4) out;

in vec2[2] v_vertex;

out vec2 g_uv;
out vec2 g_uv_pw;
out float g_len;

uniform vec2 pixel_width;
uniform float width;
uniform float aspect_ratio;
uniform mat3 model_transform;
uniform mat3 view_transform;


void main() {
    if (isnan(v_vertex[0].x) || isnan(v_vertex[0].y) || isnan(v_vertex[1].x) || isnan(v_vertex[1].y)) {
        return;
    }

    float half_width = width / 2;
    vec3 start = model_transform * vec3(v_vertex[0], 1);
    vec3 end = model_transform * vec3(v_vertex[1], 1);
    float len = length(end - start);
    vec3 tangent = (end - start) / len;
    vec3 normal = tangent.yxz * vec3(-1, 1, 1);

    vec3 p_start = view_transform * start;
    vec3 p_end = view_transform * end;
    vec3 p_tangent = view_transform * tangent;
    vec3 p_normal = view_transform * normal;

    vec3 pn_tangent = normalize(p_tangent);
    vec3 pn_normal = normalize(p_normal);


    vec3 a = p_start + (p_normal - p_tangent) * half_width + (pn_normal - pn_tangent) * vec3(pixel_width, 1);
    vec3 b = p_end + (p_normal + p_tangent) * half_width + (pn_normal + pn_tangent) * vec3(pixel_width, 1);
    vec3 c = p_start + (-p_normal - p_tangent) * half_width + (-pn_normal - pn_tangent) * vec3(pixel_width, 1);
    vec3 d = p_end + (-p_normal + p_tangent) * half_width + (-pn_normal + pn_tangent) * vec3(pixel_width, 1);

    float uv_pw_height = length(pn_tangent.xy * pixel_width) / length(p_tangent * half_width);
    float uv_pw_width = length(pn_normal.xy * pixel_width) / length(p_normal * half_width);
    float uv_height = 1 + uv_pw_height;
    float uv_width = 1 + uv_pw_width;
    float uv_len = len / half_width;
    
    g_uv_pw = vec2(uv_pw_height, uv_pw_width);
    g_uv = vec2(-uv_height, -uv_width);
    g_len = uv_len;
    gl_Position = vec4(a.xy, 0, 1);
    EmitVertex();
    
    g_uv_pw = vec2(uv_pw_height, uv_pw_width);
    g_uv = vec2(uv_len + uv_height, -uv_width);
    g_len = uv_len;
    gl_Position = vec4(b.xy, 0, 1);
    EmitVertex();

    g_uv_pw = vec2(uv_pw_height, uv_pw_width);
    g_uv = vec2(-uv_height, uv_width);
    g_len = uv_len;
    gl_Position = vec4(c.xy, 0, 1);
    EmitVertex();

    g_uv_pw = vec2(uv_pw_height, uv_pw_width);
    g_uv = vec2(uv_len + uv_height, uv_width);
    g_len = uv_len;
    gl_Position = vec4(d.xy, 0, 1);
    EmitVertex();
}