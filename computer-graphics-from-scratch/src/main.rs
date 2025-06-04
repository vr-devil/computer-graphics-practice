
fn main() {
    #[cfg(target_family = "wasm")]
    {
        use computer_graphics_from_scratch::webapp::run;
        run();
    }
}
