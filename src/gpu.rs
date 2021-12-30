use wgpu::util::DeviceExt; // for `create_buffer_init`
use winit::{
    event::{WindowEvent, KeyboardInput, VirtualKeyCode, ElementState},
    window::Window
};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Zeroable, bytemuck::Pod)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3] // color space depends on `surface.get_preferred_format()`, mostly sRGB
}

impl Vertex {
    // get Vertex Layout
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            // stride defines how wide a vertex is.
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            // step mode tells the pipeline how often it should move to the next vertex.
            step_mode: wgpu::VertexStepMode::Vertex,  // specify wgpu::VertexStepMode::Instance if we only want to change vertices when we start drawing a new instance.
            // vertex attributes describe the individual parts of the vertex.
            attributes: &[
                // attribute: position
                wgpu::VertexAttribute {
                    // define the offset in bytes of this attribute startpoint.
                    offset: 0, 
                    // tell the shader what location to store this attribute at.
                    shader_location: 0, 
                    // tell the shader the shape of the attribute.
                    format: wgpu::VertexFormat::Float32x3
                },
                // attribute: color
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3
                }
            ]
            // attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3]
        }
    }
}

// vertex attribute data for Vertex Buffer
// tips: sRGB 0.2176 == RGB 0.5 (srgb_color = (rgb_color / 255) ^ 2.2)
const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.0868241, 0.49240386, 0.0], color: [0.2176, 0.0, 0.2176] }, // A
    Vertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.2176, 0.0, 0.2176] }, // B
    Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.2176, 0.0, 0.2176] }, // C
    Vertex { position: [0.35966998, -0.3473291, 0.0], color: [0.2176, 0.0, 0.2176] }, // D
    Vertex { position: [0.44147372, 0.2347359, 0.0], color: [0.2176, 0.0, 0.2176] }, // E
];

// indices data for Index Buffer
// tips: add 2 bytes padding as wgpu requires buffers to be aligned to 4 bytes.
// tips: We don't need to implement Pod and Zeroable for our indices, because bytemuck has already implemented them for basic types such as u16.
const INDICES: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
    /* padding */ 0,
];

pub(crate) struct GPUState {
    #[allow(dead_code)]
    instance: wgpu::Instance,
    surface: wgpu::Surface,
    #[allow(dead_code)]
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub(crate) size: winit::dpi::PhysicalSize<u32>,
    clear_color: wgpu::Color,
    render_pipeline: wgpu::RenderPipeline,
    render_pipeline_hidden: wgpu::RenderPipeline,
    show_hidden_color: bool,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    indices_num: u32
}

