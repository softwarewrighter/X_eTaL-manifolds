fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<sphere_hello_web::app::App>::new().render();
}
