#[derive(Debug)]
enum WineRegions {
    Bordeaux,
    Burgundy,
    Champagne,
    Tuscany,
    Rioja,
    NapaValley,
    BarossaValley, // Added new region
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

// Returns a string describing the popularity of the wine region
fn region_popularity(region: &WineRegions) -> &'static str {
    match region {
        WineRegions::Bordeaux => "Highly Popular",
        WineRegions::Burgundy => "Very Popular",
        WineRegions::Champagne => "Famous for sparkling wines",
        WineRegions::Tuscany => "Popular in Italy",
        WineRegions::Rioja => "Well-known in Spain",
        WineRegions::NapaValley => "Famous in the USA",
        WineRegions::BarossaValley => "Renowned in Australia",
    }
}

fn main() {
    let wine1 = Wine {
        name: String::from("Chateau Margaux"),
        region: WineRegions::Bordeaux,
    };

    let wine2 = Wine {
        name: String::from("Barolo"),
        region: WineRegions::Tuscany,
    };

    let wine3 = Wine {
        name: String::from("Penfolds Grange"),
        region: WineRegions::BarossaValley,
    };

    println!("Wine 1: {} from {:?}", wine1.name, wine1.region);
    println!("Wine 2: {} from {:?}", wine2.name, wine2.region);
    println!("Wine 3: {} from {:?}", wine3.name, wine3.region);

    supported_regions(wine1.region);
    supported_regions(WineRegions::Rioja);

    // Test region_popularity function
    println!(
        "Popularity of Bordeaux: {}",
        region_popularity(&WineRegions::Bordeaux)
    );
    println!(
        "Popularity of Tuscany: {}",
        region_popularity(&WineRegions::Tuscany)
    );
    println!(
        "Popularity of BarossaValley: {}",
        region_popularity(&WineRegions::BarossaValley)
    );
}
