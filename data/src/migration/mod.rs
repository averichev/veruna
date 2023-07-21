use std::sync::Arc;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;

pub struct Migration;

impl Migration {
    pub async fn start(connection: Arc<Surreal<Db>>) {
        connection
            .query("
                DEFINE TABLE users SCHEMAFULL;
                DEFINE FIELD username ON TABLE users;
                DEFINE INDEX unique_username ON TABLE users COLUMNS username UNIQUE;
            ")
            .query("
                DEFINE TABLE roles SCHEMAFULL;
                DEFINE FIELD code ON TABLE roles;
                DEFINE INDEX unique_code ON TABLE roles COLUMNS code UNIQUE;
                INSERT INTO roles (code) VALUES ('admin'');
            ")
            .query("
                DEFINE TABLE has_roles SCHEMAFULL;
                DEFINE FIELD in ON TABLE has_roles TYPE record(users);
                DEFINE FIELD out ON TABLE has_roles TYPE record(roles);
                DEFINE INDEX unique_relationships
                    ON TABLE has_roles
                    COLUMNS in, out UNIQUE;
            ")
            .await
            .unwrap();
    }
}