// ref: https://sotrh.github.io/learn-wgpu/beginner/tutorial2-surface/
impl GPUState {
    // Init, move Window Controlling power
    // tips: Creating some of the wgpu types requires async code
    pub(crate) async fn new(window: &Window) -> Self {
        /* Chore States */
        let size = window.inner_size(); // Get the size of the Window (excluding the title bar and borders)
        let clear_color = wgpu::Color { // default clear color
            r: 0.1,
            g: 0.2,
            b: 0.3,
            a: 1.0
        };

        /* Instace */
        // Create wgpu Instace, whose is a handle to our GPU to create Adapter(s) and Surface(s)
        let instance = wgpu::Instance::new(wgpu::Backends::all()); // Backens:all => Vulkan + Metal + DX12 + Browser WebGPU

        /* Surface */
        // Create wgpu Surface by winit Window, which is the part of the window that we can draw to.
        let surface = unsafe { instance.create_surface(&window) };

        /* Adapter */
        // Create wgpu Adapter, which is a handle to our actual grahics card.
        // You can use this to get information about the graphics card
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(), // LowPower or HighPerformance
                compatible_surface: Some(&surface), // tells wgpu to find an adapter that can present to the supplied surface.
                force_fallback_adapter: false, // whether forces wgpu to pick an adapter that will work on all hardware.
            }
        ).await.expect("No backends support current surface!");
        
        /* Device & Queue */
        // Create Device & (GPU's Render) Queue by Adapter
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::SPIRV_SHADER_PASSTHROUGH, // allows us to specify extra features. https://docs.rs/wgpu/0.12.0/wgpu/struct.Features.html
                limits: wgpu::Limits::default(), // describes the limit of certain types of resources that we can create. https://docs.rs/wgpu/0.12.0/wgpu/struct.Limits.html
                label: None
            }, 
            None    
        ).await.unwrap();
        
        /* Surface Configure */
        // This will define how the surface creates its underlying SurfaceTextures.
        let config = wgpu::SurfaceConfiguration {
            // describes how SurfaceTextures will be used:
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT, // "RENDER_ATTACHMENT" means the textures will be used to write to the screen
            // defines how SurfaceTextures will be stored on the gpu,
            // Different displays prefer different formats.
            format: surface.get_preferred_format(&adapter).unwrap(), // figure out the best format to use based on the display you're using.
            // width and the height in pixels of a SurfaceTexture,
            // This should usually be the width and the height of the window.
            // tips: make sure these are not 0, as that can cause your app to crash!
            width: size.width,
            height: size.height,
            // determines how to sync the surface with the display.
            // * Fifo
            // * VSync
            // https://docs.rs/wgpu/0.12.0/wgpu/enum.PresentMode.html
            present_mode: wgpu::PresentMode::Fifo
        };
        surface.configure(&device, &config);

        /* Pipeline */ 
        // Create "Pipeline Layout"
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[]
        });

        // Load "Shaders" - 1 (WGSL)
        let shader_module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/shader.wgsl").into())
        });
        let vertex_shader_ref = &shader_module;
        let fragment_shader_ref = &shader_module;
        let vertex_entry = "vs_main";
        let fragment_entry = "fs_main";
        // Load "Shaders" - 1 (GLSL/HLSL)
        //let vertex_shader_module = device.create_shader_module(&wgpu::include_spirv!("shaders/shader.vert.spv"));
        //let fragment_shader_module = device.create_shader_module(&wgpu::include_spirv!("shaders/shader.frag.spv"));
        //let vertex_shader_ref = &vertex_shader_module;
        //let fragment_shader_ref = &fragment_shader_module;
        //let vertex_entry = "main";
        //let fragment_entry = "main";
        
        // Create "Render Pipeline" - 1
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline - 1"),
            // setup Pipeline Layout
            layout: Some(&render_pipeline_layout),
            // setup Vertex Shader
            vertex: wgpu::VertexState {
                // specify the shader module
                module: vertex_shader_ref,
                // specify the entry point of vertex shader in shader file
                entry_point: vertex_entry,
                // layout of the vertices which we want to pass to the vertex shader
                buffers: &[ 
                    Vertex::desc()
                ],
            },
            // setup Fragment Shader
            // this is technically optional, so you have to wrap it in Some(). 
            // We need it if we want to store color data to the surface.
            fragment: Some(wgpu::FragmentState {
                module: fragment_shader_ref,
                // specify the entry point of fragment shader in shader file
                entry_point: fragment_entry,
                // tells wgpu what color outputs it should set up.
                // Currently, we only need one for the "Surface"
                targets: &[
                    wgpu::ColorTargetState { 
                        format: config.format,
                        blend: Some(wgpu::BlendState::REPLACE), // REPLACE : replace old pixel data with new data
                        write_mask: wgpu::ColorWrites::ALL // ALL: write all color channels (R G B A)
                    }
                ]
            }),
            // describes how to interpret our vertices when converting them into triangles.
            // == OpenGL Vertex Buffer Layout
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // TriangleList: each three vertices will correspond to one triangle.
                strip_index_format: None,
                // `front_face` & `cull_mode`: how to determine whether a given triangle is facing forward or not.
                front_face: wgpu::FrontFace::Ccw, // Ccw: triangle is facing forward if the vertices are arranged in a counter-clockwise direction.
                cull_mode: Some(wgpu::Face::Back), // Back: triangles that are not facing forward are culled (not included in the render)
                // tips: Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // tips: Enable requires Features::DEPTH_CLAMPING
                unclipped_depth: false,
                // tips: Enable requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None, // not using a depth/stencil buffer yet
            multisample: wgpu::MultisampleState {
                // how many samples the pipeline will use
                count: 1,
                // which samples should be active
                mask: !0, // !0 means using all of them
                alpha_to_coverage_enabled: false,
            },
            multiview: None // ?
        });

        // Load "Shaders" - 2 (WGSL)
        let shader_module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader2"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/shader2.wgsl").into())
        });
        let vertex_shader_ref = &shader_module;
        let fragment_shader_ref = &shader_module;
        let vertex_entry = "vs_main";
        let fragment_entry = "fs_main";
        // Load "Shaders" - 2 (GLSL/HLSL)
        //let vertex_shader_module = device.create_shader_module(&wgpu::include_spirv!("shaders/shader2.vert.spv"));
        //let fragment_shader_module = device.create_shader_module(&wgpu::include_spirv!("shaders/shader2.frag.spv"));
        //let vertex_shader_ref = &vertex_shader_module;
        //let fragment_shader_ref = &fragment_shader_module;
        //let vertex_entry = "main";
        //let fragment_entry = "main";
        
        // Create "Render Pipeline" - 2
        let render_pipeline_hidden = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline - Hidden"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: vertex_shader_ref,
                entry_point: vertex_entry, 
                buffers: &[
                    Vertex::desc()
                ], 
            },
            fragment: Some(wgpu::FragmentState { 
                module: fragment_shader_ref,
                entry_point: fragment_entry,
                targets: &[
                    wgpu::ColorTargetState { 
                        format: config.format,
                        blend: Some(wgpu::BlendState::REPLACE), 
                        write_mask: wgpu::ColorWrites::ALL
                    }
                ]
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, 
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, 
                cull_mode: Some(wgpu::Face::Back), 
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None
        });

        let show_hidden_color = false;

        /* Vertex Buffer */
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES), // cast VERTICES to &[u8]
                usage: wgpu::BufferUsages::VERTEX
            }
        );

        /* Index Buffer */
        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(INDICES), // cast VERTICES to &[u8]
                usage: wgpu::BufferUsages::INDEX
            }
        );
        let indices_num = INDICES.len() as u32;

        Self {
            instance,
            surface,
            adapter,
            device,
            queue,
            config,
            size,
            clear_color,
            render_pipeline,
            render_pipeline_hidden,
            show_hidden_color,
            vertex_buffer,
            index_buffer,
            indices_num
        }
    }

    // resize Window
    pub(crate) fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        // size 0 will cause your app to crash!
        if new_size.width != 0 && new_size.height != 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;

            self.surface.configure(&self.device, &self.config);
        }
    }

    // indicate whether an event has been fully processed.
    // If the method returns true, the main loop won't process the event any further.
    // So the main idea of this function is catching some specific events and handle them in it.
    pub(crate) fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.clear_color = wgpu::Color {
                    r: position.x as f64 / self.size.width as f64,
                    g: position.y as f64 / self.size.height as f64,
                    b: 1.0,
                    a: 1.0
                };
                true
            },
            WindowEvent::KeyboardInput {
                input: KeyboardInput {
                    state,
                    virtual_keycode: Some(VirtualKeyCode::Space),
                    ..
                },
                ..
            } => {
                self.show_hidden_color = *state == ElementState::Pressed;
                true
            },
            _ => false
        }
    }

    pub(crate) fn update(&mut self) {
        // todo!()
        // We don't have anything to update yet
    }

    pub(crate) fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // get a frame(桢) to render to.
        // wait Surface to provide a new SurfaceTexture that we will render to
        let output_texture = self.surface.get_current_texture()?;
        // Create "TextureView" with default settings,
        // so that we can control how the render code interacts with the texture.
        let texture_view = output_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());
        // Create "CommandEncoder" to create the actual commands to send to the gpu and builds a command buffer to store them.
        // Most modern graphics frameworks expect commands to be stored in a command buffer before being sent to the gpu.
        let mut command_encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder")
        });
        {
            // Create a "RenderPass" by CommandEncoder.
            // It has all the methods for the actual drawing.
            let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                // describe where we are going to draw our color to
                // This is what [[location(0)]] in the fragment shader targets
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    // `view` field informs wgpu what texture to save the colors to.
                    // here we use the TextureView to make sure that we render to the screen.
                    view: &texture_view,
                    // it's the texture that will receive the resolved output.
                    // this will be the same as `view` field texture unless multisampling is enabled,
                    // so we don't need to store this texture currently.
                    resolve_target: None,
                    // This tells wgpu what to do with the colors on the screen (specified by frame.view)
                    ops: wgpu::Operations {
                        // tells wgpu how to handle colors stored from the previous frame.
                        load: wgpu::LoadOp::Clear(self.clear_color),
                        // tells wgpu whether we want to store the rendered results to the Texture behind our TextureView
                        // in this case, it's the SurfaceTexture.
                        store: true
                    }
                }],
                depth_stencil_attachment: None
            });

            // specify Render Pipeline to current RenderPass
            render_pass.set_pipeline(
                if self.show_hidden_color {
                    &self.render_pipeline_hidden
                } else {
                    &self.render_pipeline
                }
            );
            // send Vertex Buffer data to current RenderPass
            // tips: we could set multiple vertex buffer to a render pass
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..)); // send entire data in vertex_buffer to slot 0
            // send Index Buffer to current RenderPass
            // tips: we only could set one index buffer to a render pass
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            // Draw Call: send vertex index & instance id to wgpu
            render_pass.draw_indexed(0..self.indices_num, 0, 0..1);
        }

        // finish the command buffer, and to submit it to the GPU's render queue
        self.queue.submit(std::iter::once(command_encoder.finish()));
        output_texture.present();

        Ok(())
    }
}