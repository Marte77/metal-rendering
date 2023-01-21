//
//  Shaders.metal
//  metal-rendering
//
//  Created by Martinho Tavares Malhão on 21/01/2023.
//

#include <metal_stdlib>
#include "ShaderDefinitions.h"

using namespace metal;

struct VertexOut2D {
    float4 color,
        pos
            [[position]]; //usamos esta sintaxe para dizer ao rasterizer do Metal para usar este campo pos como as coordenadas do ecrã normalizadas (entre 0 e 1). é float4, mas usado para gráficos 2D, significa que a 3 coordenada indica a profundidade, que vai estar a 0 e a 4a coord indica onde guardar localizacoes (?)
    //https://donaldpinckney.com/metal/2018/07/05/metal-intro-1.html#writing-vertex-and-fragment-shader-code esta antes deste capitulo a explicacao do pos
};

vertex VertexOut2D vertexShader2D(const device //device indica o espaco de memoria que a gpu usa, device é um espaço read-write, constant é readonly. temos um vertexid diferente por cada vertice, logo é mais eficiente usar device
                         Vertex2D *vertexArray /*ponteiro para o nosso MTLBuffer*/
                            [[buffer(0)]], //esta sintaxe é especifica do Metal, estamos a dizer ao Metal que este vertexArray é onde devemos passar o primeiro buffer. o 0 corresponde ao 0 que usamos no index do renderEncoder.setVertexBuffer
                         unsigned int vid
                         [[vertex_id]] //semelhante ao buffer(0), dizemos que o vertex id vai para este parametro
) {
    //este codigo não muda nada nos vértices, chama-se um pass-through vertex shader, algo comum
    Vertex2D in = vertexArray[vid];
    VertexOut2D out;
    
    out.color = in.color;
    out.pos = float4(in.pos.x, in.pos.y, 0, 1); //nao precisamos de normalizar o x e y pq no swift ja demos os dados normalizados, damos 0 ao z pq nao tem profundidade sendo 2D
    return out;
}

//retorna float4 (equivalente ao vector_float4 do simd.h) pois corresponde aos dados da cor de cada vertice
fragment float4 fragmentShader2D(
                                 VertexOut2D interpolated [[stage_in]] //esta param indica ao Metal que os resultados do rasterizer tem de vir para aqui
) {
    return interpolated.color;
}
