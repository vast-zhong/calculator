extern crate meval;

use eframe::egui::{self, Color32, Painter, Pos2, Rect, Vec2};

// Draw the result block
// display is the text content we want to show
fn result_block(ui: &mut egui::Ui, display: &str) {
    // get the full rect of the UI area
    let full_rect = ui.max_rect();
    // the size of the result block, width: full rect width, height: 1/5 full rect height
    let block_size = Vec2::new(full_rect.width(), full_rect.height() / 5.0);
    // create the result block rect, full_rect.min: left-up as start point
    let block_rect = Rect::from_min_size(full_rect.min, block_size);
    // use ui.painter to draw the result block
    let painter: &Painter = ui.painter();
    // draw the background color
    painter.rect_filled(
        block_rect, 
        5.0,  
        Color32::from_rgb(20, 26, 34), // #141A22
    );
    // draw the text
    painter.text(
        block_rect.center(),
        egui::Align2::CENTER_CENTER,
        display,
        egui::FontId::proportional(24.0),
        Color32::WHITE,
    );
}

// Draw the input block
fn input_block(ui: &mut egui::Ui) -> Option<String> {
    let full_rect = ui.max_rect();
    let mut clicked_button = None;

    let result_h = full_rect.height() / 5.0;
    // get the start position of the input block
    let top_left = Pos2::new(full_rect.min.x, full_rect.min.y + result_h);
    // the size of the input block, width: full rect width, height: 4/5 full rect height
    let block_size = Vec2::new(full_rect.width(), full_rect.height() - result_h);
    let block_rect = Rect::from_min_size(top_left, block_size);
    // then draw the input block
    let painter: &Painter = ui.painter();
    painter.rect_filled(
        block_rect, 
        5.0,  
        Color32::from_rgb(42, 48, 58), // #2A303A
    );
    // the size of each cell in input block
    let cell_w = block_rect.width() / 4.0;
    let cell_h = block_rect.height() / 5.0;
    // the gap between each cell
    let gap = 4.0; 
    // each cell in the input block
    let labels = [
        ["+/-", "AC", "DEL", "/"],
        ["7", "8", "9", "*"],
        ["4", "5", "6", "-"],
        ["1", "2", "3", "+"],
        ["%", "0", ".", "="],
    ];

    // double loop to draw each cell
    // every row
    for (row_i, row) in labels.iter().enumerate() {
        // every col
        for (col_i, &text) in row.iter().enumerate() {
            // get the position of the cell
            let x = block_rect.min.x + col_i as f32 * cell_w;
            let y = block_rect.min.y + row_i as f32 * cell_h;
            // create the button rect
            // 1/2 gap is left on the left/right side of the button, sum is gap 
            let button_rect = Rect::from_min_size(
                Pos2::new(x + gap / 2.0, y + gap / 2.0), 
                Vec2::new(cell_w - gap, cell_h - gap)
            );
            // check the interaction state of the button
            // egui::Sense::click() means the button can be clicked
            let response = ui.interact(button_rect, ui.id().with((row_i, col_i)), egui::Sense::click());
            
            // animation effect cal
            let is_pressed = response.is_pointer_button_down_on();
            let animation_progress = if is_pressed { 1.0 } else { 0.0 };
            
            // color animation effect
            // change the color of the button
            // change the color of the button when it is pressed, plus 20
            let animated_color = Color32::from_rgb(
                42 + (20.0 * animation_progress) as u8,
                48 + (20.0 * animation_progress) as u8, 
                58 + (20.0 * animation_progress) as u8
            );
            
            // Change the size of the button
            // Zoom animation effect
            let scale_factor = 1.0 - 0.05 * animation_progress;
            let animated_rect = Rect::from_center_size(
                button_rect.center(),
                button_rect.size() * scale_factor
            );
            
            // Check if the button is clicked
            // Store the text of the clicked button
            if response.clicked() {
                clicked_button = Some(text.to_string());
            }
            
            // draw the button background
            painter.rect_filled(
                animated_rect,
                egui::CornerRadius::same(8),
                animated_color,
            );
            
            // draw the button border
             painter.rect_stroke(
                 animated_rect,
                 egui::CornerRadius::same(8),
                 egui::Stroke::new(0.4, Color32::GRAY),
                 egui::StrokeKind::Inside,
             );
            
            // draw the button text
            painter.text(
                animated_rect.center(),
                egui::Align2::CENTER_CENTER,
                text,
                egui::FontId::proportional(20.0),
                Color32::from_rgb(255, 255, 255),
            );
        }
    }
    
    // return the text of the clicked button
    clicked_button
}


fn main() -> Result<(), eframe::Error> {
    // create a window
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 500.0]),
        ..Default::default()
    };

    // run egui app
    eframe::run_native(
        "TinyCalc", // title
        options, 
        Box::new(|cc| {
            // return eframe::App trait
            Ok(Box::new(TinyCalc::new(cc)))
        }),
    )
}

struct TinyCalc {
    // the result we display
    display: String,
    // the expression
    expression: String,
    // calculated
}

//TinyCalc struct new function
impl TinyCalc {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {        
        Self {
            display: "0".to_string(),
            expression: String::new(),
        }
    }

    fn handle_button(&mut self, button: &str) {

    }
}

impl eframe::App for TinyCalc {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            result_block(ui, &self.display);
            if let Some(button) = input_block(ui) {
                self.handle_button(&button);
            }
        });
    }
}