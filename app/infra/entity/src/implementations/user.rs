use domain::entity::User;

impl From<User> for crate::entity::user::ActiveModel {
    fn from(user: User) -> Self {
        Self {
            id: sea_orm::Set(user.id.to_string()),
            discriminator: sea_orm::Set(user.discriminator.into()),
            name: sea_orm::Set(user.name.into()),
            email: sea_orm::Set(user.email.as_str().to_owned()),
            web_page: sea_orm::Set(user.web_page.map(|url| url.to_string())),
            created_at: sea_orm::Set(user.created_at.into()),
            updated_at: sea_orm::Set(user.updated_at.into()),
        }
    }
}
