//! Ambient lines people say to each other (and to Elias) on the street.

use super::people::{Job, Person, RelKind};
use crate::util::Rng;

pub fn era_small_talk(year: i32) -> &'static [&'static str] {
    if year < 1930 {
        &[
            "Dizem que a Lei Seca vai durar pra sempre.",
            "O rádio da vizinha tocou jazz a noite inteira.",
            "O bonde atrasou de novo hoje cedo.",
            "Meu primo foi pra Chicago. Diz que lá tem trabalho.",
            "Você viu o preço do café? Um roubo.",
            "Essa umidade vai me matar antes do inverno.",
            "O padre falou do demônio no sermão. De novo.",
            "Tranca a porta hoje, ouviu? Tranca bem.",
            "O cais anda cheio de gente estranha à noite.",
            "Minha mãe diz que sonhou com um homem sem rosto.",
            "Ouvi um trompete lá no Tremé que fez eu chorar.",
            "Tem cheiro de chuva vindo do rio.",
        ]
    } else if year < 1946 {
        &[
            "Não há emprego, não há dinheiro, não há nada.",
            "Os caras do sindicato apanharam de novo ontem.",
            "Meu irmão está na fila da sopa desde as cinco.",
            "Dizem que a guerra na Europa não vai chegar aqui.",
            "O rádio disse que vai ter racionamento.",
            "Esse chefe da polícia come na mão da máfia.",
            "Você ouviu o programa de mistério ontem? Arrepiante.",
            "Meu vizinho sumiu. Deixou até o chapéu.",
            "Se você tem um emprego, segura ele com os dentes.",
            "O lago está congelado de novo, olha só.",
        ]
    } else if year < 1960 {
        &[
            "Os russos têm a bomba agora. A gente faz o quê?",
            "Meu marido voltou da guerra, mas não voltou inteiro.",
            "Comprei uma geladeira nova. Um luxo!",
            "Dizem que tem espião até no correio.",
            "A praia estava cheia domingo passado.",
            "Tem um homem que caminha na orla toda noite, sozinho.",
            "O jornal publicou um código que ninguém decifra.",
            "Televisão é moda. Eu prefiro o rádio.",
        ]
    } else if year < 1980 {
        &[
            "Paz e amor, cara. Mas tranca o carro.",
            "Mais uma carta pro jornal. Ninguém dorme direito.",
            "Meu filho foi convocado pro Vietnã.",
            "A polícia está parando todo mundo de cabelo comprido.",
            "Ouviu o disco novo? Mudou minha vida.",
            "Não pegue carona com estranhos. Sério.",
            "O avião sumiu com todo mundo dentro, dá pra acreditar?",
            "Meu analista diz que o problema é a minha mãe.",
        ]
    } else {
        &[
            "O metrô virou uma selva depois das dez.",
            "Perdi dinheiro na bolsa de novo.",
            "Tem um cara vendendo computador na rua 14.",
            "Não vai pro parque à noite, ouviu?",
            "Meu bipe não para de tocar.",
            "Mais uma criança desaparecida no noticiário.",
            "Esse aluguel vai me mandar pra rua.",
            "A cidade nunca dorme. Nem eu.",
        ]
    }
}

pub fn friend_lines() -> &'static [&'static str] {
    &[
        "E aí, como vai a família?",
        "Sábado tem baile, você vem?",
        "Me paga uma bebida e te conto um segredo.",
        "Você está com uma cara de quem não dorme.",
        "Ha! Você não muda nunca.",
        "Lembra quando a gente era criança aqui?",
        "Cuidado com quem você anda.",
        "Eu te devo uma, não esqueci.",
    ]
}

pub fn rival_lines() -> &'static [&'static str] {
    &[
        "Olha quem apareceu...",
        "Você ainda me deve explicações.",
        "Fica longe da minha família.",
        "Um dia você vai pagar.",
        "Não tenho nada pra falar com você.",
        "Vai cuidar da sua vida.",
    ]
}

