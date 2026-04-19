pub struct HelpCategory {
    pub name: String,
    pub keybinds: Vec<&'static str>,
}

pub fn get_help_categories() -> Vec<HelpCategory> {
    vec![
        HelpCategory {
            name: "Global".to_string(),
            keybinds: vec![
                "j/k or arrows - navigate",
                "l - next tab",
                "h - previous tab",
                "? - show help",
                "C-c - quit",
            ],
        },
        HelpCategory {
            name: "Tabs".to_string(),
            keybinds: vec![
                "1-5 - switch tab",
            ],
        },
        HelpCategory {
            name: "Params".to_string(),
            keybinds: vec![
                "a - add param",
                "d - delete param",
                "space - toggle param",
                "j/k - select row",
            ],
        },
        HelpCategory {
            name: "Headers".to_string(),
            keybinds: vec![
                "a - add header",
                "d - delete header",
                "space - toggle header",
                "j/k - select row",
            ],
        },
        HelpCategory {
            name: "Body".to_string(),
            keybinds: vec![
                "C-y - copy selection",
                "C-d - cut selection",
                "C-v or C-p - paste",
            ],
        },
        HelpCategory {
            name: "Result".to_string(),
            keybinds: vec![
                "r - make request",
                "C-s - save request",
                "j/k - scroll",
            ],
        },
        HelpCategory {
            name: "Sidebar".to_string(),
            keybinds: vec![
                "a - add collection/request",
                "d - delete collection/request",
                "o - load/open",
                "enter - load/open",
            ],
        },
    ]
}