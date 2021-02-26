in vec2 g_uv;

out vec4 f_color;

uniform sampler2D sprite_texture;

void main() {
    f_color = texture(sprite_texture, g_uv);
}