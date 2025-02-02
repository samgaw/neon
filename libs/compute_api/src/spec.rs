//! `ComputeSpec` represents the contents of the spec.json file.
//!
//! The spec.json file is used to pass information to 'compute_ctl'. It contains
//! all the information needed to start up the right version of PostgreSQL,
//! and connect it to the storage nodes.
use serde::Deserialize;
use std::collections::HashMap;

/// String type alias representing Postgres identifier and
/// intended to be used for DB / role names.
pub type PgIdent = String;

/// Cluster spec or configuration represented as an optional number of
/// delta operations + final cluster state description.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct ComputeSpec {
    pub format_version: f32,

    // The control plane also includes a 'timestamp' field in the JSON document,
    // but we don't use it for anything. Serde will ignore missing fields when
    // deserializing it.
    pub operation_uuid: Option<String>,
    /// Expected cluster state at the end of transition process.
    pub cluster: Cluster,
    pub delta_operations: Option<Vec<DeltaOp>>,

    pub storage_auth_token: Option<String>,

    pub startup_tracing_context: Option<HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Cluster {
    pub cluster_id: String,
    pub name: String,
    pub state: Option<String>,
    pub roles: Vec<Role>,
    pub databases: Vec<Database>,
    pub settings: GenericOptions,
}

/// Single cluster state changing operation that could not be represented as
/// a static `Cluster` structure. For example:
/// - DROP DATABASE
/// - DROP ROLE
/// - ALTER ROLE name RENAME TO new_name
/// - ALTER DATABASE name RENAME TO new_name
#[derive(Clone, Debug, Deserialize)]
pub struct DeltaOp {
    pub action: String,
    pub name: PgIdent,
    pub new_name: Option<PgIdent>,
}

/// Rust representation of Postgres role info with only those fields
/// that matter for us.
#[derive(Clone, Debug, Deserialize)]
pub struct Role {
    pub name: PgIdent,
    pub encrypted_password: Option<String>,
    pub options: GenericOptions,
}

/// Rust representation of Postgres database info with only those fields
/// that matter for us.
#[derive(Clone, Debug, Deserialize)]
pub struct Database {
    pub name: PgIdent,
    pub owner: PgIdent,
    pub options: GenericOptions,
}

/// Common type representing both SQL statement params with or without value,
/// like `LOGIN` or `OWNER username` in the `CREATE/ALTER ROLE`, and config
/// options like `wal_level = logical`.
#[derive(Clone, Debug, Deserialize)]
pub struct GenericOption {
    pub name: String,
    pub value: Option<String>,
    pub vartype: String,
}

/// Optional collection of `GenericOption`'s. Type alias allows us to
/// declare a `trait` on it.
pub type GenericOptions = Option<Vec<GenericOption>>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn parse_spec_file() {
        let file = File::open("tests/cluster_spec.json").unwrap();
        let _spec: ComputeSpec = serde_json::from_reader(file).unwrap();
    }
}
