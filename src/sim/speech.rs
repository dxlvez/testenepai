//! Generative speech: lines are assembled from fragments shaped by the
//! speaker's personality, family, faith, money, fear and the situation, so
//! nobody says exactly the same thing twice.

use super::people::*;
use crate::util::Rng;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Situation {
    /// a gun is pointed at them
    Aimed,
    /// being robbed
    Robbed,
    /// tied up
    Tied,
    /// knows they are about to be killed
    Execution,
    /// walking as a hostage
    Hostage,
    /// just got hurt
    Hurt,
    /// dying on the ground
    Dying,
    /// watching someone be attacked
    Witness,
}

fn family_names(p: &Person, pop: &Population) -> (Vec<String>, Option<String>) {
    let kids: Vec<String> = p.children.iter().map(|c| pop.get(*c)).filter(|c| c.alive()).map(|c| c.first.clone()).collect();
    let spouse = p.spouse.map(|s| pop.get(s).first.clone());
    (kids, spouse)
}

fn join_names(v: &[String]) -> String {
    match v.len() {
        0 => String::new(),
        1 => v[0].clone(),
        _ => format!("{} e {}", v[..v.len() - 1].join(", "), v[v.len() - 1]),
    }
}

/// A single unique line for a person in a desperate situation.
pub fn plead(p: &Person, pop: &Population, s: Situation, r: &mut Rng, year: i32) -> String {
    let t = p.traits;
    let (kids, spouse) = family_names(p, pop);
    let brave = t.courage as i32 - t.fear as i32 > 30;
    let devout = t.faith > 70;
    let greedy = t.greed > 65 || p.job.income() >= 7;
    let o = p.o();
    let mut parts: Vec<String> = Vec::new();

    // opener — breath, stutter, disbelief
    let openers: &[&str] = if brave {
        &["Olha aqui.", "Escuta bem.", "Você não sabe com quem está mexendo.", "Tá bom, tá bom.", "Calma lá."]
    } else if t.fear > 70 {
        &["N-não... não, não, não...", "Por favor...", "Ai meu Deus...", "Espera! Espera!", "Eu... eu...", "Moço, pelo amor de Deus..."]
    } else {
        &["Calma. Calma.", "Ei, ei, devagar.", "Não precisa disso.", "Tá bom. Tá bom.", "Espera um pouco."]
    };
    parts.push(r.pick(openers).to_string());

    match s {
        Situation::Aimed | Situation::Robbed => {
            let offers: Vec<String> = vec![
                format!("Leva tudo, leva a carteira, eu tenho {} aqui.", if p.money > 0 { format!("{} trocados", p.money) } else { "pouca coisa".into() }),
                "Pode levar o relógio, é de ouro, era do meu pai.".into(),
                "Não tenho muito, juro, é o dinheiro do aluguel.".into(),
                "Pega o que quiser, só não machuca ninguém.".into(),
                if greedy { "Eu tenho mais em casa. Muito mais. A gente pode conversar.".into() } else { "É tudo que eu tenho, eu juro.".into() },
            ];
            parts.push(r.pick(&offers).clone());
        }
        Situation::Tied => {
            let lines: Vec<String> = vec![
                "Por que você está fazendo isso comigo?".into(),
                "A corda está cortando meus pulsos...".into(),
                "Eu não vi seu rosto, eu juro que não vi nada.".into(),
                "Me solta e eu nunca vou contar pra ninguém.".into(),
                "Alguém vai sentir minha falta. Alguém vai vir.".into(),
                "Eu preciso de água. Por favor. Só um pouco de água.".into(),
                if greedy { "Quanto você quer? Diga um número.".into() } else { "Eu não tenho nada pra te dar...".into() },
            ];
            parts.push(r.pick(&lines).clone());
        }
        Situation::Execution | Situation::Hostage | Situation::Dying => {
            let lines: Vec<String> = vec![
                "Eu não quero morrer.".into(),
                "Não faz isso. Você ainda pode ir embora.".into(),
                "Eu faço qualquer coisa. Qualquer coisa.".into(),
                format!("Eu nunca fiz mal a ninguém, eu sou só um{} {}.", if p.female { "a" } else { "" }, p.job.label(p.female).to_lowercase()),
                "Olha pra mim. Olha nos meus olhos. Eu sou uma pessoa.".into(),
                "Por favor... ainda não...".into(),
            ];
            parts.push(r.pick(&lines).clone());
        }
        Situation::Hurt => {
            parts.push(r.pick(&["Meu braço! Meu braço!", "Você me cortou!", "Por que?!", "Tá sangrando... tá sangrando muito...", "Socorro!"]).to_string());
        }
        Situation::Witness => {
            parts.push(r.pick(&["Alguém chama a polícia!", "Ele vai matar ele!", "Meu Deus, meu Deus!", "Não olha, não olha pra ele!"]).to_string());
        }
    }

    // family: the strongest plea
    if !kids.is_empty() && r.chance(0.7) && !brave {
        let k = join_names(&kids);
        let lines = [
            format!("Eu tenho {}... {} {} esperando em casa.", if kids.len() == 1 { "um filho" } else { "filhos" }, k, if kids.len() == 1 { "está" } else { "estão" }),
            format!("{}. Pensa n{} {}. Quem vai cuidar del{}?", k, if kids.len() == 1 { "o" } else { "os" }, if kids.len() == 1 { "meu menino" } else { "meus meninos" }, if kids.len() == 1 { "e" } else { "es" }),
            format!("Hoje é aniversário d{} {}. Eu prometi que chegava cedo.", if kids.len() == 1 { "o" } else { "a" }, kids[0]),
        ];
        parts.push(r.pick(&lines).clone());
    } else if let Some(sp) = spouse {
        if r.chance(0.5) && !brave {
            parts.push(r.pick(&[format!("{} vai ficar sozinh{}...", sp, if p.female { "o" } else { "a" }), format!("Deixa eu ver {} só mais uma vez.", sp), format!("{} está me esperando pro jantar.", sp)]).clone());
        }
    }
    if devout && r.chance(0.6) {
        parts.push(r.pick(&["Ave Maria, cheia de graça...", "Deus está vendo você.", "Que Nossa Senhora tenha piedade de nós.", "Pai nosso que estais no céu...", "Eu rezo por você. Eu rezo."]).to_string());
    }
    if brave && r.chance(0.6) {
        parts.push(r.pick(&["Se for pra atirar, atira olhando nos meus olhos.", "A polícia vai te caçar feito cachorro.", "Você não tem coragem.", "Meu irmão vai te encontrar."]).to_string());
    }
    if p.debt > 100 && r.chance(0.2) {
        parts.push("Se foi o Zeller que te mandou, diz pra ele que eu pago semana que vem!".into());
    }
    let _ = (year, o);
    parts.join(" ")
}

