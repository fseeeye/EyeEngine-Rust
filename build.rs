// Compile GLSL/HLSL files in SPIR-V
// ref: https://github.com/sotrh/learn-wgpu/blob/0.7/docs/beginner/tutorial3-pipeline/README.md
// ref: https://doc.rust-lang.org/cargo/reference/build-scripts.html

use anyhow::*;
use glob::glob;
use std::fs::{read_to_string, write};
use std::path::PathBuf;

struct ShaderData {
    src: String, 
    src_path: PathBuf,
    spv_path: PathBuf,
    kind: shaderc::ShaderKind
}

impl ShaderData {
    // Load Shader from GLSL/HLSL file path
    pub fn load(src_path: PathBuf) -> Result<Self> {
        let extension = src_path
            .extension()
            .context(format!("File has no extension: {:?}", src_path))?
            .to_str()
            .context("Extension cannot be converted to `&str`")?;

        let kind = match extension {
            "vert" => shaderc::ShaderKind::Vertex,
            "frag" => shaderc::ShaderKind::Fragment,
            "comp" => shaderc::ShaderKind::Compute,
            _ => bail!("Unsupported shader extension: {}({})", src_path.display(), extension) 
        };

        let src = read_to_string(src_path.clone())?;
        let spv_path = src_path.with_extension(format!("{}.spv", extension)); // foo.[vert|frag|comp].spv

        Ok(Self {
            src,
            src_path,
            spv_path,
            kind
        })
    }
}

fn main() -> Result<()> {
    // Collect all shaders recursively within `/src`
    let mut shader_paths = [
        glob("./src/**/*.vert")?,
        glob("./src/**/*.frag")?,
        glob("./src/**/*.comp")?
    ];

    // Info: this could be parallelized
    let shaders = shader_paths
        .iter_mut()
        .flatten()
        .map(|glob_result| ShaderData::load(glob_result?))
        .collect::<Vec<Result<_>>>()
        .into_iter()
        .collect::<Result<Vec<_>>>()?;
    
    let mut compiler = shaderc::Compiler::new().context("Unable to create shader compiler")?;

    // Tips: This can't be parallelized. The [shaderc::Compiler] is
    // not thread safe. Also, it creates a lot of resources. You could
    // spawn multiple processes to handle this, but it would probably
    // be better just to only compile shaders that have been changed
    // recently.
    for shader in shaders {
        // This tells cargo to re-run this script if something in `/src` changes.
        println!("cargo:rerun-if-changed={}", shader.src_path.as_os_str().to_str().unwrap());
        println!("!!!");
        // Compile shader
        let compiled = compiler.compile_into_spirv(
            &shader.src,
            shader.kind,
            &shader.src_path.to_str().unwrap(), 
            "main", 
            None
        )?;
        // write into spv file
        write(shader.spv_path, compiled.as_binary_u8())?;
    }

    return Ok(())
}