use domain::object::{
    chrono::Local, email_address::EmailAddress, rusty_ulid::Ulid, url::Url, UserDiscriminator,
    UserName,
};

pub struct UserApplicationService;

pub trait IUserApplicationService {
    fn register(
        discriminator: UserDiscriminator,
        name: UserName,
        email: EmailAddress,
        web_page: Url,
    ) -> Result<domain::entity::User, Box<dyn std::error::Error>>;
}

impl IUserApplicationService for UserApplicationService {
    fn register(
        discriminator: UserDiscriminator,
        name: UserName,
        email: EmailAddress,
        web_page: Url,
    ) -> Result<domain::entity::User, Box<dyn std::error::Error>> {
        let user = domain::entity::User::new(
            Ulid::generate(),
            discriminator,
            name,
            email,
            web_page,
            Local::now(),
            Local::now(),
        );
        // 登録前チェック処理

        // リポジトリに保存するなどの処理

        Ok(user)
    }
}