/// Street / bar conversation between two people: builds a unique sentence
/// from topics around them.
pub fn chatter(sp: &Person, other: &Person, pop: &Population, rumor: Option<&str>, year: i32, r: &mut Rng) -> String {
    if let Some(ru) = rumor {
        if r.chance(0.5) {
            let intro = r.pick(&["Você soube? ", "Dizem por aí que ", "Não conta pra ninguém, mas ", "Ouvi no mercado que ", "Minha vizinha jura que ", "É verdade que "]);
            return format!("{}{}", intro, super::lines::lower_first(ru));
        }
    }
    let topic = r.idx(9);
    match topic {
        0 => {
            // a mutual acquaintance
            if let Some(rel) = sp.rels.iter().filter(|x| x.to != other.id).nth(r.idx(sp.rels.len().max(1))) {
                let q = pop.get(rel.to);
                let what = match rel.kind {
                    RelKind::Friend => r.pick(&["anda sumido", "me emprestou dinheiro de novo", "está pensando em casar", "tá bebendo demais", "arrumou emprego novo"]).to_string(),
                    RelKind::Rival | RelKind::Enemy => r.pick(&["me olhou torto de novo", "anda falando de mim pela cidade", "vai ter o que merece", "é uma cobra"]).to_string(),
                    RelKind::Lover => r.pick(&["...não, nada, esquece", "tem uns olhos que...", "vai estar no baile sábado"]).to_string(),
                    RelKind::Debtor => "ainda me deve e fica fugindo de mim".into(),
                    RelKind::Creditor => "está me cobrando de novo".into(),
                    _ => r.pick(&["mudou muito", "anda estranho", "perguntou de você"]).to_string(),
                };
                return format!("{} {}.", q.first, what);
            }
            super::lines::era_small_talk(year)[r.idx(super::lines::era_small_talk(year).len())].to_string()
        }
        1 => {
            let (kids, _) = family_names(sp, pop);
            if !kids.is_empty() {
                let k = &kids[r.idx(kids.len())];
                return r.pick(&[
                    format!("{} tirou nota boa na escola, acredita?", k),
                    format!("{} está com febre de novo. Não sei mais o que fazer.", k),
                    format!("{} quer ser músico. Músico! Imagina.", k),
                    format!("Peguei {} fumando atrás da igreja.", k),
                    format!("{} perguntou por que o céu fica vermelho às vezes.", k),
                ])
                .clone();
            }
            r.pick(&["Às vezes eu queria ter tido filhos.", "Casa vazia é casa triste.", "Minha mãe não para de perguntar quando eu caso."]).to_string()
        }
        2 => {
            // work
            let w = sp.job.label(sp.female).to_lowercase();
            r.pick(&[
                format!("Trabalhar de {} nessa cidade é pra quem tem estômago.", w),
                "O patrão cortou meu pagamento de novo.".to_string(),
                "Amanhã eu pego cedo. Não sei como aguento.".to_string(),
                "Tô pensando em largar tudo e ir embora.".to_string(),
                format!("Hoje apareceu um sujeito estranho procurando {}.", if sp.work.is_some() { "o patrão" } else { "trabalho" }),
            ])
            .clone()
        }
        3 => {
            // weather / city mood
            r.pick(&[
                "Essa chuva não para. Parece castigo.",
                "Tem uma névoa estranha vindo do rio à noite.",
                "Você sentiu o chão tremer ontem de madrugada? Eu senti.",
                "O céu estava vermelho no fim da tarde. Vermelho sangue.",
                "Esfriou de repente. Quase pude ver minha respiração.",
            ])
            .to_string()
        }
        4 => {
            // faith / superstition
            if sp.traits.faith > 60 {
                r.pick(&["O padre disse que o fim está perto.", "Acendi uma vela pra minha avó.", "Sonhei com uma esfera. Uma bola escura, girando. Acordei chorando."]).to_string()
            } else {
                r.pick(&["Igreja? Só pra casamento e enterro.", "Deus tirou férias dessa cidade.", "Minha sorte é a única religião que eu tenho."]).to_string()
            }
        }
        5 => {
            // money
            if sp.debt > 50 {
                format!("Devo {} pra gente perigosa. Se eu sumir, você sabe por quê.", sp.debt)
            } else if sp.job.income() >= 7 {
                r.pick(&["Os negócios vão bem. Bem até demais.", "Comprei um chapéu novo. Italiano.", "Dinheiro atrai urubu, sabe?"]).to_string()
            } else {
                r.pick(&["Tá tudo caro. Até o pão.", "Se eu tivesse cinco dólares a mais...", "Joguei no bicho de novo. Perdi de novo."]).to_string()
            }
        }
        6 => {
            // about the listener
            r.pick(&[
                format!("{}, você está com uma cara péssima.", other.first),
                format!("Você não mudou nada, {}.", other.first),
                format!("{}, me paga uma bebida que eu te conto.", other.first),
                format!("Sua mãe está bem, {}?", other.first),
            ])
            .clone()
        }
        7 => {
            // strangers & the uncanny (Elias leaves marks in the world)
            r.pick(&[
                "Tem um homem de roupa esquisita andando por aí, fazendo pergunta.",
                "Minha vizinha jura que viu a mesma pessoa em dois lugares ao mesmo tempo.",
                "O rádio chiou e falou um nome. Parecia 'Elias'.",
                "Tem dias que eu acordo com a sensação de que isso tudo já aconteceu.",
            ])
            .to_string()
        }
        _ => super::lines::era_small_talk(year)[r.idx(super::lines::era_small_talk(year).len())].to_string(),
    }
}

