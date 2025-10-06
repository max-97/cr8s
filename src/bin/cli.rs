use clap::{value_parser, Arg, Command};

extern crate cr8s;

#[tokio::main]
async fn main() {
    let matches = Command::new("Cr8s")
        .about("Cr8s commands")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("users")
                .about("User management")
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("create")
                        .about("Create new user")
                        .arg_required_else_help(true)
                        .arg(Arg::new("username").required(true))
                        .arg(Arg::new("password").required(true))
                        .arg(
                            Arg::new("roles")
                                .required(true)
                                .num_args(1..)
                                .value_delimiter(','),
                        ),
                )
                .subcommand(Command::new("list").about("List existing users"))
                .subcommand(
                    Command::new("delete")
                        .about("Delete user by ID")
                        .arg_required_else_help(true)
                        .arg(Arg::new("id").required(true).value_parser(value_parser!(i32))),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("users", sub_matches)) => match sub_matches.subcommand() {
            Some(("create", create_params)) => {
                cr8s::commands::create_user(
                    create_params
                        .get_one::<String>("username")
                        .unwrap()
                        .to_owned(),
                    create_params
                        .get_one::<String>("password")
                        .unwrap()
                        .to_owned(),
                    create_params
                        .get_many::<String>("roles")
                        .unwrap()
                        .map(|v| v.to_owned())
                        .collect(),
                )
                .await;
            }
            Some(("list", _list_params)) => cr8s::commands::list_users().await,
            Some(("delete", delete_params)) => {
                cr8s::commands::delete_user(delete_params.get_one::<i32>("id").unwrap().to_owned())
                    .await
            }
            Some((&_, _)) => {}
            None => {}
        },
        Some((&_, _)) => {}
        None => {}
    }
}
