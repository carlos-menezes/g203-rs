mod util;

use clap::{command, Parser, Subcommand};
use g203_lib::{Controller, Direction};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Solid {
        /// the RGB color value
        #[arg(required = true, help = "e.g. FF0000")]
        rgb: String,
    },
    Breathe {
        /// the RGB color value
        #[arg(required = true, help = "e.g. FF0000")]
        rgb: String,
        /// the rate of the effect
        #[arg(default_value = "1000", help = "the rate of the effect")]
        rate: u16,
        /// the brightness of the effect
        #[arg(default_value = "100", help = "the brightness of the effect")]
        brightness: u8,
    },
    Cycle {
        /// the rate of the effect
        #[arg(default_value = "1000", help = "the rate of the effect")]
        rate: u16,
        /// the brightness of the effect
        #[arg(default_value = "100", help = "the brightness of the effect")]
        brightness: u8,
    },
    Triple {
        /// the RGB color value
        #[arg(required = true, help = "e.g. FF0000")]
        rgb_left: String,

        /// the RGB color value
        #[arg(required = true, help = "e.g. FF0000")]
        rgb_center: String,

        /// the RGB color value
        #[arg(required = true, help = "e.g. FF0000")]
        rgb_right: String,
    },
    Wave {
        /// the rate of the effect
        #[arg(default_value = "1000", help = "the rate of the effect")]
        rate: u16,
        /// the brightness of the effect
        #[arg(default_value = "100", help = "the brightness of the effect")]
        brightness: u8,
        /// the direction of the effect
        #[arg(default_value = "l", help = "the direction of the effect")]
        direction: String,
    },
    Blend {
        /// the rate of the effect
        #[arg(default_value = "1000", help = "the rate of the effect")]
        rate: u16,
        /// the brightness of the effect
        #[arg(default_value = "100", help = "the brightness of the effect")]
        brightness: u8,
    },
}

fn main() {
    let controller = Controller::new().unwrap();
    let cli = Cli::parse();

    match cli.command {
        Commands::Solid { rgb } => {
            let hex = util::hex_to_rgb(&rgb).unwrap();
            controller.set_solid(hex).unwrap()
        }
        Commands::Breathe {
            rgb,
            rate,
            brightness,
        } => {
            let hex = util::hex_to_rgb(&rgb).unwrap();
            controller.set_breathe(hex, rate, brightness).unwrap()
        }
        Commands::Cycle { rate, brightness } => controller.set_cycle(rate, brightness).unwrap(),
        Commands::Triple {
            rgb_left,
            rgb_center,
            rgb_right,
        } => {
            let hex_left = util::hex_to_rgb(&rgb_left).unwrap();
            let hex_center = util::hex_to_rgb(&rgb_center).unwrap();
            let hex_right = util::hex_to_rgb(&rgb_right).unwrap();
            controller
                .set_triple([hex_left, hex_center, hex_right])
                .unwrap()
        }
        Commands::Wave {
            rate,
            brightness,
            direction,
        } => {
            let direction = match direction.as_str() {
                "l" => Direction::Left,
                "r" => Direction::Right,
                _ => panic!("Invalid direction"),
            };
            controller.set_wave(rate, brightness, direction).unwrap()
        }
        Commands::Blend { rate, brightness } => controller.set_blend(rate, brightness).unwrap(),
    }
}
