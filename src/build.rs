use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    // 告诉 Cargo 如果 assets/icon.png 变化则重新运行脚本
    println!("cargo:rerun-if-changed=assets/icon.png");

    // 目标源文件路径：放在 src/icon_data.rs
    let dest_path = Path::new("src").join("icon_data.rs");

    // 加载原始图片
    let img = image::open("assets/icon.png").expect("Failed to open icon.png");

    // 目标尺寸 (16, 32, 64)
    let sizes = [16, 32, 64];

    // 准备存储每个尺寸的数组
    let mut arrays = Vec::new();

    for &size in &sizes {
        // 调整图片大小到 size x size
        let resized =
            image::imageops::resize(&img, size, size, image::imageops::FilterType::Lanczos3);

        // 将 RGBA 像素展平为字节数组
        let mut pixels = Vec::with_capacity((size * size * 4) as usize);
        for y in 0..size {
            for x in 0..size {
                let pixel = resized.get_pixel(x, y);
                pixels.push(pixel[0]); // R
                pixels.push(pixel[1]); // G
                pixels.push(pixel[2]); // B
                pixels.push(pixel[3]); // A
            }
        }

        arrays.push((size, pixels));
    }

    // 生成 Rust 代码
    let mut code = String::new();
    code.push_str("// Auto-generated icon data\n");
    code.push_str("use crate::miniquad::conf::Icon;\n\n");

    for (size, pixels) in &arrays {
        let array_len = size * size * 4;
        code.push_str(&format!("const ICON_{}: [u8; {}] = [\n", size, array_len));
        for (i, &byte) in pixels.iter().enumerate() {
            if i % 16 == 0 {
                code.push_str("    ");
            }
            code.push_str(&format!("{}, ", byte));
            if (i + 1) % 16 == 0 {
                code.push('\n');
            }
        }
        code.push_str("];\n\n");
    }

    code.push_str("pub const ICON: Option<Icon> = Some(Icon {\n");
    code.push_str(&format!("    small: ICON_16,\n"));
    code.push_str(&format!("    medium: ICON_32,\n"));
    code.push_str(&format!("    big: ICON_64,\n"));
    code.push_str("});\n");

    // 写入文件到 src/icon_data.rs
    let mut f = File::create(&dest_path).unwrap();
    f.write_all(code.as_bytes()).unwrap();
    
    //windows icon
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
    	embed_resource::compile("assets/resources.rc", embed_resource::NONE).manifest_optional().unwrap();
    }
}
