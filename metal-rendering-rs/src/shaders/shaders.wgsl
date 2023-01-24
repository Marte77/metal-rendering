struct VertexInput { //contem os dados que passamos da CPU para a GPU, equivalente da struct que criamos Vertex no rust
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput { //contem o output do nosso vertex shader
    @builtin(position) //indica ao wgpu que este valor vai ser usado como clip coordinates (é o mesmo que o gl_Position do GLSL)
     clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex //marcamos como vertex para indicar que aqui é o ponto de entrada do vertex shader
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput; //se usar var tens que indicar o tipo da variável, com let o tipo é inferido mas passa a ser uma variavel imutavel
    //let x = f32(1 - i32(in_vertex_index)) * 0.5; //f32() e i32() são casts
    out.tex_coords = model.tex_coords;
    out.clip_position = vec4<f32>(model.position,1.0);

    return out;
}

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>; // uniforms
@group(0)@binding(1)
var s_diffuse: sampler;         // uniforms
@fragment //usamos da mesma maneira que o vertex, indicar que este e o ponto de entrada dos fragment shaders
fn fs_main(in: VertexOutput) -> @location(0) //indica ao wgpu que o vec4 retornado tem de ser guardado no primeiro color target
                                    vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}