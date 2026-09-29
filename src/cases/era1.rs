//! ERA I — 1920–1930 — Nova Orleans.

use super::defs::*;
use crate::city::gen::CityId;
use crate::city::map::BKind;
use crate::sim::people::Job;
use Place::*;

pub fn case01() -> CaseDef {
    CaseDef {
        id: 1,
        title: "O Homem do Machado",
        city: CityId::NewOrleans,
        year: 1920,
        intro: "Nova Orleans, 1920. Há dois anos um assassino invade casas pelas portas dos fundos, sempre de madrugada, sempre com um machado. Esta semana foi a vez de Giuseppe Cavaretta, dono de uma mercearia na Rua Royal. A esposa sobreviveu. A cidade inteira toca jazz à noite porque uma carta ao jornal prometeu poupar quem tocasse.",
        brief: "Giuseppe Cavaretta foi morto a machadadas em sua casa. A esposa, Rosa, sobreviveu ferida. A porta dos fundos foi aberta com um cinzel. Uma carta assinada 'Do Inferno' chegou ao jornal.",
        start: HomeOf(0),
        cast: vec![
            person("Giuseppe", "Cavaretta", false, 51, Job::Merchant, Role::Victim, B(BKind::House, 3), "Dono de mercearia. Morto a machadadas na própria cama.").dead(),
            person("Rosa", "Cavaretta", true, 46, Job::Merchant, Role::Witness, B(BKind::House, 3), "Esposa de Giuseppe. Sobreviveu com um corte no ombro. Tem medo de falar.")
                .temper(80, 30, 70, 30)
                .topics(vec![
                    topic("noite", "O que a senhora viu naquela noite?", "Acordei com a porta rangendo... Vi uma sombra alta, com um quepe. Igual ao dos guardas. Depois só o grito do Giuseppe. Não conte a ninguém que eu disse isso.")
                        .lie("Nada. Eu dormia. Acordei com o sangue.", 10)
                        .reveals(&[6]),
                    topic("inimigos", "Seu marido tinha inimigos?", "Giuseppe era bom com todo mundo. Só o açougueiro Picard vivia implicando. E um guarda vinha toda sexta-feira buscar 'a contribuição'. Giuseppe mandou ele embora na última vez.").reveals(&[7]),
                    topic("relogio", "O relógio da cozinha parou às 3:17. E essas datas riscadas?", "Aquele relógio nunca parou em vinte anos. E essas datas... não fui eu. Eu nem sei ler direito. Juro por Nossa Senhora.").req(Req::Clue(11)),
                ]),
            person("Auguste", "Picard", false, 44, Job::Butcher, Role::Suspect, B(BKind::House, 7), "Açougueiro do Mercado Francês. Brigava com Giuseppe por fregueses.")
                .temper(30, 70, 45, 60)
                .topics(vec![
                    topic("briga", "Soube que o senhor brigou com Giuseppe.", "Brigamos por freguês, sim! Ele baixava o preço da linguiça só pra me irritar. Mas eu mato porco, moço. Não gente."),
                    topic("machado", "Seu machado de carne estava sujo de sangue.", "É sangue de porco! Leve ao hospital, a doutora olha no microscópio. Eu não tenho nada a esconder.").req(Req::Clue(3)),
                    topic("noite", "Onde o senhor estava às três da manhã?", "Na cama, do lado da minha mulher, roncando. O bairro inteiro ouve eu roncar."),
                ]),
            person("Lucien", "Toussaint", false, 29, Job::Musician, Role::Suspect, B(BKind::Apartment, 2), "Trompetista do Clube Blue Parrot. Tocou até o amanhecer na noite do crime.")
                .temper(40, 50, 60, 40)
                .topics(vec![
                    topic("carta", "A carta do assassino fala de jazz. O que acha disso?", "Acho que alguém gosta de ser famoso. Mas vou te contar uma coisa estranha: naquela noite um homem me pagou vinte dólares pra eu tocar até o sol nascer. Um homem de uniforme, com o quepe baixo.").reveals(&[13]),
                    topic("uniforme", "Como era o homem de uniforme?", "Alto. Bigode. Anel de ouro com uma pedra vermelha. Não perguntei o nome, vinte dólares não fazem pergunta.").req(Req::Topic("carta")),
                ]),
            person("Walter", "Hale", false, 42, Job::Police, Role::Suspect, B(BKind::House, 11), "Guarda da Delegacia Central. Faz a ronda da Rua Royal há dez anos. Usa um anel com pedra vermelha.")
                .temper(20, 80, 15, 85)
                .flees()
                .topics(vec![
                    topic("ronda", "Onde o senhor estava na madrugada do ataque?", "...Tá bom. Passei na Rua Royal. Tinha uma dívida pra cobrar, só isso. Quando saí, o velho estava vivo. Vivo!")
                        .lie("Fazendo ronda no porto, como sempre. Pergunte a qualquer um.", 8),
                    topic("caderno", "O que é este caderno com nomes de comerciantes?", "Isso é contabilidade da delegacia, forasteiro. Você não entende como esta cidade funciona. E se continuar perguntando, vai entender do pior jeito.").req(Req::Clue(4)),
                    topic("anel", "Um músico disse que um guarda de anel vermelho pagou pra ele tocar a noite toda.", "Músico bêbado inventa história. Metade da cidade tem anel.").req(Req::Clue(13)),
                ]),
            person("Clara", "Bell", true, 31, Job::Journalist, Role::Contact, B(BKind::Apartment, 5), "Repórter do jornal. Cobre o caso do Homem do Machado e não acredita na versão da polícia.")
                .temper(20, 85, 75, 30)
                .soul(1)
                .topics(vec![
                    topic("axeman", "O que você sabe sobre o Homem do Machado?", "Seis ataques em dois anos. Todos comerciantes italianos. Todos pela porta dos fundos, com um cinzel. E todos se recusavam a pagar alguém. A polícia diz que é um louco. Loucos não escolhem quem não paga.").reveals(&[14]),
                    topic("carta", "E a carta 'Do Inferno'?", "Chegou à redação sem selo, alguém deixou na porta. Papel bom demais pra um louco. Está no arquivo do jornal, pode ir ver."),
                    topic("voce", "Por que você se importa tanto, Clara?", "Porque ninguém mais se importa. E você, Elias? Você fala como quem já leu o final do livro. Às vezes eu sonho com alguém que fala assim."),
                ]),
            person("Samuel", "Reed", false, 37, Job::Dockworker, Role::Witness, B(BKind::House, 4), "Estivador, vizinho dos Cavaretta. Volta do cais de madrugada.")
                .temper(70, 40, 60, 55)
                .topics(vec![
                    topic("noite", "O senhor viu algo na noite do crime?", "Eu voltava do cais, lá pelas três. Vi um sujeito sair pelos fundos dos Cavaretta. Botas pesadas, capa escura, andar de quem manda na rua. Não vi o rosto, juro.")
                        .lie("Não vi nada. Eu durmo cedo. Não quero confusão com ninguém.", 6)
                        .reveals(&[8]),
                ]),
            person("Marguerite", "Hale", true, 39, Job::Housewife, Role::Witness, B(BKind::House, 11), "Esposa de Walter Hale. Fala baixo e olha para a porta o tempo todo.")
                .temper(85, 25, 70, 20)
                .topics(vec![
                    topic("marido", "Onde seu marido estava naquela madrugada?", "Ele chegou às quatro. Cheirava a ferro. Lavou a camisa sozinho no tanque... Walter nunca lavou nada na vida. Por favor, não diga que fui eu.")
                        .lie("Em casa. Comigo. A noite toda.", 5)
                        .reveals(&[9]),
                ]),
        ],
        clues: vec![
            clue("Porta dos fundos arrombada", ClueKind::Physical, Some(HomeOf(0)), Look3d::Object, "Um painel inferior da porta foi removido com um cinzel. O mesmo método de todos os ataques.", &[])
                .forensic("As marcas são limpas e pacientes. O invasor não tinha pressa: sabia que nenhuma patrulha passaria ali naquela hora."),
            clue("Machado da cozinha", ClueKind::Physical, Some(HomeOf(0)), Look3d::Weapon, "O machado era da própria casa, usado para rachar lenha. Foi largado no quintal.", &[])
                .forensic("Sangue humano. No cabo, lama cinzenta misturada com serragem — a mesma dos estábulos da delegacia."),
            clue("Carta 'Do Inferno'", ClueKind::Document, Some(B(BKind::Newspaper, 0)), Look3d::Paper, "'Serei poupado em toda casa onde tocarem jazz.' Assinada: Do Inferno.", &[3])
                .forensic("O papel teve o timbre raspado com lâmina. Sob a luz dá para ver: é papel oficial da Delegacia Central."),
            clue("Machado de carne de Picard", ClueKind::Physical, Some(WorkOf(2)), Look3d::Weapon, "Um machado de açougue com sangue seco no fio.", &[2])
                .forensic("Pelos e gordura suína. É sangue de porco.")
                .herring(),
            clue("Caderno de 'contribuições'", ClueKind::Document, Some(B(BKind::Police, 0)), Look3d::Paper, "Um caderno com nomes de comerciantes italianos e valores semanais. Ao lado de 'Cavaretta', em vermelho: RECUSOU.", &[4])
                .hidden(2)
                .network(Job::Police),
            clue("Camisa de uniforme no varal", ClueKind::Physical, Some(HomeOf(4)), Look3d::Object, "Uma camisa de uniforme, lavada às pressas. A gola ainda está rosada.", &[4]).hidden(1),
            testimony("Rosa viu um quepe", "Rosa Cavaretta viu uma sombra alta usando um quepe de guarda.", &[4]),
            testimony("A briga com Picard", "Giuseppe e o açougueiro Picard brigavam por fregueses. E um guarda cobrava 'contribuição' toda sexta.", &[2, 4]),
            testimony("Samuel viu as botas", "Às 3h, Samuel viu um homem de botas pesadas e capa escura saindo pelos fundos.", &[4]),
            testimony("Walter chegou às quatro", "Marguerite: Walter chegou às 4h cheirando a ferro e lavou a própria camisa.", &[4]),
            clue("Eco: a porta dos fundos", ClueKind::Temporal, Some(HomeOf(0)), Look3d::None, "No eco, uma silhueta de quepe entra com um cinzel. Um relógio marca 3:17.", &[4]),
            clue("Relógio parado às 3:17", ClueKind::Physical, Some(HomeOf(0)), Look3d::Glow, "O relógio da cozinha parou às 3:17. No mostrador, riscadas a unha: 1921, 1946, 1971.", &[])
                .hidden(1),
            clue("Pasta de ataques futuros", ClueKind::Document, Some(B(BKind::Police, 0)), Look3d::Glow, "Uma pasta de ocorrências do Homem do Machado. A última está datada de amanhã. A hora: 3:17.", &[])
                .hidden(3),
            testimony("O homem que pagou o jazz", "Lucien recebeu vinte dólares de um homem de uniforme, com anel de pedra vermelha, para tocar a noite toda.", &[4]),
            testimony("Todos se recusavam a pagar", "Clara: todas as vítimas eram comerciantes que se recusavam a pagar 'alguém'.", &[4]),
        ],
        echoes: vec![
            echo(HomeOf(0), Ghost::Struggle, &["Uma porta range no escuro.", "Um quepe contra a luz do lampião.", "O relógio: 3:17.", "Um machado sobe."], 10),
            echo(WorkOf(2), Ghost::Argue, &["Dois homens discutem sobre preços.", "'Você vai se arrepender, Cavaretta!'", "Um aperto de mão, depois. Relutante."], 7),
        ],
        culprit: 4,
        methods: &[
            "Machado de carne roubado do açougue de Picard",
            "Entrou pela porta dos fundos com um cinzel e usou o machado da própria casa",
            "Navalha de barbeiro, depois simulou o machado",
            "Pistola com silenciador",
        ],
        method: 1,
        motives: &[
            "Ciúme — era apaixonado por Rosa",
            "Dívida de jogo nunca paga",
            "Giuseppe se recusou a pagar 'proteção' e ameaçou denunciar o esquema",
            "Ritual ocultista inspirado no jazz",
        ],
        motive: 2,
        anomalies: &[
            "Nenhuma — é apenas um crime",
            "O relógio parado e a pasta mostram ataques marcados para a mesma hora (3:17) — inclusive ataques que ainda não aconteceram",
            "O machado tinha ferrugem de cinquenta anos",
            "A carta foi escrita pela própria vítima",
        ],
        anomaly: 1,
        threads: &[(4, 0), (0, 1), (4, 7), (4, 3), (5, 1)],
        story: "Walter Hale comandava um esquema de 'proteção' contra os comerciantes italianos. Quem se recusava a pagar recebia a visita do Homem do Machado — e a carta sobre o jazz era o disfarce perfeito para parecer obra de um louco. Giuseppe recusou e ameaçou ir ao jornal.",
        sphere: "Os ataques não aconteceram 'antes de acontecer' por acaso: 3:17 é a hora em que a esfera toca esta linha do tempo. Em todas as versões de Nova Orleans, alguém morre às 3:17. Você só mudou quem.",
        on_true: outcome(
            "Walter Hale foi preso. O caderno de 'contribuições' foi parar na primeira página. Marguerite deixou a cidade com as filhas. Os comerciantes italianos pararam de pagar.",
            "GUARDA DA DELEGACIA CENTRAL É PRESO COMO O 'HOMEM DO MACHADO'",
            "...e a delegacia central afirma que o caso do Homem do Machado está encerrado...",
            vec![(4, "jailed"), (7, "left_city"), (1, "survived"), (5, "grateful")],
        )
        .flags(vec!["axeman_caught"]),
        on_false: outcome(
            "A cidade comemorou a prisão. Três semanas depois, outra porta dos fundos amanheceu arrombada. O Homem do Machado continuou.",
            "PRESO O 'HOMEM DO MACHADO'. MAS A CIDADE CONTINUA TOCANDO JAZZ",
            "...novo ataque na madrugada. A polícia fala em imitador...",
            vec![(4, "criminal")],
        )
        .flags(vec!["axeman_free"]),
        digit: "2",
        reward: 60,
    }
}

pub fn rest() -> Vec<CaseDef> {
    vec![]
}
