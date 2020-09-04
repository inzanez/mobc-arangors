use arangors::connection::role::Normal;
use arangors::ClientError;
use mobc::{async_trait, Manager};

#[cfg(feature = "reqwest")]
use arangors::client::reqwest::ReqwestClient;
#[cfg(feature = "surf")]
use arangors::client::surf::SurfClient;

#[derive(Clone, Debug)]
pub struct ArangoDBConnectionManager {
    url: String,
    username: String,
    password: String,
    use_jwt: bool,
    validate: bool,
}

impl ArangoDBConnectionManager {
    pub fn new(
        url: &str,
        username: &str,
        password: &str,
        use_jwt: bool,
        validate: bool,
    ) -> ArangoDBConnectionManager {
        ArangoDBConnectionManager {
            url: url.to_owned(),
            username: username.to_owned(),
            password: password.to_owned(),
            use_jwt,
            validate,
        }
    }
}

#[async_trait]
impl Manager for ArangoDBConnectionManager {
    type Connection = arangors::Connection;
    type Error = ClientError;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        if self.use_jwt == true {
            let client =
                arangors::Connection::establish_jwt(&self.url, &self.username, &self.password)
                    .await?;
            return Ok(client);
        } else {
            let client = arangors::Connection::establish_basic_auth(
                &self.url,
                &self.username,
                &self.password,
            )
            .await?;
            return Ok(client);
        }
    }

    #[cfg(feature = "surf")]
    async fn check(&self, conn: Self::Connection) -> Result<Self::Connection, Self::Error> {
        if self.validate {
            arangors::connection::GenericConnection::<SurfClient, Normal>::validate_server(
                &self.url,
            )
            .await?;
        }

        Ok(conn)
    }
    #[cfg(feature = "reqwest")]
    async fn check(&self, conn: Self::Connection) -> Result<Self::Connection, Self::Error> {
        if self.validate {
            arangors::connection::GenericConnection::<ReqwestClient, Normal>::validate_server(
                &self.url,
            )
            .await?;
        }

        Ok(conn)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
