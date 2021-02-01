in vec2 g_uv;
in vec2 g_uv_pw;
in vec4 g_color;
in float g_len;

out vec4 f_color;

uniform vec2 pixel_width;
uniform float width;

float line_alpha();

void main() {
    float alpha = line_alpha();
    f_color = vec4(g_color.rgb, g_color.a * alpha);
}

float line_alpha() {
    float y = g_uv.y;
    if (g_uv.x < 0.0) {
        float x = g_uv.x;
        if (y * y + x * x < 1.0)
            return 1.0;
        // TODO: anti-alias corner
        return 0.0;
    } else if (g_uv.x > g_len) {
        float x = g_uv.x - g_len;
        if (y * y + x * x < 1.0)
            return 1.0;
        // TODO: anti-alias corner
        return 0.0;
    } else {
        float dist = abs(y);
        if (dist < 1.0)
            return 1.0;
        if (dist - 1.0 < g_uv_pw.y)
            return 1.0 - (dist - 1.0) / g_uv_pw.y;
        return 0.0;
    }

}