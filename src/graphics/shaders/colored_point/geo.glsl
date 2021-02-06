layout(points) in;
layout(triangle_strip, max_vertices = 4) out;

in vec2[1] v_vertex;
in vec4[1] v_color;

out vec2 g_uv;
out vec4 g_color;

uniform vec2 pixel_width;
uniform float aspect_ratio;
uniform float width;
uniform mat3 model_transform;
uniform mat3 view_transform;

void main() {

    float half_width = width / 2;
    vec3 vertex = model_transform * vec3(v_vertex[0], 1);
    vec3 end = vertex;

    vec3 p_vertex = view_transform * vertex;

    vec3 tangent = vec3(1, 0, 0);
    vec3 normal = vec3(0, 1, 0);

    vec3 p_tangent = view_transform * tangent;
    vec3 p_normal = view_transform * normal;

    vec3 pn_tangent = normalize(p_tangent);
    vec3 pn_normal = normalize(p_normal);


    vec3 a = p_vertex + (p_normal - p_tangent) * half_width + (pn_normal - pn_tangent) * vec3(pixel_width, 1);
    vec3 b = p_vertex + (p_normal + p_tangent) * half_width + (pn_normal + pn_tangent) * vec3(pixel_width, 1);
    vec3 c = p_vertex + (-p_normal - p_tangent) * half_width + (-pn_normal - pn_tangent) * vec3(pixel_width, 1);
    vec3 d = p_vertex + (-p_normal + p_tangent) * half_width + (-pn_normal + pn_tangent) * vec3(pixel_width, 1);

    float uv_height = 1;
    float uv_width = 1;
    
    g_color = v_color[0];
    g_uv = vec2(-uv_height, -uv_width);
    gl_Position = vec4(a.xy, 0, 1);
    EmitVertex();
    
    g_color = v_color[0];
    g_uv = vec2(uv_height, -uv_width);
    gl_Position = vec4(b.xy, 0, 1);
    EmitVertex();

    g_color = v_color[0];
    g_uv = vec2(-uv_height, uv_width);
    gl_Position = vec4(c.xy, 0, 1);
    EmitVertex();

    g_color = v_color[0];
    g_uv = vec2(uv_height, uv_width);
    gl_Position = vec4(d.xy, 0, 1);
    EmitVertex();
}
