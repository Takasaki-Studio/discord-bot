use poise::serenity_prelude::User;

pub trait AvatarOrDefault {
    fn get_avatar_or_default(&self) -> String;
}

impl AvatarOrDefault for User {
    fn get_avatar_or_default(&self) -> String {
        if self.avatar_url().is_none() {
            self.default_avatar_url()
        } else {
            self.avatar_url().unwrap()
        }
    }
}
