layout(points) in;
layout(triangle_strip, max_vertices = 4) out;

in vec2[1] v_vertex;
in vec2[1] v_size;
in int[1] v_texture_index;

out vec2 g_uv;

uniform mat3 model_transform;
uniform mat3 view_transform;

void main() {

    mat3 mv = view_transform * model_transform;

    vec2 v = v_vertex[0];
    vec2 s = v_size[0] / 2.0;
    vec3 a = mv * vec3(v + s * vec2(1, -1), 1);
    vec3 b = mv * vec3(v + s * vec2(1, 1), 1);
    vec3 c = mv * vec3(v + s * vec2(-1, -1), 1);
    vec3 d = mv * vec3(v + s * vec2(-1, 1), 1);

    float uv_size = 1.0 / 8.0;

    vec2 uv_origin = vec2(v_texture_index[0] % 8, 7 - v_texture_index[0] / 8) / 8.0;

    g_uv = uv_origin + vec2(uv_size, 0.0);
    gl_Position = vec4(a.xy, 0, 1);
    EmitVertex();
    
    g_uv = uv_origin + vec2(uv_size, uv_size);
    gl_Position = vec4(b.xy, 0, 1);
    EmitVertex();

    g_uv = uv_origin + vec2(0.0, 0.0);
    gl_Position = vec4(c.xy, 0, 1);
    EmitVertex();

    g_uv = uv_origin + vec2(0.0, uv_size);
    gl_Position = vec4(d.xy, 0, 1);
    EmitVertex();
}
