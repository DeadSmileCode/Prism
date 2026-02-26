// mod abi;
// mod bus;
// mod manager;
// mod state;
mod ui;

// use crate::ui::NexusShell;
// use iced::Theme;
use crate::ui::PrismUI;

fn main() {
    tracing_subscriber::fmt::init();

    // iced::application(
    //     NexusShell::new,
    //     NexusShell::update,
    //     NexusShell::view,
    // )
    // .theme(|_state: &NexusShell| Theme::Dark) 
    // .title("Nexus Desktop")
    // .run()

    iced::daemon(
        PrismUI::new,
        PrismUI::update,
        PrismUI::view
    )
    .subscription(PrismUI::subscription)
    // .title(Example::title)
    // .theme(Example::theme)
    // .scale_factor(Example::scale_factor)
    .run();

}