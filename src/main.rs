
fn main() {
    #[cfg(target_family = "wasm")]
    {
        use computer_graphics_practice::webapp::run;
        run();
    }
}
