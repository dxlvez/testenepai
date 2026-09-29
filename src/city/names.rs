//! Names of people and places for every city.

use super::gen::CityId;
use super::map::BKind;

pub fn first_names(city: CityId, female: bool, year: i32) -> &'static [&'static str] {
    use CityId::*;
    match (city, female) {
        (NewOrleans, false) if year < 1990 => &["Jean", "Louis", "Pierre", "Antoine", "Joseph", "Henri", "Marcel", "Émile", "Thomas", "John", "Samuel", "Isaac", "Luca", "Salvatore", "Tobias", "Auguste", "Clement", "Remy", "Lucien", "Walter", "Arthur", "Earl", "Otis", "Felix"],
        (NewOrleans, true) if year < 1990 => &["Marie", "Margaret", "Colette", "Josephine", "Clara", "Odile", "Rose", "Hattie", "Celeste", "Louise", "Adele", "Eliza", "Beatrice", "Delphine", "Ivy", "Nora", "Pearl", "Sophie", "Violet", "Mabel", "Agnes", "Blanche"],
        (NewOrleans, false) => &["Darnell", "Andre", "Jamal", "Kevin", "Troy", "Marcus", "Beau", "Tyrone", "Justin", "Derek", "Terrence", "Jean-Luc"],
        (NewOrleans, true) => &["Keisha", "Tiffany", "Latoya", "Brandy", "Amber", "Monique", "Jasmine", "Crystal", "Danielle", "Chantal", "Clara"],
        (Chicago, false) => &["Frank", "Tony", "Michael", "Patrick", "Stanley", "Walter", "Eddie", "Harold", "George", "Casimir", "Vito", "Leo", "Ray", "Joe", "Albert", "Bernard", "Jimmy", "Irving", "Otto", "Sal"],
        (Chicago, true) => &["Dorothy", "Helen", "Mary", "Irene", "Ruth", "Bernice", "Evelyn", "Anna", "Frances", "Stella", "Gloria", "Edith", "Lillian", "Clara", "Rosa", "Vera", "June", "Mae"],
        (Bavaria, false) | (Berlin, false) => &["Andreas", "Josef", "Georg", "Lorenz", "Karl", "Johann", "Franz", "Anton", "Ludwig", "Michael", "Hans", "Xaver", "Wilhelm", "Friedrich", "Otto", "Heinrich", "Klaus", "Dieter"],
        (Bavaria, true) | (Berlin, true) => &["Viktoria", "Cäzilia", "Maria", "Anna", "Theresia", "Katharina", "Elisabeth", "Magdalena", "Gertrud", "Hildegard", "Ursula", "Ingrid", "Clara", "Luise", "Greta", "Frieda"],
        (London, false) => &["Albert", "Reginald", "Alfie", "Harold", "Stanley", "Bert", "Frederick", "Arthur", "Cyril", "Wilfred", "Ernest", "Percy", "Sidney", "Tom", "Charlie", "Ronald"],
        (London, true) => &["Doris", "Edna", "Winifred", "Gladys", "Mabel", "Ivy", "Elsie", "Florence", "Violet", "Peggy", "Joan", "Clara", "Iris", "Maud", "Nellie", "Vera"],
        (Adelaide, false) => &["Harold", "Keith", "Ronald", "Thomas", "Clive", "Norman", "Alfred", "Reginald", "Colin", "Bruce", "Lionel", "Gordon", "Stanley", "Percy", "Arthur", "Donald", "Leslie", "Neville"],
        (Adelaide, true) => &["Jessica", "Joan", "Beryl", "Doreen", "Marjorie", "Clara", "Gwen", "Shirley", "Nancy", "Maud", "Olive", "Phyllis", "Iris", "Hazel", "Elsie", "Thelma"],
        (Bergen, false) => &["Ole", "Knut", "Olav", "Arne", "Leif", "Sverre", "Einar", "Bjørn", "Tor", "Magnus", "Nils", "Halvor", "Per", "Gunnar", "Rolf", "Sigurd"],
        (Bergen, true) => &["Ingrid", "Astrid", "Solveig", "Kari", "Liv", "Sigrid", "Randi", "Gunhild", "Berit", "Ragnhild", "Clara", "Marit", "Turid", "Åse"],
        (SanFrancisco, false) => &["Gary", "Dennis", "Richard", "Wayne", "Larry", "Paul", "Jerry", "Lee", "Carlos", "Wei", "Terry", "Roger", "Donald", "Frank", "Steve", "Ray", "Daniel", "Chuck"],
        (SanFrancisco, true) => &["Linda", "Susan", "Carol", "Janis", "Donna", "Judy", "Mei", "Sharon", "Patricia", "Kathy", "Rosa", "Clara", "Grace", "Diane", "Joan", "Barbara"],
        (Portland, false) => &["Dale", "Earl", "Gordon", "Harlan", "Russell", "Vernon", "Lyle", "Wayne", "Glen", "Duane", "Carl", "Floyd", "Merle", "Roy"],
        (Portland, true) => &["Darlene", "Judy", "Carol", "Sharon", "Bonnie", "Marlene", "Arlene", "Clara", "Joyce", "Janet", "Norma", "Loretta"],
        (NewYork, false) => &["Michael", "David", "Kevin", "Anthony", "Marcus", "Jason", "Brian", "Luis", "Terrence", "Vincent", "Eric", "Scott", "Andre", "Tyrone", "Joseph", "Daniel", "Sean", "Victor"],
        (NewYork, true) => &["Jennifer", "Lisa", "Michelle", "Tanya", "Maria", "Denise", "Karen", "Angela", "Clara", "Nicole", "Tracy", "Monique", "Rachel", "Stephanie", "Yvette", "Laura"],
        (LosAngeles, false) => &["Carlos", "Jose", "Kevin", "Jason", "Ricky", "Darnell", "Hector", "Sung", "Travis", "Brandon", "Miguel", "Tony", "Eddie", "Oscar"],
        (LosAngeles, true) => &["Maria", "Jessica", "Ashley", "Brenda", "Lupe", "Tiffany", "Min-ji", "Stacy", "Rosa", "Clara", "Veronica", "Crystal", "Angela"],
    }
}

