#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
// esta struct so usa f32, mas se tivesse algum tipo de dados que nao implemente Pod ou Zeroable temos de fazer com que eles os implementem `unsafe impl bytemuck::Pod for Vertex {}`
pub struct VertexColor {
    pub position: [f32;3],
    pub color: [f32;3]
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32;3],
    pub tex_coords: [f32;2]
}

impl Vertex {
    pub fn descriptor<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress, //indica o quao grande e a estrutura Vertex para quando o shader for ler o proximo vertice saber quantos bytes salta
            step_mode: wgpu::VertexStepMode::Vertex,//diz a pipeline se cada elemento do array representa per-vertex data ou per-instance. Se quisermos mudar os vertices apenas quando desenhamos uma instance nova, devemos passar ::Instance em vez de ::Vertex
            attributes: &[ // atributos descrevem as partes individuais da nossa struct Vertex, uma representacao 1:1 da struct
                wgpu::VertexAttribute {
                    offset: 0,//sendo o primeiro atributo tem offset 0, no de baixo metemos o tamanho dum vec<f32> com 3 posicoes
                    shader_location: 0, //diz onde e que o shader vai guardar este atributo (semelhatne a uniforms do opengl). @location(0) corresponde ao Vertex.position enquanto que @location(1) corresponde ao Vertex.color
                    format: wgpu::VertexFormat::Float32x3, //vec3<f32> tamanho do array posicao
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                }
            ]
        }
    }
}



impl VertexColor {
    pub fn descriptor<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<VertexColor>() as wgpu::BufferAddress, //indica o quao grande e a estrutura Vertex para quando o shader for ler o proximo vertice saber quantos bytes salta
            step_mode: wgpu::VertexStepMode::Vertex,//diz a pipeline se cada elemento do array representa per-vertex data ou per-instance. Se quisermos mudar os vertices apenas quando desenhamos uma instance nova, devemos passar ::Instance em vez de ::Vertex
            attributes: &[ // atributos descrevem as partes individuais da nossa struct Vertex, uma representacao 1:1 da struct
                wgpu::VertexAttribute {
                    offset: 0,//sendo o primeiro atributo tem offset 0, no de baixo metemos o tamanho dum vec<f32> com 3 posicoes
                    shader_location: 0, //diz onde e que o shader vai guardar este atributo (semelhatne a uniforms do opengl). @location(0) corresponde ao Vertex.position enquanto que @location(1) corresponde ao Vertex.color
                    format: wgpu::VertexFormat::Float32x3, //vec3<f32> tamanho do array posicao
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                }
            ]
        }
    }
}
