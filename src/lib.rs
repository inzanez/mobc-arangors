use mobc::{Manager, async_trait};
use arangors::ClientError;

#[derive(Clone, Debug)]
pub struct ArangoDBConnectionManager {
    url: String,
    username: String,
    password: String,
    use_jwt: bool,
}

impl ArangoDBConnectionManager {
    pub fn new(
        url: &str,
        username: &str,
        password: &str,
        use_jwt: bool,
    ) -> ArangoDBConnectionManager {
        ArangoDBConnectionManager {
            url: url.to_owned(),
            username: username.to_owned(),
            password: password.to_owned(),
            use_jwt,
        }
    }
}

#[async_trait]
impl Manager for ArangoDBConnectionManager {
    type Connection = arangors::Connection;
    type Error = ClientError;

   async fn connect(&self) -> Result<Self::Connection, Self::Error> {
      if self.use_jwt == true {
          let client = arangors::Connection::establish_jwt(&self.url, &self.username, &self.password).await?;
          return Ok(client);

      } else {
          let client = arangors::Connection::establish_basic_auth(&self.url, &self.username, &self.password).await?;
          return Ok(client);
      }
   }

   async fn check(&self, conn: Self::Connection) -> Result<Self::Connection, Self::Error> {
       conn.validate_server().await?;
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
