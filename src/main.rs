use ui::window::Window;
mod ui;

fn main() {
    let mut app_window = Window::new(10,20,22,20);
    app_window.resize(30, 20);
    println!("Hello, world!");
}
