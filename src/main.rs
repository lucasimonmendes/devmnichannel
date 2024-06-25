use std::collections::BTreeMap;

mod models;
mod ui;

fn main() {
    let title = "
██████╗ ███████╗██╗   ██╗███╗   ███╗███╗   ██╗██╗ ██████╗██╗  ██╗ █████╗ ███╗   ██╗███╗   ██╗███████╗██╗     
██╔══██╗██╔════╝██║   ██║████╗ ████║████╗  ██║██║██╔════╝██║  ██║██╔══██╗████╗  ██║████╗  ██║██╔════╝██║     
██║  ██║█████╗  ██║   ██║██╔████╔██║██╔██╗ ██║██║██║     ███████║███████║██╔██╗ ██║██╔██╗ ██║█████╗  ██║     
██║  ██║██╔══╝  ╚██╗ ██╔╝██║╚██╔╝██║██║╚██╗██║██║██║     ██╔══██║██╔══██║██║╚██╗██║██║╚██╗██║██╔══╝  ██║     
██████╔╝███████╗ ╚████╔╝ ██║ ╚═╝ ██║██║ ╚████║██║╚██████╗██║  ██║██║  ██║██║ ╚████║██║ ╚████║███████╗███████╗
╚═════╝ ╚══════╝  ╚═══╝  ╚═╝     ╚═╝╚═╝  ╚═══╝╚═╝ ╚═════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝╚═╝  ╚═══╝╚══════╝╚══════╝
                                                                                                             
";
    let phrase = "Hi, I'm Devy, your comunication assistant, what do you want?";

    ui::print_header(title, phrase);

    let mut main_menu: ui::Menu = BTreeMap::new();
    main_menu.insert(
        "1",
        ui::MenuItem {
            label: "Write a LinkedIn Post",
            action: ui::MenuAction::Execute(write_linkedin),
        },
    );
    main_menu.insert(
        "2",
        ui::MenuItem {
            label: "Write a Instagram Post",
            action: ui::MenuAction::Execute(write_instagram),
        },
    );
    main_menu.insert(
        "3",
        ui::MenuItem {
            label: "Exit",
            action: ui::MenuAction::Exit,
        },
    );

    ui::print_menu(&main_menu);
}

fn write_linkedin() -> Result<(), std::io::Error> {
    models::write_linkedin_post()
}

fn write_instagram() -> Result<(), std::io::Error> {
    models::write_instagram_post()
}
