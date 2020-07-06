#[derive(Debug)]
pub enum Environment {
    Production,
    Custom(url::Url),
}

impl<'a> From<&'a Environment> for url::Url {
    fn from(environment: &Environment) -> Self {
        match environment {
            Environment::Production => {
                url::Url::parse("https://api.cloudflare.com/client/v4/").unwrap()
            }
            Environment::Custom(url) => url.clone(),
        }
    }
}
