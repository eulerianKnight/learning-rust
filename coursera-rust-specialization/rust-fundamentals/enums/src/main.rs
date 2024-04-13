#[derive(Debug)]
enum WineRegions {
    Bordeaux,
    Burgundy,
    Champagne,
    Tuscany,
    Rioja,
    NapaValley,
}

struct Wine {
    name: String,
    region: WineRegions, // wine regions used as a type
}

fn supported_regions(w: WineRegions) {
    match w {
        WineRegions::Rioja => println!("Rioja is supported!"),
        _ => println!("{:?} is not supported!", w),
    }
}

enum FileSize {
    Bytes(u64),
    Kilobytes(u64),
    Megabytes(u64),
    Gigabytes(u64),
}

fn format_size(size: u64) -> String {
    let filesize = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::Kilobytes(size / 1000),
        1_000_000..=999_999_999 => FileSize::Megabytes(size / 1_000_000),
        _ => FileSize::Gigabytes(size / 1_000_000_000),
    };

    match filesize {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{:.2} KB", kb as f64 / 1000.0),
        FileSize::Megabytes(mb) => format!("{:.2} MB", mb as f64 / 1000.0),
        FileSize::Gigabytes(gb) => format!("{:.2} GB", gb as f64 / 1000.0),
    }
}

enum Shape {
    Circle(f64),
    Square(f64),
}

fn main() {
    let shapes = vec![Shape::Circle(5.0), Shape::Square(3.0)];

    let total_area: f64 = shapes
        .iter()
        .map(|shape| match shape {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(length) => length * length,
        })
        .sum();

    println!("Total area: {} sq. units", total_area);
}

// fn main() {
//     let result = format_size(6888837399);
//     println!("{}", result)
// }


// fn main() {
//     let wine1 = Wine {
//         name: String::from("Chateau Margaux"),
//         region: WineRegions::Bordeaux,
//     };

//     let wine2 = Wine {
//         name: String::from("Barolo"),
//         region: WineRegions::Tuscany,
//     };

//     // println!("Wine 1: {} from {:?}", wine1.name, wine1.region);
//     // println!("Wine 2: {} from {:?}", wine2.name, wine2.region);
//     supported_regions(wine1.region);
//     supported_regions(WineRegions::Rioja);
// }
