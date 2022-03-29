pub fn show_error_message(title: &str, err: &anyhow::Error) {
    rfd::MessageDialog::new()
        .set_buttons(rfd::MessageButtons::Ok)
        .set_title(title)
        .set_description(&err.to_string())
        .show();
}

pub fn is_approx_integer(val: f64) -> bool {
    val.fract().abs() < 1e-5
}
