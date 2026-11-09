fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<baseline_knn_web::app::App>::new().render();
}
