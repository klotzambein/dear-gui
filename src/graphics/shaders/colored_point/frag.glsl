in vec2 g_uv;
in vec4 g_color;

out vec4 f_color;

uniform vec2 pixel_width;
uniform float width;

float line_alpha();

void main() {
    float alpha = line_alpha();
    f_color = vec4(g_color.rgb, g_color.a * alpha);
}

float line_alpha() {
    return 1.0 - length(g_uv.xy);
}