/// Short reply to the chatter (so conversations flow).
pub fn reply(listener: &Person, r: &mut Rng) -> String {
    let t = listener.traits;
    if t.sociability < 30 {
        r.pick(&["Hm.", "Sei.", "Se você diz.", "Não quero saber disso."]).to_string()
    } else if t.curiosity > 70 {
        r.pick(&["Sério? Me conta tudo.", "Não acredito! E aí?", "Quem te contou isso?", "E ninguém fez nada?"]).to_string()
    } else if t.faith > 75 {
        r.pick(&["Deus nos proteja.", "Vou rezar por isso.", "Isso é coisa do demônio."]).to_string()
    } else if t.morality < 25 {
        r.pick(&["Bem feito.", "Cada um com seus problemas.", "Isso dá pra ganhar dinheiro, sabia?"]).to_string()
    } else {
        r.pick(&["Nossa.", "É, a vida tá difícil.", "Verdade.", "Ha! Essa é boa.", "Nem me fale.", "Cuidado com isso."]).to_string()
    }
}

/// Things people say when they spot Elias doing something.
pub fn react_to_weapon(p: &Person, r: &mut Rng) -> String {
    if p.traits.courage > 75 {
        r.pick(&["Abaixa isso antes que se machuque.", "Tá apontando isso pra quem?", "Você não é daqui, né? Aqui a gente resolve diferente."]).to_string()
    } else {
        r.pick(&["Ele está armado!", "Abaixa isso, moço, pelo amor de Deus!", "Não atira! Não atira!", "Corre, corre!"]).to_string()
    }
}
