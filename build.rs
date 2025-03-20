use std::{fs::File, io::BufWriter};


fn convert_png_to_ico(png_path: &str, ico_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 使用 image crate 读取 PNG 文件
    let img = image::open(png_path)?;
    // 转换为 RGBA8 格式
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();
    
    // 创建 IconImage，注意 ico crate 需要提供图像的宽、高和像素数据
    let icon_image = ico::IconImage::from_rgba_data(width as u32, height as u32, rgba.into_raw());
    
    // 创建一个图标目录，并添加图标条目
    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);
    // 先将 IconImage 转换为 IconDirEntry
    let entry = ico::IconDirEntry::encode(&icon_image)
        .expect("Icon 编码失败");
    icon_dir.add_entry(entry);
    
    // 写入 ICO 文件
    let file = File::create(ico_path)?;
    let mut writer = BufWriter::new(file);
    icon_dir.write(&mut writer)?;
    Ok(())
}
fn main() {
    #[cfg(target_os = "windows")]

    {
        let png_path = "./assets/icons/icon.png";
        let ico_path = "./assets/icons/icon.ico";
        convert_png_to_ico(png_path, ico_path).expect("图标转换失败");
        //embed_resource::compile("./assets/resource.rc", embed_resource::NONE).manifest_optional().unwrap();

        let mut res = winres::WindowsResource::new();
        res.set_icon("./assets/icons/icon.ico");
        res.compile().unwrap();

    }
    //slint_build::compile("./ui/settings_frame.slint").unwrap();
    slint_build::compile("./ui/main_window.slint").unwrap();
   // slint_build::compile("./ui/app.slint").unwrap();
   

    /*
    if let Ok(_) = slint_build::compile("ui/main_window.slint") {
        println!("cargo:warning=compile ui/main_window.slint success\n");
    } 
    if let Ok(_) = slint_build::compile("ui/settings_window.slint") {
        println!("cargo:warning=compile ui/settings_window.slint success\n");
        } */
}