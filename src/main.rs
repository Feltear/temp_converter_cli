use clap::Parser;

///
/// @brief Structure dérivé de clap Parser
/// @args temp Température à convertir
/// @args marker Unité de mesure de la température à convertir
///
#[derive(Parser)]
struct Cli {
    temp: f64,
    marker: char,
}

fn main() {
    // parse the elements from the call in the terminal
    let args = Cli::parse();

    // Celsius to fahrenheit and celsius to kelvin
    let mut c_fn = [|x| println!("{:.1}°F", (x * 1.8 + 32.0)), |x| println!("{:.1}K", x + 273.15)];

    // fahrenheit to celsius and fahrenheit to kelvin
    let mut f_fn = [|x| println!("{:.1}°C", (x - 32.0) / 1.8), |x| println!("{:.1}K", ((x - 32.0) * 5.0 / 9.0) + 273.15)];

    // kelvin to celsius and kelvin to fahrenheit
    let mut k_fn = [|x| println!("{:.1}°C", x - 273.15), |x| println!("{:.1}°F", (x - 273.15) * 9.0 / 5.0 + 32.0)];

    // match the unit marker
    match args.marker {
        'C' | 'c' => {
            for c in c_fn.iter_mut() {
                (*c)(args.temp);
            }
        }
        'F' | 'f' => {
            for f in f_fn.iter_mut() {
                (*f)(args.temp);
            }
        }
        'K' | 'k' => {
            for k in k_fn.iter_mut() {
                (*k)(args.temp);
            }
        }
        _ => println!("{} is a wrong type of unit", args.marker)
    }
}