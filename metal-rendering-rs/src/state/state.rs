use crate::util::vertex::*;
use crate::util::texture;
//use log::{debug, error, info, log_enabled, warn, Level};
use wgpu::util::DeviceExt;
use winit::dpi::PhysicalSize;
use winit::{
    dpi::PhysicalPosition,
    event::{WindowEvent},
    window::Window,
};

pub struct State {
    surface: wgpu::Surface,             //representa a nossa janela
    device: wgpu::Device,               //representa a placa grafica encarregue de renderizar
    queue: wgpu::Queue,                 //lista dos comandos que estao para ser executados na gpu
    config: wgpu::SurfaceConfiguration, //é a parte da janela onde desenhamos as coisas
    pub size: PhysicalSize<u32>,        //tamanho da janela em pixeis
    window: Window,                     //janela do programa
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    diffuse_bind_group: wgpu::BindGroup,
    diffuse_texture: texture::Texture,

    num_vertices: u32,
    mouse_position: Option<PhysicalPosition<f64>>,
}
const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.0868241, 0.49240386, 0.0], tex_coords: [0.4131759,  0.99240386], }, // A
    Vertex { position: [-0.49513406, 0.06958647, 0.0], tex_coords: [0.0048659444, 0.56958647], }, // B
    Vertex { position: [-0.21918549, -0.44939706, 0.0], tex_coords: [0.28081453, 1.0 -  0.05060294], }, // C
    Vertex { position: [0.35966998, -0.3473291, 0.0], tex_coords: [0.85967,  1.0 - 0.1526709], }, // D
    Vertex { position: [0.44147372, 0.2347359, 0.0], tex_coords: [0.9414737, 1.0 - 0.7347359], }, // E
];
const INDICES: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
];
impl State {
    pub async fn new(window: Window) -> Self {
        let size = window.inner_size();

        // instance é a primeira coisa que criamos fornecendo-lhe os backends que queremos suportar, nao tem de viver durante a execucao toda do programa. cria o Adapter e Surface
        let instance = wgpu::Instance::new(wgpu::Backends::all());

        // esta superficie, ao contrario da instance, tem de viver durante o programa todo
        let surface = unsafe { instance.create_surface(&window) };

        // adapter vai ser uma especie de wrapper da gpu, da nos acesso a varias infos como o nome da gpu, que backend e utilizado, etc.
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance, //existe LowPower() e HighPerformance(). se nao for especificado o wgpu escolhe LowPower, que provavelmente será uma gpu integrada. dando default() presumo que seja o OS a escolher ou entao LowPower()
                compatible_surface: Some(&surface), //diz ao wgpu para encontrar um adapter que possa desenhar na surface dada
                force_fallback_adapter: false, //forces wgpu to pick an adapter that will work on all hardware. This usually means that the rendering backend will use a "software" system, instead of hardware such as a GPU.
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(), // certas features que o wgpu expoe, aqui https://docs.rs/wgpu/latest/wgpu/struct.Features.html a maioria sao complexas demais para um projeto simples como este
                    limits: if cfg!(target_arch = "wasm32") {
                        // o webgl nao suporta todas as coisas que o wgpu suporta, entao damos limites ao dispositivo caso estejamos num ambiente wasm, https://docs.rs/wgpu/latest/wgpu/struct.Limits.html
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: Some("Device"),
                },
                None,
            )
            .await
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT, // indica ao wgpu como e que as texturas sao usadas ao escrever no ecra
            format: surface.get_supported_formats(&adapter)[0], // diz como  eque as texturas vao ser guardadas na surface, isto retorna um vetor com varias maneiras de guardar e normalmente o primeiro é a maneira preferível
            width: size.width,                                  //se forem 0 pode crashar a app
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo, //determina como e que a surface sincroniza com o ecra, usando Fifo fica ligado a refresh rate do ecra aka VSync
            // esta funcao em baixo retorna os modos suportados pelo dispositivo caso queremos mudar durante a execucao do programa
            //let modes = surface.get_supported_modes(&adapter);
            //para mudarmos basta alterar este campo do config e chamar a funcao surface.configure(&device, &config)
            alpha_mode: wgpu::CompositeAlphaMode::Auto, // opacidade das texturas
        };
        surface.configure(&device, &config);

        let bytes = match attohttpc::get("https://cdn.discordapp.com/attachments/412661004527206401/1067553603792941077/AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA.jpg").send() {
            Err(e) => panic!("erro download imagem {:?}", e),
            Ok(r) => match r.bytes() {
                Err(e) => panic!("erro bytes {:?}", e),
                Ok(b) => b,
            },
        };

        let diffuse_texture = texture::Texture::from_bytes(&device, &queue, &bytes, "happy-tree.png").unwrap(); // CHANGED!

        //BindGroup descreve um conjunto de recursos e como é que estes devem ser acedidos pelo shader
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,//estas bindings sao apenas visiveis pelo fragment shader
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        // This should match the filterable field of the, orresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        // com isto podemos mudar o bindgroup durante o runtime desde que o bindgrouplayout seja igual
        let diffuse_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                    }
                ],
                label: Some("diffuse_bind_group"),
            }
        );


        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/shaders.wgsl").into()), // existe a macro include_wgsl! -> device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main", // especificamos qual e a funcao de entrada do vertex shader
                buffers: &[
                    Vertex::descriptor()
                ], // indica o formato/tipo dos vertices que vamos dar ao vertex shader
            },
            fragment: Some(wgpu::FragmentState {
                // o fragment shader tecnicamente nao e necessario ent metemos num Some()
                module: &shader,
                entry_point: "fs_main", // igual ao vertex
                targets: &[Some(wgpu::ColorTargetState {//targets dizem ao wgpu que outputs de cor é que devem ser usados
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // com isto indicamos que para cada 3 vertices, se desenha um triangulo
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // diz ao wgpu como determinar se o triangulo esta de frente. FrontFace::Ccw diz que se os vertices estiver colocados em ordem reversa do relogio o triangulo e considerado estar de frente
                cull_mode: Some(wgpu::Face::Back), // triangulos que nao encaixam no front face sao "culled", dando Face::Back nao sao considerados
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1, // determina quantas samples a pipeline vai usar, colocamos 1 pq nao estamos em multisampling
                mask: !0, // indica quantas samples vao ser usadas
                alpha_to_coverage_enabled: true, // refere-se a anti aliasing
            },
            multiview: None, // quantos array layers um attachement deve ter, nao vamos usar logo None
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX
        });

        let index_buffer = device.create_buffer_init( //index buffer é utilizado para reutilizarmos vertices para desenhar os triangulos sem termos de estar a repetir triangulos e aumentando imensamente a memoria utilizada
            &wgpu::util::BufferInitDescriptor{
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: wgpu::BufferUsages::INDEX
            }
        );
        let num_indices = INDICES.len() as u32;

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            diffuse_bind_group,
            diffuse_texture,

            mouse_position: None,
            num_vertices:VERTICES.len() as u32
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    //se quisermos capturar um evento retornamos true, se nao retornamos falso e o evento e enviado para outros sitios aka funcao run
    pub fn input(&mut self, event: &WindowEvent) -> bool {
        //info!("the answer was: {:?}", event);
        match event {
            #[allow(unused_variables, deprecated)]
            WindowEvent::CursorMoved {
                device_id,
                position,
                modifiers,
            } => {
                self.mouse_position = Some(position.clone());
                true
            }
            _ => false,
        }
    }

    pub fn update(&mut self) {}

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?; // esta funcao é blocking, vai esperar que a surface retorne a SurfaceTexture

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default()); // criamos a TextureView para podermos fazer com que o nosso codigo interaja com a textura

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mouse_pos = //self
                //.mouse_position
                //.unwrap_or(
                    PhysicalPosition { x: 0.9, y: 0.3 }
                //)
            ;

            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    // o @location(0) no fragment shaders vai escrever para aqui
                    view: &view, //informa o wgpu sobre  qual textura (a nossa view) vai receber as cores que damos. dando a nossa view escrevemos no ecra
                    resolve_target: None, // vai ser a textura que recebe as nossas cores, é o mesmo que a nossa view visto que nao temos multisampling ativo logo passamos None
                    ops: wgpu::Operations {
                        // indica a wgpu o que fazer com as cores, neste caso passamos uma cor estatica
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            //este load diz o que fazer com a frame anterior, usando a operacao clear estamos a apagar a frame anterior, se nao vai repetir
                            r: 0.0,
                            g: mouse_pos.x / self.window.inner_size().width as f64,
                            b: mouse_pos.y / self.window.inner_size().height as f64,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None, // diz onde e que vamos desenhar a nossa cor definida nos attachements
            });

            render_pass.set_pipeline(&self.render_pipeline); //definimos a pipelne para usar a que nos criamos
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            render_pass.set_vertex_buffer(0,self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1); // dizer para desenhar x vertices e 1 instancia, é daqui que o @builtin(vertex_index) vem. usando indices, passamos a usar draw_indexed onde passamos o numero de indices
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
