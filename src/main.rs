use clap::Parser;
use otptool::migration::{decode_from_image, decode_from_link};

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    link: Option<String>,

    #[arg(short, long)]
    image: Option<String>,
}

fn main() {
    let args = Cli::parse();

    if let Some(link) = args.link {
        let res = decode_from_link(&link).unwrap();
        print_parameters(&res);
        return;
    }

    if let Some(image) = args.image {
        let res = decode_from_image(image.into()).unwrap();
        print_parameters(&res);
        return;
    }

    println!("unknown input");
}

fn print_parameters(otp_parameters: &[otptool::otp::OtpParameters]) {
    for otp_parameter in otp_parameters {
        let mut issuer = otp_parameter.issuer.clone();
        if issuer.is_empty() {
            issuer = "-".into();
        }

        println!(
            "name: {}, issuer: {}, secret: {}",
            otp_parameter.name, issuer, otp_parameter.secret
        );
    }
}
