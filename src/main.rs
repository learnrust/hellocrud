use clap::{arg, command, Command};
mod classes;
mod functions;
mod hello;
mod test;
fn main() {
    let matches = command!()
        .help_template(
            "{before-help}{name}-{version} {about-with-newline}{author-with-newline} 
{usage-heading} [Options] [Commands] [Options] 

{all-args}{after-help} ",
        )
        .version("1.1")
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("create")
                .about("Create operation")
                .arg(arg!([CREATE_VALUE])),
        )
        .subcommand(
            Command::new("retrieve")
                .about("retrieve operation")
                .arg(arg!([RETRIEVE_VALUE])),
        )
        .subcommand(
            Command::new("update")
                .about("Update operation")
                .arg(arg!([UPDATE_VALUE])),
        )
        .subcommand(
            Command::new("delete")
                .about("Delete opteration")
                .arg(arg!([NAME])),
        )
        .subcommand(
            Command::new("demo")
                .about("funcions and classes demo")
                .arg(arg!([DEMO_VALUE])),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("create", sub_matches)) => {
            println!(
                "'subcommand create' was used, argument is: {:?}",
                sub_matches.get_one::<String>("NAME")
            );
            let result = functions::add(5, 3);
            println!("functions::add return: {}", result);
        }
        Some(("retrieve", sub_matches)) => {
            println!(
                "'subcommand retrieve' was used, argument is: {:?}",
                sub_matches.get_one::<String>("NAME")
            );
            functions::greeting("Retrieve operation".to_string());
            let result = functions::multiply(5, 3);
            println!("functions::multiply return: {}", result);
            let triangle = classes::Triangle {
                base: 8.0,
                height: 4.0,
            };

            println!(
                "triangle of classes::Triangle instance retured: base = {}, height = {}, area = {}",
                triangle.base,
                triangle.height,
                triangle.area()
            );
        }
        Some(("update", sub_matches)) => {
            println!(
                "'subcommand update' was used, argument is: {:?}",
                sub_matches.get_one::<String>("UPDATE_VALUE")
            );
            functions::hello();
            functions::greeting("Update operation".to_string());
            let rectangle = classes::Rectangle {
                width: 10.0,
                height: 5.0,
            };
            println!("rectangle instance of classes::Rectangle return: width = {}, height = {}, area = {}",
		     rectangle.width,
		     rectangle.height,
		     rectangle.area());
        }
        Some(("delete", sub_matches)) => {
            println!(
                "'subcommand delete' was used, argument is: {:?}",
                sub_matches.get_one::<String>("NAME")
            );
            functions::greeting("Delete operation".to_string());
            functions::hello_world();
            let circle = classes::Circle { radius: 3.0 };
            println!(
                "circle instance of classes::Circle return: radius = {}, circumference = {}",
                circle.radius,
                circle.circumference()
            );
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