pub fn last_names(city: CityId) -> &'static [&'static str] {
    use CityId::*;
    match city {
        NewOrleans => &["Boudreaux", "Thibodeaux", "LeBlanc", "Fontenot", "Guidry", "Broussard", "Arceneaux", "Mouton", "Delacroix", "Marchand", "Moreau", "Picard", "Fournier", "Cavaretta", "Lombardi", "Miller", "Reed", "Cross", "Hale", "Morrow", "Toussaint", "Batiste", "Jolivette", "Duval", "Beaulieu", "Saucier", "Landry", "Castille", "Rougeau", "Voss"],
        Chicago => &["Kowalski", "O'Brien", "Novak", "Moretti", "Schultz", "Kelly", "Nowak", "Walsh", "Romano", "Brennan", "Wagner", "Sullivan", "Lewandowski", "Esposito", "Keller", "Doyle", "Gruber", "Maloney", "Russo", "Becker"],
        Bavaria => &["Gruber", "Huber", "Brandl", "Schlittenbauer", "Bauer", "Wagner", "Hofer", "Pichler", "Moser", "Leitner", "Aigner", "Stadler", "Riedl", "Maier", "Fischer", "Wimmer"],
        London => &["Smith", "Baker", "Wright", "Hughes", "Clarke", "Cooper", "Turner", "Harris", "Ward", "Pratt", "Hawkins", "Finch", "Blackwood", "Ashby", "Whitlock", "Carter", "Doyle", "Pike"],
        Adelaide => &["Thomson", "Harkness", "Cleland", "Boxall", "McRae", "Whitfield", "Ashby", "Pritchard", "Lyons", "Mortimer", "Fenwick", "Carrick", "Holloway", "Bramble", "Penrose", "Tamblyn", "Dunstan", "Ellery", "Ricketts", "Gale"],
        Berlin => &["Schmidt", "Neumann", "Krüger", "Lehmann", "Hoffmann", "Wolff", "Richter", "Zimmermann", "Braun", "Hartmann", "Lange", "Vogel", "Weber", "Kraus", "Sommer", "Engel"],
        Bergen => &["Hansen", "Johannessen", "Olsen", "Larsen", "Nilsen", "Berg", "Haugland", "Solheim", "Dahl", "Lie", "Brekke", "Vik", "Rasmussen", "Halvorsen", "Mjelde", "Fjeld"],
        SanFrancisco => &["Chen", "Russo", "Kowalczyk", "Delgado", "Murphy", "Wong", "Anderson", "Lindqvist", "Fong", "Castillo", "Harlan", "Okafor", "Nakamura", "Brody", "Faraday", "Moreno", "Vickers", "Toschi", "Hale", "Lam"],
        Portland => &["Lindgren", "Olson", "Petersen", "McCall", "Harrison", "Burke", "Jensen", "Tillman", "Hayes", "Colt", "Sorensen", "Whitaker", "Price", "Mercer"],
        NewYork => &["Rosario", "Goldberg", "Delaney", "Washington", "Ruiz", "Kaplan", "Brennan", "Jackson", "Santiago", "Weiss", "Moretti", "Okoye", "Fitzgerald", "Cohen", "Vasquez", "Harper", "Novak", "Greene", "Castellano", "Byrne"],
        LosAngeles => &["Garcia", "Hernandez", "Kim", "Park", "Johnson", "Martinez", "Nguyen", "Lopez", "Williams", "Ramirez", "Cho", "Brooks", "Flores", "Reyes", "Taylor", "Vega"],
    }
}

