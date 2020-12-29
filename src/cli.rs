use clap::ArgMatches;
use crate::config::ProjectConfig;
use std::io;
use crate::db_api::DbConnection;
use crate::security::hash_password;

pub struct CliActionError {
    pub error_msg: String,
}

impl CliActionError {
    pub fn new(error_msg: String) -> CliActionError {
        CliActionError {
            error_msg,
        }
    }
}

pub struct CliActionSuccess {
    pub term_after_cli_actions: bool,
}

impl CliActionSuccess {
    pub fn new(term_after_cli_actions: bool) -> CliActionSuccess {
        CliActionSuccess {
            term_after_cli_actions,
        }
    }
}

pub struct CliActions {
    create_db: bool,
    create_db_user: bool,
    create_db_tables: bool,
    add_admin_user: bool,
    change_user_password: bool,
    change_pw_username: String,
    drop_db: bool,
    drop_db_user: bool,
    drop_db_tables: bool,
}

impl CliActions {
    pub fn from_args(args: &ArgMatches) -> CliActions {
        let mut create_db = false;
        let mut create_db_user = false;
        let mut create_db_tables = false;
        let mut add_admin_user = false;
        let mut change_user_password = false;
        let mut change_pw_username = "";
        let mut drop_db = false;
        let mut drop_db_user = false;
        let mut drop_db_tables = false;

        let subcmd_install = args.subcommand_matches("install");

        if subcmd_install.is_some() {
            let subcmd_install = subcmd_install.unwrap();

            create_db = subcmd_install.is_present("create-db");
            create_db_user = subcmd_install.is_present("create-db-user");
            create_db_tables = subcmd_install.is_present("create-db-tables");
            add_admin_user = subcmd_install.is_present("create-admin-user");
        }

        let subcmd_maintenance = args.subcommand_matches("maintenance");

        if subcmd_maintenance.is_some() {
            let subcmd_maintenance = subcmd_maintenance.unwrap();

            change_user_password = subcmd_maintenance.is_present("change-user-password");
            change_pw_username = subcmd_maintenance.value_of("username").unwrap_or_default();
        }

        let subcmd_uninstall = args.subcommand_matches("uninstall");

        if subcmd_uninstall.is_some() {
            let subcmd_uninstall = subcmd_uninstall.unwrap();

            drop_db = subcmd_uninstall.is_present("drop-db");
            drop_db_user = subcmd_uninstall.is_present("drop-db-user");
            drop_db_tables = subcmd_uninstall.is_present("drop-db-tables");
        }

        CliActions {
            create_db,
            create_db_user,
            create_db_tables,
            add_admin_user,
            change_user_password,
            change_pw_username: change_pw_username.to_owned(),
            drop_db,
            drop_db_user,
            drop_db_tables,
        }
    }

    pub fn require_db_admin(&self) -> bool {
        let create_db = self.create_db;
        let create_db_user = self.create_db_user;
        let drop_db = self.drop_db;
        let drop_db_user = self.drop_db_user;

        if create_db || create_db_user || drop_db || drop_db_user {
            return true;
        }

        false
    }
}