pub fn lover_lines() -> &'static [&'static str] {
    &[
        "Aqui não... alguém pode ver.",
        "Te vejo mais tarde, no lugar de sempre.",
        "Sonhei com você de novo.",
        "Um dia a gente vai embora daqui.",
    ]
}

pub fn job_lines(j: Job) -> &'static [&'static str] {
    match j {
        Job::Police | Job::Detective => &["Cidade podre.", "Mais um corpo esta semana.", "O chefe quer um culpado, qualquer um.", "Circulando, circulando."],
        Job::Musician => &["Toquei até as quatro ontem.", "Essa melodia não sai da minha cabeça.", "Um homem me pediu uma música que eu nunca compus."],
        Job::Bartender => &["O que vai ser?", "Aqui ninguém viu nada, entendeu?", "Fecho às três. Às três."],
        Job::Priest => &["Deus ouve até o que você não diz.", "Acendi uma vela pelos mortos da semana."],
        Job::Journalist => &["Tenho uma matéria que vai derrubar gente grande.", "O editor cortou minha melhor linha de novo."],
        Job::Doctor | Job::Nurse => &["Chegaram três feridos só essa noite.", "O necrotério está lotado."],
        Job::Gangster | Job::Smuggler => &["Tá olhando o quê?", "Chegou carga nova no cais.", "Quem fala demais, nada no rio."],
        Job::Dockworker => &["Minhas costas estão acabadas.", "O navio de ontem trouxe caixas sem nome."],
        Job::Vendor | Job::Merchant | Job::Butcher | Job::Baker => &["Freguesia fraca hoje.", "Tudo fresquinho!", "Fiado só amanhã."],
        _ => &[],
    }
}

/// A line one person says to another in a street conversation.
pub fn chat_line(r: &mut Rng, speaker: &Person, other: &Person, year: i32, rumor: Option<&str>) -> String {
    if let Some(ru) = rumor {
        if r.chance(0.6) {
            let intro = *r.pick(&["Você soube? ", "Dizem por aí que ", "Não conta pra ninguém, mas ", "Ouvi no bar que ", "É verdade que "]);
            return format!("{}{}", intro, lower_first(ru));
        }
    }
    let rel = speaker.rel_to(other.id).map(|r| r.kind);
    let pool: &[&str] = match rel {
        Some(RelKind::Friend) | Some(RelKind::Coworker) => {
            if r.chance(0.5) {
                friend_lines()
            } else {
                era_small_talk(year)
            }
        }
        Some(RelKind::Rival) | Some(RelKind::Enemy) => rival_lines(),
        Some(RelKind::Lover) => lover_lines(),
        Some(RelKind::Debtor) => &["Eu vou te pagar, só preciso de mais uma semana.", "Não me cobra aqui na rua!"],
        Some(RelKind::Creditor) => &["E o meu dinheiro?", "Semana que vem eu vou buscar, com ou sem educação."],
        _ => {
            let jl = job_lines(speaker.job);
            if !jl.is_empty() && r.chance(0.35) {
                jl
            } else {
                era_small_talk(year)
            }
        }
    };
    r.pick(pool).to_string()
}

pub fn lower_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_lowercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

/// What a passer-by mutters about Elias.
pub fn about_elias(r: &mut Rng, out_of_time: bool, wanted: bool, known: bool, trust: i32, alias: &str) -> Option<String> {
    if wanted && r.chance(0.5) {
        return Some(r.pick(&["É ele... o do jornal!", "Não olha, não olha pra ele.", "Alguém chama a polícia!"]).to_string());
    }
    if out_of_time && r.chance(0.6) {
        return Some(
            r.pick(&[
                "Que roupa é essa, moço?",
                "Você não é daqui, é?",
                "Olha os trajes desse aí...",
                "É algum tipo de palhaço?",
                "Mãe, olha o homem esquisito!",
            ])
            .to_string(),
        );
    }
    if known && trust > 20 {
        return Some(format!("{} {}!", r.pick(&["Boa noite,", "Olá,", "Ei,", "Como vai,"]), alias));
    }
    if known && trust < -20 {
        return Some(r.pick(&["Hunf.", "Você de novo.", "Não quero conversa."]).to_string());
    }
    None
}
