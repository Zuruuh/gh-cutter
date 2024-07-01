#![allow(unused_variables, private_interfaces, dead_code)]

use color_eyre::eyre::{self, Context};
use tui::App;

mod config;
mod tui;

pub fn install_hooks() -> color_eyre::Result<()> {
    let hook_builder = color_eyre::config::HookBuilder::default();
    let (panic_hook, eyre_hook) = hook_builder.into_hooks();

    let panic_hook = panic_hook.into_panic_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        App::restore().unwrap();
        panic_hook(panic_info);
    }));

    let eyre_hook = eyre_hook.into_eyre_hook();
    eyre::set_hook(Box::new(move |error| {
        App::restore().unwrap();
        eyre_hook(error)
    }))?;

    Ok(())
}

// http://127.0.0.1:12677/oauth
#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    install_hooks()?;
    let config_path = xdg::BaseDirectories::new()
        .map(|xdg_dirs| xdg_dirs.get_config_home())
        .map(|mut config_home| {
            config_home.push("gh-cutter.toml");

            config_home
        })
        .ok();

    // if let Some(config_path) = config_path {
    //     if !try_exists(&config_path).await? {
    //         tokio::fs::write(
    //             &config_path,
    //             toml::to_string_pretty(&config::Config::default()).unwrap(),
    //         )
    //         .await?;
    //
    //         print!("no");
    //     }
    //
    //
    // }

    let mut app = App::new(None)?;
    app.run().wrap_err("Unexpected error")?;
    drop(app);

    Ok(())
    // tui::run()
}
