pub fn register_metrics() {
    metrics::register_gauge!("imglaze_active_sockets", "Active Sockets");
    metrics::register_counter!("imglaze_images_changed", "Amount of images changed");
}
