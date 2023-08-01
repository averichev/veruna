use std::sync::Arc;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;

pub struct Migration;

impl Migration {
    pub async fn start(connection: &Arc<Surreal<Db>>) {
        connection
            .query("
                define ns veruna;
                define db veruna;
                use ns veruna;
                use db veruna;
            ")
            .query("
                DEFINE TABLE policy SCHEMAFULL;
                DEFINE FIELD rule ON TABLE policy TYPE string;
                DEFINE FIELD object ON TABLE policy TYPE string;
                DEFINE FIELD action ON TABLE policy TYPE string;
                DEFINE INDEX unique_rule ON TABLE policy COLUMNS rule, object, action UNIQUE;
                INSERT INTO policy (rule, object, action) VALUES ($rule, $object, $action);
            ")
            .bind(("rule", "r.subject.role == \"admin\""))
            .bind(("object", "/admin/*"))
            .bind(("action", "read"))
            .query("
                DEFINE TABLE users SCHEMAFULL;
                DEFINE FIELD username ON TABLE users TYPE string;
                DEFINE INDEX unique_username ON TABLE users COLUMNS username UNIQUE;
            ")
            .query("
                DEFINE TABLE roles SCHEMAFULL;
                DEFINE FIELD code ON TABLE roles TYPE string;
                DEFINE INDEX unique_code ON TABLE roles COLUMNS code UNIQUE;
                INSERT INTO roles (code) VALUES ($admin_role);
            ")
            .bind(("admin_role", "admin"))
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