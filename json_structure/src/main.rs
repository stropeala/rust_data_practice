mod clients;
mod data;
mod organizers;
mod timers;

use dotenvy::dotenv;

use clients::add_client;
use organizers::organizer;
use timers::{add_entry_time, add_exit_time};

fn main() -> Result<(), anyhow::Error> {
    // Load environment once, up front, rather than on every file-path lookup.
    dotenv().ok();

    let client_array = [
        ["Ionescu", "Ana", "0722000001", "Bucuresti"],
        ["Popescu", "Mihai", "0722000002", "Cluj-Napoca"],
        ["Georgescu", "Elena", "0722000003", "Iasi"],
        ["Radu", "Andrei", "0722000004", "Timisoara"],
        ["Dumitru", "Maria", "0722000005", "Constanta"],
        ["Stan", "Cristian", "0722000006", "Craiova"],
        ["Stoica", "Ioana", "0722000007", "Brasov"],
        ["Gheorghe", "Alexandru", "0722000008", "Galati"],
        ["Matei", "Gabriela", "0722000009", "Ploiesti"],
        ["Constantin", "Vlad", "0722000010", "Oradea"],
        ["Marin", "Diana", "0722000011", "Braila"],
        ["Tudor", "Razvan", "0722000012", "Arad"],
        ["Barbu", "Simona", "0722000013", "Pitesti"],
        ["Nistor", "Bogdan", "0722000014", "Sibiu"],
        ["Florea", "Roxana", "0722000015", "Bacau"],
        ["Dobre", "Catalin", "0722000016", "Targu Mures"],
        ["Serban", "Larisa", "0722000017", "Baia Mare"],
        ["Iancu", "Sorin", "0722000018", "Buzau"],
        ["Voicu", "Alina", "0722000019", "Botosani"],
        ["Dinu", "Marius", "0722000020", "Satu Mare"],
        ["Nedelcu", "Carmen", "0722000021", "Ramnicu Valcea"],
        ["Preda", "Adrian", "0722000022", "Suceava"],
        ["Ilie", "Monica", "0722000023", "Piatra Neamt"],
        ["Petrescu", "Florin", "0722000024", "Drobeta-Turnu Severin"],
        ["Enache", "Raluca", "0722000025", "Targoviste"],
        ["Munteanu", "Daniel", "0722000026", "Focsani"],
        ["Lupu", "Andreea", "0722000027", "Bistrita"],
        ["Cristea", "Ovidiu", "0722000028", "Resita"],
        ["Neagu", "Bianca", "0722000029", "Slatina"],
        ["Toma", "George", "0722000030", "Calarasi"],
        ["Sandu", "Nicoleta", "0722000031", "Alba Iulia"],
        ["Mihai", "Ciprian", "0722000032", "Giurgiu"],
        ["Ciobanu", "Denisa", "0722000033", "Deva"],
        ["Vasile", "Iulian", "0722000034", "Hunedoara"],
        ["Nicolae", "Oana", "0722000035", "Zalau"],
        ["Diaconu", "Paul", "0722000036", "Sfantu Gheorghe"],
        ["Anghel", "Teodora", "0722000037", "Barlad"],
        ["Moldovan", "Sebastian", "0722000038", "Vaslui"],
        ["Cojocaru", "Mihaela", "0722000039", "Roman"],
        ["Manea", "Robert", "0722000040", "Turda"],
        ["Iordache", "Cristina", "0722000041", "Medias"],
        ["Balan", "Stefan", "0722000042", "Slobozia"],
        ["Rusu", "Georgiana", "0722000043", "Alexandria"],
        ["Popa", "Emanuel", "0722000044", "Voluntari"],
        ["Ciobotaru", "Valentina", "0722000045", "Lugoj"],
        ["Grigore", "Marian", "0722000046", "Medgidia"],
        ["Albu", "Camelia", "0722000047", "Onesti"],
        ["Pavel", "Cosmin", "0722000048", "Miercurea Ciuc"],
        ["Dragomir", "Anca", "0722000049", "Sighetu Marmatiei"],
        ["Vlad", "Horia", "0722000050", "Petrosani"],
    ];

    for client in client_array {
        add_client(client[0], client[1], client[2], client[3]).expect("Could not add new client!");
    }

    add_entry_time().expect("Could not add entry times!");
    add_exit_time().expect("Could not add exit times!");
    organizer().expect("Could not organize data!");

    Ok(())
}
