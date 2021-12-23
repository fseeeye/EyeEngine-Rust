use winit::{
    event::WindowEvent,
    window::Window
};

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
    render_pipeline: wgpu::RenderPipeline
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
        // Create wgpu Instace, whose main purpose is to create Adapter(s) and Surface(s)
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
        // Load Shaders (WGSL)
        let shader_module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into())
        });
        let vertex_shader_ref = &shader_module;
        let fragment_shader_ref = &shader_module;
        let vertex_entry = "vs_main";
        let fragment_entry = "fs_main";
        // Load Shaders (GLSL/HLSL)
        //let vertex_shader_module = device.create_shader_module(&wgpu::include_spirv!("shader.vert.spv"));
        //let fragment_shader_module = device.create_shader_module(&wgpu::include_spirv!("shader.frag.spv"));
        //let vertex_shader_ref = &vertex_shader_module;
        //let fragment_shader_ref = &fragment_shader_module;
        //let vertex_entry = "main";
        //let fragment_entry = "main";
        // Create "Pipeline Layout"
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[]
        });
        // Create "Render Pipeline"
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            // setup vertex shader
            vertex: wgpu::VertexState {
                module: vertex_shader_ref,
                entry_point: vertex_entry, // specify the entry point of vertex shader in shader file
                buffers: &[], // what type of vertices we want to pass to the vertex shader
            },
            // setup fragment shader
            // this is technically optional, so you have to wrap it in Some(). 
            // We need it if we want to store color data to the surface.
            fragment: Some(wgpu::FragmentState { 
                module: fragment_shader_ref,
                entry_point: fragment_entry,
                // tells wgpu what color outputs it should set up.
                // Currently, we only need one for the "Surface"
                targets: &[wgpu::ColorTargetState { 
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE), // REPLACE : replace old pixel data with new data
                    write_mask: wgpu::ColorWrites::ALL // ALL: write to all colors (R G B A)
                }]
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
                // tips: Requires Features::DEPTH_CLAMPING
                unclipped_depth: false,
                // tips: Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None, // not using a depth/stencil buffer yet
            multisample: wgpu::MultisampleState {
                // how many samples the pipeline will use
                count: 1,
                // which samples should be active
                mask: !0, // !0 means using all of them
                alpha_to_coverage_enabled: false, // not covering anti-aliasing yet
            },
            multiview: None // ?
        });

        Self {
            instance,
            surface,
            adapter,
            device,
            queue,
            config,
            size,
            clear_color,
            render_pipeline
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
            }
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

            // set specific Pipeline to RenderPass
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.draw(0..3, 0..1);
        }

        // finish the command buffer, and to submit it to the GPU's render queue
        self.queue.submit(std::iter::once(command_encoder.finish()));
        output_texture.present();

        Ok(())
    }
}