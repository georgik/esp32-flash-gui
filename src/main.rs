use dioxus::html::{text, textarea};
use dioxus::prelude::*;
use espflash::{
    cli::{
        config::Config,
        connect, ConnectArgs,
        print_board_info,
        }
    };
use tokio;

fn main() {
    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let mut info = use_state(&cx, || String::from("..."));

    cx.render(rsx! {
        input { r#type: "text", id: "username", name: "username" }
        textarea { id: "chip_info", name: "chip_info", value: "{info}" }
        button {
            onclick: move |_|  {
                println!("Start flashing firmware");
                let connect_args = ConnectArgs {
                    port: Some("/dev/tty.usbserial-0001".to_string()),
                    baud: Some(115200 as u32),
                    no_stub: false,
                    };
                let config = Config::default();
                let mut flasher = connect(&connect_args, &config).unwrap();
                let chip = flasher.chip();
                let target = chip.into_target();
                let device_info = flasher.device_info().unwrap();
                let content = format!("Chip: {:?}\nCrystal frequency: {:?}MHz", device_info.chip, device_info.crystal_frequency);
                // let text_value: &str = content.as_str();
                info.set(content);

                // tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                println!("End flashing firmware");
            },
            "Flash"
        }
    })
}

