
#![cfg_attr(windows, windows_subsystem = "windows")]

mod utils;
use std::time::Duration;

use utils::center_window;
use anyhow::Result;
use slint::{LogicalPosition, Timer,};


slint::include_modules!();
fn main() ->Result<(), slint::PlatformError>  {
    
    
    
    //let setting = SettingsWindow::new()?;
    /*
    let app = App::new()?;
    app.on_open_settings(|| {
        let settings = SettingsFrame::new().unwrap();
        // 如果需要在关闭时执行某些逻辑，可以注册 on_close 回调
        settings.on_close(|| {
            println!("Settings window closed");
        });
        settings.show().unwrap();
    }); */
    
    
    let main = MainWindow::new()?;

    let main_weak = main.as_weak();
    let handel = main.as_weak();
    main.on_close_window(move ||{
        handel.upgrade().unwrap().hide().unwrap();
    });

    let handel = main.as_weak();
    main.on_minimized_window(move |enable|{
        handel.upgrade().unwrap().window().set_minimized(enable);
    });

    let handel = main.as_weak();
    main.on_maximized_window(move |enable|{
        handel.upgrade().unwrap().window().set_maximized(enable);
    });

    let handel = main.as_weak();
    main.on_move_window(move |offset_x, offset_y|{
        let main = handel.upgrade().unwrap();
        let logical_pos = main.window().position().to_logical(main.window().scale_factor());
        main.window().set_position(LogicalPosition::new(logical_pos.x + offset_x, logical_pos.y + offset_y));
        
    });

   // let handel_setting = setting.as_weak();
    
    main.on_settings_btn_pressed(move ||{
    
    });
    main.on_account_btn_pressed(move ||{
        let url = "https://www.example.com";
        if webbrowser::open(url).is_ok() {
            println!("已在默认浏览器中打开链接: {}", url);
        } else {
            eprintln!("无法打开链接: {}", url);
        }
    });
    main.on_support_btn_pressed(move ||{
        let url = "https://www.example.com";
        if webbrowser::open(url).is_ok() {
            println!("已在默认浏览器中打开链接: {}", url);
        } else {
            eprintln!("无法打开链接: {}", url);
        }
    });
    Timer::single_shot(Duration::ZERO, move || {
        center_window(main_weak.unwrap().window());
    });
    
    main.run() 
   
   // setting.run()
}