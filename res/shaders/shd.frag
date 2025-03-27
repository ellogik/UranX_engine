#version 330 core
out vec4 FragColor;

in vec2 TexCoords;
in vec3 FragPos;
in vec3 Normal;

uniform sampler2D texture_diffuse1; // Основная текстура
uniform vec3 lightPos;  // Позиция света
uniform vec3 viewPos;   // Камера
uniform vec3 lightColor;

void main() {
    // Получаем цвет из текстуры
    vec3 texColor = texture(texture_diffuse1, TexCoords).rgb;

    // Освещение (ламбертово освещение)
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(lightPos - FragPos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * lightColor;

    // Итоговый цвет
    vec3 result = diffuse * texColor;
    FragColor = vec4(result, 1.0);
}