/// Building names per city/culture.
pub fn building_pool(city: CityId, k: BKind, year: i32) -> &'static [&'static str] {
    use BKind::*;
    use CityId::*;
    match (city, k) {
        (_, House) | (_, Apartment) | (_, Farmhouse) => &[""],
        (Bavaria, Bar) => &["Gasthaus zum Löwen", "Wirtshaus Brandl", "Zur Alten Post"],
        (Bavaria, Church) => &["Pfarrkirche St. Martin"],
        (Bavaria, Police) => &["Gendarmerie"],
        (Bavaria, Hotel) => &["Gasthof Hinterfeld"],
        (Bavaria, General) => &["Kramerladen Huber", "Bäckerei Maier", "Metzgerei Wimmer"],
        (Bavaria, Hospital) => &["Praxis Dr. Riedl"],
        (Bavaria, Mansion) => &["Gutshof von Aigner"],
        (Bavaria, Market) => &["Marktplatz"],
        (London, Bar) => &["The Crown & Anchor", "The Blind Beggar", "The Ten Bells", "The Prospect of Whitby", "The Black Dog", "The Ship"],
        (London, Club) => &["Café de Paris", "The Blue Lantern"],
        (London, Cabaret) => &["Windmill Theatre"],
        (London, Police) => &["Scotland Yard — Divisão H"],
        (London, Hotel) => &["The Savoy", "Hotel Russell", "Pensão da Sra. Pike"],
        (London, Newspaper) => &["The Evening Standard", "Daily Herald"],
        (London, Church) => &["Christ Church Spitalfields", "St. Dunstan"],
        (London, Station) => &["Estação Liverpool Street"],
        (London, Restaurant) => &["Lyons Corner House", "Fish & Chips do Bert"],
        (Berlin, Bar) => &["Kneipe Zum Hirsch", "Bar Moka Efti", "Eckkneipe Lange"],
        (Berlin, Club) => &["Resi", "Clärchens Ballhaus"],
        (Berlin, Police) => &["Polizeipräsidium"],
        (Berlin, Hotel) => &["Hotel Adlon (reconstruído)", "Pension Vogel"],
        (Berlin, Newspaper) => &["Der Tagesspiegel", "Neues Deutschland"],
        (Berlin, Station) => &["Bahnhof Friedrichstraße"],
        (Bergen, Bar) => &["Kafé Bryggen", "Wesselstuen", "Kroa"],
        (Bergen, Hotel) => &["Hotel Rosenkrantz", "Hotel Neptun", "Hotel Norge"],
        (Bergen, Police) => &["Politikammer"],
        (Bergen, Church) => &["Mariakirken", "Domkirken"],
        (Bergen, Newspaper) => &["Bergens Tidende"],
        (Bergen, Market) => &["Fisketorget"],
        (Bergen, Station) => &["Bergen Stasjon"],
        (Adelaide, Bar) => &["Hotel Grand Pier (bar)", "The Criterion", "Colonist Hotel"],
        (Adelaide, Hotel) => &["Strathmore Hotel", "Glenelg Esplanade Hotel"],
        (Adelaide, Newspaper) => &["The Advertiser", "The News"],
        (SanFrancisco, Club) => &["The Fillmore", "Avalon Ballroom", "Matrix"],
        (SanFrancisco, Bar) => &["Vesuvio", "Tosca Café", "The Saloon", "Specs"],
        (SanFrancisco, Newspaper) => &["San Francisco Chronicle", "Examiner"],
        (Portland, Bar) => &["The Rialto", "Shanghai Tunnel Bar", "Kelly's Olympian"],
        (Portland, Station) => &["Aeroporto Internacional de Portland", "Union Station"],
        (NewYork, Club) => &["Danceteria", "Paradise Garage", "The Tunnel", "Palladium"],
        (NewYork, Bar) => &["McSorley's", "Pete's Tavern", "The Subway Inn", "Blarney Stone"],
        (NewYork, Newspaper) => &["New York Post", "Daily News"],
        (NewYork, Station) => &["Grand Central"],
        (LosAngeles, Club) => &["The Roxy", "Whisky a Go Go", "Viper Room"],
        (LosAngeles, Bar) => &["Frolic Room", "Barney's Beanery", "King Eddy Saloon"],
        (LosAngeles, Restaurant) => &["Canter's Deli", "Pink's Hot Dogs", "Taqueria El Sol"],
        (LosAngeles, Newspaper) => &["Los Angeles Times"],
        (_, Bar) => &["Bar do Lafitte", "O Galo Cego", "Taverna Três Luas", "Bar Saint-Roch", "Bar do Porto", "O Último Gole", "Bar Marais", "Bar Cypress"],
        (_, Club) => {
            if year < 1950 {
                &["Clube Blue Parrot", "Clube Toussaint", "Clube Anjo Negro", "Clube Lua de Cobre"]
            } else if year < 1980 {
                &["Clube Fillmore Rubro", "Clube Hi-Fi", "Clube Céu Violeta", "Clube Estática"]
            } else {
                &["Clube Neon Oblivion", "Clube Red Room", "Clube Subsolo", "Clube Vapor"]
            }
        }
        (_, Cabaret) => &["Casa da Madame Rougeau", "Salão Veludo", "Casa das Sete Lanternas"],
        (_, Restaurant) => &["Restaurante Beaumont", "Café du Monde Sombrio", "Diner da Esquina", "Restaurante Lapin", "Cantina Moretti"],
        (_, Hotel) => &["Hotel Saint-Aubin", "Hotel Grand Meridian", "Hotel Pelican", "Hotel Excelsior"],
        (_, Church) => &["Igreja de São Judas", "Catedral de Santa Luzia", "Igreja da Última Hora"],
        (_, Police) => &["Delegacia Central"],
        (_, Hospital) => &["Hospital da Caridade", "Hospital Municipal"],
        (_, Station) => &["Estação Terminal"],
        (_, Bank) => &["Banco Hibernia", "Banco do Delta", "First National"],
        (_, Newspaper) => &["The Daily Picayune", "O Arauto da Noite", "The Evening Ledger"],
        (_, Radio) => &["Rádio WRTH", "Rádio KVAL"],
        (_, Clothing) => &["Alfaiataria Delacroix", "Alfaiataria Weiss"],
        (_, General) => &["Armazém Guidry", "Mercearia Kowalski", "Armazém Boudreaux", "Empório Hale"],
        (_, GunShop) => &["Armas Marchand", "Armaria Colt & Filhos"],
        (_, Pharmacy) => &["Farmácia Lamarque", "Botica do Dr. Voss"],
        (_, Pawn) => &["Penhores Zeller", "Casa de Penhores Gold"],
        (_, Warehouse) => &["Depósito Nº 3", "Depósito Nº 7", "Depósito Nº 12", "Depósito Nº 19"],
        (_, Factory) => &["Fábrica Delta Conservas", "Fundição Norte"],
        (_, Mansion) => &["Mansão Devereaux", "Mansão Whitcombe", "Mansão Ashford", "Mansão LeBlanc"],
        (_, Office) => &["Escritório Sterling & Filhos", "Cartório Municipal", "Agência Pinkerton-Vale"],
        (_, Abandoned) => &["Prédio Abandonado"],
        (_, Barn) => &["Celeiro"],
        (_, Market) => &["Mercado Francês", "Mercado Municipal"],
        (_, Lab) => &["Laboratório"],
        (_, Safehouse) => &["Quarto Alugado"],
    }
}
