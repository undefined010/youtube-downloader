use std::process::Command;
use eframe::egui;


fn main() -> Result<(), eframe::Error>{
    
    let opt = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([450.0,350.0]),..Default::default()
        
    };
    
    let app_name = "Youtube Downloader";
    let mut yt_url = String::from("");

    eframe::run_simple_native(app_name, opt, move |ctx , _frame| {

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Download a Youtube video");
            
            ui.horizontal(|ui| {
                ui.label("Video URL : ");
                ui.text_edit_singleline(&mut yt_url);
            });
            
            if ui.button("download").clicked() {
                
                println!("{}",yt_url);
                
                let _ = Command::new("python3")
                .arg("python-pytube/main.py")
                .arg(&yt_url)
                .spawn();              
            }   
        });
    })
    

}