pub async fn do_cli_actions(args: &ArgMatches<'_>, prj_config: &ProjectConfig) -> Result<CliActionSuccess, CliActionError> {
    let cli_actions = CliActions::from_args(args);
    let require_db_admin = cli_actions.require_db_admin();

    let mut db_admin_user = String::new();
    let mut db_admin_pw = String::new();

    if require_db_admin {
        // Get pg username from stdin
        print!("Postgres admin username: ");
        let stdin_read_success = io::stdin().read_line(&mut db_admin_user);

        if stdin_read_success.is_err() {
            let err =
                CliActionError::new(String::from("Failed to read postgres username from stdin"));

            return Err(err);
        }

        // Get pg password from stdin
        print!("Postgres admin password: ");
        let stdin_pass = rpassword::read_password();

        if stdin_pass.is_err() {
            let err =
                CliActionError::new(String::from("Failed to read postgres password from stdin"));

            return Err(err);
        }

        db_admin_pw = stdin_pass.ok().unwrap();
    }

    let create_db = cli_actions.create_db;
    let create_db_user = cli_actions.create_db_user;
    let create_db_tables = cli_actions.create_db_tables;
    let change_user_pw = cli_actions.change_user_password;
    let drop_db = cli_actions.drop_db;
    let drop_db_user = cli_actions.drop_db_user;
    let drop_db_tables = cli_actions.drop_db_tables;
    let add_admin_user = cli_actions.add_admin_user;

    let mut db_root_connection : Option<DbConnection> = None;
    let mut db_connection : Option<DbConnection> = None;

    if create_db || create_db_user || drop_db || drop_db_user {
        let connection =
            DbConnection::new_pg_root_connection(prj_config,
                                                 db_admin_user.as_str(),
                                                 db_admin_pw.as_str()).await;

        if connection.is_err() {
            let cli_action_error =
                CliActionError::new(String::from("Failed to connect to Postgres as admin"));

            return Err(cli_action_error);
        }

        db_root_connection = Some(connection.ok().unwrap());
    }

    if create_db_tables || change_user_pw || drop_db_tables || add_admin_user {
        let connection =
            DbConnection::new(prj_config, true, false).await;

        if connection.is_err() {
            let cli_action_error =
                CliActionError::new(String::from("Failed to connect to Postgres"));

            return Err(cli_action_error);
        }

        db_connection = Some(connection.ok().unwrap());
    }

    if create_db || create_db_user || create_db_tables {
        let db_username = prj_config.postgres_config.user.get_value();

        if create_db_user {
            let db_user_password = prj_config.postgres_config.password.get_value();
            let create_result =
                db_root_connection.as_ref().unwrap().create_pg_user(db_username.as_str(),
                                                  db_user_password.as_str()).await;

            if create_result.is_err() {
                let err = CliActionError::new(String::from("Failed to create database user"));

                return Err(err);
            }
        }

        if create_db {
            let db_name = prj_config.postgres_config.db_name.get_value();

            let create_result =
                db_root_connection.as_ref().unwrap().create_pg_database(db_name.as_str(),
                                                      db_username.as_str()).await;

            if create_result.is_err() {
                let err = CliActionError::new(String::from("Failed to create database"));

                return Err(err);
            }
        }

        if create_db_tables {
            let create_result =
                db_connection.as_ref().unwrap().create_pg_tables().await;

            if create_result.is_err() {
                let err = CliActionError::new(String::from("Failed to create tables"));

                return Err(err);
            }
        }

        let cli_actions_success = CliActionSuccess::new(true);

        return Ok(cli_actions_success);
    }

    if change_user_pw {
        // Get new password for the user from stdin
        print!("Enter new user password: ");
        let stdin_pass = rpassword::read_password();

        if stdin_pass.is_err() {
            let err =
                CliActionError::new(String::from("Failed to read postgres password from stdin"));

            return Err(err);
        }

        let target_user = cli_actions.change_pw_username;
        let new_user_pw = stdin_pass.ok().unwrap();
        let secret_key = prj_config.security_config.session_private_key.get_value();

        let hashed_pw = hash_password(new_user_pw.as_str(), secret_key.as_str());

        if hashed_pw.is_none() {
            let err =
                CliActionError::new(String::from("Failed to hash password"));

            return Err(err);
        }

        let hashed_pw = hashed_pw.unwrap();
        let action_success =
            db_connection.as_ref().unwrap().change_user_pw_by_username(target_user.as_str(), hashed_pw.as_str()).await;

        if action_success.is_err() {
            let err =
                CliActionError::new(String::from("Failed exec SQL code"));

            return Err(err);
        }

        let cli_actions_success = CliActionSuccess::new(true);

        return Ok(cli_actions_success);
    }

    if drop_db || drop_db_user || drop_db_tables {
        if drop_db {
            let drop_result =
                db_root_connection.as_ref().unwrap().drop_pg_db().await;

            if drop_result.is_err() {
                let err = CliActionError::new(String::from("Failed to drop database"));

                return Err(err);
            }
        }

        if drop_db_user {
            let target_user = prj_config.postgres_config.user.get_value();
            let drop_result =
                db_root_connection.as_ref().unwrap().drop_pg_db_user(target_user.as_str()).await;

            if drop_result.is_err() {
                let err = CliActionError::new(String::from("Failed to drop database user"));

                return Err(err);
            }
        }

        if drop_db_tables {
            let drop_result =
                db_root_connection.as_ref().unwrap().drop_pg_tables().await;

            if drop_result.is_err() {
                let err = CliActionError::new(String::from("Failed to drop database tables"));

                return Err(err);
            }
        }

        let cli_actions_success = CliActionSuccess::new(true);

        return Ok(cli_actions_success);
    }

    let cli_actions_success = CliActionSuccess::new(false);

    Ok(cli_actions_success)
}