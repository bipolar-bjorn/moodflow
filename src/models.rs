#[derive(PartialEq)]
pub enum AppScreen {
    Welcome,
    AddUser,
    MainApp,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Tab {
    Add,
    History,
    Analytics,
    Settings,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum SettingsTab {
    General,
    Moods,
    Tags,
    Goals,
}

// User
#[allow(dead_code)]
pub struct User {
    pub id: Option<i64>,
    pub name: String,
    pub pin_hash: Option<String>,
    pub email: Option<String>,
}
