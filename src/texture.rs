use image::GenericImageView;
use anyhow::Result;

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler
}

impl Texture {
    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        label: Option<&str>
    ) -> Result<Self> {
        let img = image::load_from_memory(bytes)?;
        Self::from_image(device, queue, &img, label)
    }

    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &image::DynamicImage,
        label: Option<&str>
    ) -> Result<Self> {
        let rgba = img.as_rgba8().unwrap(); // convert image into Vec of RGBA bytes.
        let dimensions = img.dimensions(); // get width and height of this image.

        let texutre_size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            // All textures are stored as 3D, we represent our 2D texture by setting depth to 1.
            depth_or_array_layers: 1,
        };
        // Create "Texture"
        let texture = device.create_texture(
            &wgpu::TextureDescriptor {
                label,
                size: texutre_size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                // Most images are stored using sRGB so we need to set that here.
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                // TEXTURE_BINDING : tells wgpu that we want to use this texture in shaders
                // COPY_DST : means that we want to copy data to this texture
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            }
        );

        // Load data into "Texture"
        queue.write_texture(
            // where to copy the pixel data
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            // the actual pixel data
            rgba, 
            // the layout of the texture
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * dimensions.0),
                rows_per_image: std::num::NonZeroU32::new(dimensions.1)
            },
            texutre_size
        );

        // Create "Texture View": offser a view into our Texture.
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        // Create "Sampler" for Texture: control how the Texture is sampled.
        // app supplies a "Texture Coordinate" and the sampler return the corresponding color based on the texture and some internal parameters.
        let sampler = device.create_sampler(
            &wgpu::SamplerDescriptor {
                // address_mode_* :
                // determine what to do if the sampler gets a texture coordinate that's outside the texture itself
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                // describe what to do when a fragment covers multiple pixels or there are multiple fragments for a single pixel,
                // this often comes into play when viewing a surface from up close, or from far away.
                // `Linear`: Attempt to blend the in-between fragments so that they seem to flow together.
                // `Nearest`: In-between fragments will use the color of the nearest pixel. 
                //      This creates an image that's crisper from far away, but pixelated up close. 
                //      This can be desirable, however, if your textures are designed to be pixelated, 
                //      like in pixel art games, or voxel games like Minecraft.
                mag_filter: wgpu::FilterMode::Linear, // how to filter the texture when it needs to be magnified (made larger)
                min_filter: wgpu::FilterMode::Nearest, // how to filter the texture when it needs to be minified (made smaller)
                // how to blend between mipmaps.
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            }
        );

        Ok(Self {
            texture,
            view: texture_view,
            sampler
        })
    }

    // Depth Format for creating the depth stage of the render_pipeline and the depth texture itself.
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    pub fn create_depth_texture(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, label: &str) -> Self {
        // create Texture
        let size = wgpu::Extent3d {
            // depth texture needs to be the same size as our screen
            width:  config.width,
            height: config.height,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(
            &wgpu::TextureDescriptor {
                label: Some(label),
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: Self::DEPTH_FORMAT,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT //  we need render to this texture
                    | wgpu::TextureUsages::TEXTURE_BINDING,
            }
        );

        // create Texture View
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        // create Texture Sampler
        let sampler = device.create_sampler(
            &wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::FilterMode::Nearest,
                // If we do decide to render our depth texture, we need to use CompareFunction::LessEqual
                // This is due to how the samplerShadow and sampler2DShadow() interacts with the texture() function in GLSL.
                compare: Some(wgpu::CompareFunction::LessEqual), 
                lod_min_clamp: -100.0,
                lod_max_clamp: 100.0,
                ..Default::default()
            }
        );

        return Self { texture, view, sampler }
    }
}