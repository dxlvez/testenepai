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
    vec![case02(), case03(), case04(), case05()]
}

pub fn case02() -> CaseDef {
    CaseDef {
        id: 2,
        title: "O Menino na Parede",
        city: CityId::NewOrleans,
        year: 1922,
        intro: "Nova Orleans, 1922. Henri Duval, oito anos, desapareceu há onze dias. O pai jura que o menino fugiu. A mãe jura que ele nunca saiu de casa. Os vizinhos contam versões diferentes. E a casa dos Duval tem uma parede que ninguém lembra de ter visto construir.",
        brief: "Henri Duval, 8 anos, sumiu. Pai: 'fugiu'. Mãe: 'nunca saiu'. Na casa há uma parede de tijolos recente, ainda sem pintura.",
        start: HomeOf(1),
        cast: vec![
            person("Henri", "Duval", false, 8, Job::Child, Role::Victim, B(BKind::House, 5), "Oito anos. Desenhava esferas escuras nas paredes do quarto.").dead(),
            person("Armand", "Duval", false, 41, Job::Dockworker, Role::Suspect, B(BKind::House, 5), "O pai. Estivador, bebe demais, perdeu dinheiro no jogo.")
                .temper(35, 60, 35, 60)
                .topics(vec![
                    topic("fuga", "Para onde o senhor acha que Henri fugiu?", "Pro porto. Ele sempre quis entrar num navio. Menino teimoso.").lie("Pro porto. Ele sempre quis entrar num navio. Menino teimoso.", 3),
                    topic("parede", "Quem construiu a parede nova do quarto?", "Eu. Tinha rachadura, entrava rato. Terminei na semana passada... na semana em que ele sumiu. Coincidência.").req(Req::Clue(0)),
                    topic("divida", "Soube que o senhor deve dinheiro a Remy Castille.", "Devo. Todo mundo deve ao Castille. Ele ameaçou levar 'o que eu tivesse de mais valioso'. Eu achei que falava da casa.").req(Req::Clue(6)).reveals(&[9]),
                ]),
            person("Odile", "Duval", true, 36, Job::Housewife, Role::Witness, B(BKind::House, 5), "A mãe. Não dorme. Ouve batidas atrás da parede de madrugada.")
                .temper(80, 40, 80, 20)
                .topics(vec![
                    topic("noite", "O que aconteceu na noite em que Henri sumiu?", "Eu o coloquei na cama às oito. Às três e dezessete eu acordei com a porta da frente aberta. A cama dele estava quente. Ele não saiu. Ele não sairia sem o soldadinho de chumbo.").reveals(&[4]),
                    topic("batidas", "A senhora ouve batidas na parede?", "Toda noite. Três batidas. Armand diz que é o encanamento. Não tem encanamento naquela parede.").reveals(&[5]),
                ]),
            person("Remy", "Castille", false, 50, Job::Gangster, Role::Suspect, B(BKind::Mansion, 1), "Agiota do French Quarter. Cobra dívidas de um jeito que ninguém esquece.")
                .temper(10, 85, 10, 95)
                .flees()
                .topics(vec![
                    topic("divida", "Armand Duval lhe devia dinheiro?", "Devia e deve. Mas eu não mexo com criança, forasteiro. Criança não paga dívida.").lie("Nunca ouvi falar de nenhum Duval.", 6),
                    topic("menino", "Onde o senhor estava na noite do sumiço?", "No meu clube, com vinte testemunhas. Pergunte ao pianista.").reveals(&[8]),
                ]),
            person("Josephine", "Batiste", true, 67, Job::Retired, Role::Witness, B(BKind::House, 6), "Vizinha. Vê tudo pela janela. Diz que tem 'o dom'.")
                .temper(30, 60, 70, 20)
                .topics(vec![
                    topic("viu", "A senhora viu algo naquela noite?", "Vi o menino na janela às três e pouco, acenando pra alguém na rua. Não era o pai. Era um homem de sobretudo, parado debaixo do poste, olhando o relógio.").reveals(&[7]),
                    topic("dom", "Que 'dom' é esse?", "Eu sonho com as coisas antes. Sonhei com aquela parede antes de existir. E sonhei com você, moço. Você estava mais velho.").req(Req::Topic("viu")),
                ]),
            person("Lucien", "Toussaint", false, 31, Job::Musician, Role::Contact, B(BKind::Apartment, 2), "O trompetista. Toca no clube de Castille.")
                .temper(40, 50, 60, 40)
                .topics(vec![topic("castille", "Castille estava no clube naquela noite?", "A noite toda, sentado na mesa do canto. Mas o motorista dele saiu às duas e voltou às quatro, encharcado.").reveals(&[10])]),
        ],
        clues: vec![
            clue("Parede de tijolos nova", ClueKind::Physical, Some(HomeOf(1)), Look3d::Object, "Uma parede recente no quarto do menino. A argamassa ainda está úmida por dentro.", &[1])
                .forensic("O tijolo foi assentado de dentro para fora — como se alguém estivesse do lado de dentro quando a parede foi erguida."),
            clue("Soldadinho de chumbo", ClueKind::Physical, Some(HomeOf(1)), Look3d::Object, "O brinquedo preferido de Henri, caído atrás da cama.", &[]),
            clue("Desenhos de esferas", ClueKind::Document, Some(HomeOf(1)), Look3d::Glow, "Folhas e folhas com a mesma bola escura com pontos vermelhos. Na última, um homem de sobretudo segurando a mão de um menino.", &[]).hidden(1),
            clue("Bilhete de embarque", ClueKind::Document, Some(Docks), Look3d::Paper, "Um bilhete infantil para o vapor 'Natchez', comprado por Armand dois dias antes do sumiço. Nunca usado.", &[1]).herring(),
            testimony("A cama ainda quente", "Odile: às 3:17 a porta estava aberta e a cama de Henri, quente.", &[]),
            testimony("Três batidas", "Odile ouve três batidas atrás da parede toda madrugada.", &[1]),
            clue("Livro de dívidas de Castille", ClueKind::Document, Some(B(BKind::Mansion, 1)), Look3d::Paper, "'Duval — 400 dólares — cobrar o que tiver de mais valioso.'", &[3]).network(Job::Gangster),
            testimony("O homem sob o poste", "Josephine viu Henri acenando para um homem de sobretudo sob o poste, às 3h.", &[3]),
            testimony("Álibi de Castille", "Castille passou a noite no clube, com testemunhas.", &[]),
            testimony("Ameaça de Castille", "Armand: Castille prometeu levar 'o que ele tivesse de mais valioso'.", &[3]),
            testimony("O motorista encharcado", "O motorista de Castille saiu às duas e voltou às quatro, encharcado.", &[3]),
            clue("Eco: o quarto do menino", ClueKind::Temporal, Some(HomeOf(1)), Look3d::None, "No eco, o menino abre a janela e desce por uma corda. Embaixo, o motorista de Castille o recebe.", &[3]),
            clue("Recibo do orfanato de Baton Rouge", ClueKind::Document, Some(B(BKind::Office, 0)), Look3d::Paper, "Um menino 'sem nome' foi entregue num orfanato em Baton Rouge dois dias após o sumiço. Pagamento: 400 dólares.", &[3]).hidden(2).network(Job::Journalist),
        ],
        echoes: vec![
            echo(HomeOf(1), Ghost::Flee, &["Uma janela range.", "Um menino desce por uma corda, em silêncio.", "Um homem de chapéu o recebe na calçada.", "O relógio da igreja: três e dezessete."], 11),
        ],
        culprit: 3,
        methods: &["O pai matou o menino e o emparedou", "O motorista de Castille o tirou pela janela e o vendeu a um orfanato", "O menino fugiu sozinho para o porto", "A mãe escondeu o filho do pai"],
        method: 1,
        motives: &["Cobrar a dívida de Armand com o que ele tinha de mais valioso", "Vingança de uma antiga amante", "Resgate", "O menino viu algo que não devia"],
        motive: 0,
        anomalies: &["Nenhuma", "Henri desenhou a esfera e o homem de sobretudo semanas antes — e as batidas na parede vêm de um quarto que existirá em outra linha do tempo", "O soldadinho de chumbo é de 1990", "A mãe é a mesma mulher em 1971"],
        anomaly: 1,
        threads: &[(3, 0), (1, 3), (0, 4), (2, 0)],
        story: "Castille cobrou a dívida de Armand do jeito mais cruel: mandou o motorista buscar o menino pela janela e o vendeu a um orfanato de Baton Rouge por 400 dólares. Armand ergueu a parede para esconder o quarto vazio da esposa e da própria vergonha.",
        sphere: "As três batidas não são do menino. São de uma criança em 1986, emparedada numa casa construída no mesmo terreno. A esfera costura quartos iguais em anos diferentes.",
        on_true: outcome(
            "Henri foi encontrado vivo no orfanato de Baton Rouge. Castille foi preso por sequestro. Armand parou de beber — por um ano.",
            "MENINO DESAPARECIDO É ENCONTRADO VIVO EM BATON ROUGE",
            "...o agiota Remy Castille foi preso esta manhã...",
            vec![(3, "jailed"), (0, "survived"), (1, "grateful")],
        )
        .flags(vec!["henri_alive"]),
        on_false: outcome("A parede foi derrubada. Não havia nada lá. O menino nunca foi encontrado.", "CASO DUVAL: POLÍCIA ENCERRA INVESTIGAÇÃO", "...a família Duval deixou a cidade...", vec![(1, "left_city"), (2, "left_city")]).flags(vec!["henri_lost"]),
        digit: "9",
        reward: 50,
    }
}

pub fn case03() -> CaseDef {
    CaseDef {
        id: 3,
        title: "O Trem Sem Passageiro",
        city: CityId::Chicago,
        year: 1924,
        intro: "Chicago, 1924. Um homem embarcou no expresso das 23h vindo de Nova Orleans. Há testemunhas, há bilhete, há bagagem. Quando o trem chegou à Estação Terminal de Chicago, ele não estava mais lá. A mala ainda estava no bagageiro. Dentro dela: um relógio parado às 3:17.",
        brief: "Theodore Marsh embarcou, mas não desembarcou. Bilhete, bagagem e testemunhas confirmam o embarque.",
        start: Near(BKind::Station, 0),
        cast: vec![
            person("Theodore", "Marsh", false, 45, Job::Banker, Role::Victim, B(BKind::Hotel, 0), "Contador de um banco de Nova Orleans. Viajava com uma maleta de documentos.").dead(),
            person("Wallace", "Keller", false, 52, Job::Driver, Role::Witness, B(BKind::House, 2), "Condutor do trem. Vinte anos de ferrovia.")
                .temper(40, 60, 60, 50)
                .topics(vec![topic("viagem", "O senhor viu Marsh durante a viagem?", "Picotei o bilhete dele em Memphis. Cabine 7. Depois disso... a porta ficou trancada até Chicago. Quando abrimos, só a mala.").reveals(&[3])]),
            person("Doris", "Brennan", true, 29, Job::Waiter, Role::Witness, B(BKind::Apartment, 3), "Garçonete do vagão-restaurante.")
                .temper(60, 40, 70, 40)
                .topics(vec![topic("jantar", "Marsh jantou no vagão-restaurante?", "Jantou com outro homem. Mais velho. Os dois tinham o mesmo relógio, o mesmo anel, o mesmo jeito de mexer o café. Pareciam o mesmo homem com vinte anos de diferença.").reveals(&[5])]),
            person("Frank", "Moretti", false, 38, Job::Gangster, Role::Suspect, B(BKind::House, 4), "Contrabandista de uísque. Viajava no mesmo trem.")
                .temper(20, 80, 20, 80)
                .flees()
                .topics(vec![
                    topic("marsh", "O senhor conhecia Theodore Marsh?", "O contador? Ele ia depor contra a gente. Livros do banco, dinheiro lavado. Mas quem sumiu com ele não fui eu. Eu só paguei pra ele descer em Cairo.").lie("Nunca vi mais gordo.", 6),
                    topic("cairo", "O que aconteceu em Cairo, Illinois?", "O trem parou quatro minutos. Ele devia descer e pegar o dinheiro. Não desceu. A porta da cabine estava trancada por dentro. Eu mesmo forcei.").req(Req::Topic("marsh")).reveals(&[7]),
                ]),
            person("Mabel", "Marsh", true, 41, Job::Housewife, Role::Contact, B(BKind::Hotel, 0), "A esposa. Veio de Nova Orleans buscar o marido.")
                .temper(60, 50, 60, 40)
                .topics(vec![
                    topic("marido", "Seu marido estava com medo de alguém?", "Do banco. De uns italianos. E de si mesmo, ultimamente. Dizia que via um homem igual a ele no espelho do barbeiro, mais velho.").reveals(&[8]),
                    topic("relogio", "O relógio da mala era dele?", "Era do pai dele. Parou no dia em que o pai morreu. Às 3:17.").req(Req::Clue(1)),
                ]),
            person("Otto", "Gruber", false, 60, Job::Scientist, Role::Suspect, B(BKind::House, 7), "Relojoeiro alemão. Viajava na cabine 8, ao lado.")
                .temper(40, 40, 70, 30)
                .topics(vec![topic("barulho", "Ouviu algo na cabine ao lado?", "Às três e dezessete, um zumbido. Como um rádio entre estações. E a voz de dois homens idênticos discutindo. Depois, silêncio. Eu não abri. Um homem da minha idade aprende a não abrir portas.").reveals(&[9])]),
        ],
        clues: vec![
            clue("Mala de Marsh", ClueKind::Physical, Some(B(BKind::Station, 0)), Look3d::Object, "Roupas, uma navalha, documentos do banco. Tudo dobrado com cuidado demais.", &[]),
            clue("Relógio parado às 3:17", ClueKind::Physical, Some(B(BKind::Station, 0)), Look3d::Glow, "Um relógio de bolso de ouro. Parado às 3:17. Gravado atrás: 'T.M. — 1944'.", &[]).hidden(1),
            clue("Livros do banco", ClueKind::Document, Some(B(BKind::Station, 0)), Look3d::Paper, "Contas que mostram dinheiro de contrabando lavado pelo Banco do Delta.", &[3]).forensic("As páginas de 1923 foram arrancadas e reescritas com outra caligrafia — a do próprio Marsh, mas trêmula, envelhecida."),
            testimony("Cabine 7 trancada", "O condutor picotou o bilhete em Memphis; a cabine ficou trancada até Chicago.", &[]),
            clue("Janela da cabine lacrada", ClueKind::Physical, Some(B(BKind::Station, 0)), Look3d::Object, "A janela da cabine 7 está lacrada com tinta velha. Não abre há anos.", &[]),
            testimony("O homem mais velho", "Doris viu Marsh jantar com um homem idêntico a ele, vinte anos mais velho.", &[]),
            clue("Telegrama para Moretti", ClueKind::Document, Some(HomeOf(3)), Look3d::Paper, "'CONTADOR DESCE EM CAIRO. ENTREGA 500. — F.'", &[3]),
            testimony("A parada em Cairo", "Moretti forçou a porta em Cairo: estava vazia e trancada por dentro.", &[3]),
            testimony("O homem no espelho", "Mabel: Marsh via um homem igual a ele, mais velho, nos espelhos.", &[]),
            testimony("O zumbido das 3:17", "O relojoeiro ouviu um zumbido e dois homens idênticos discutindo.", &[]),
            clue("Eco: cabine 7", ClueKind::Temporal, Some(B(BKind::Station, 0)), Look3d::None, "No eco, dois Theodores. O mais velho segura o mais novo pelos ombros. Ambos somem num clarão vermelho.", &[]),
        ],
        echoes: vec![echo(B(BKind::Station, 0), Ghost::Vanish, &["O balanço do trem.", "Dois homens com o mesmo rosto.", "'Você não pode depor. Eu sei o que acontece depois.'", "Um clarão vermelho. Ninguém."], 10)],
        culprit: 0,
        methods: &["Moretti o jogou do trem em Cairo", "O condutor o escondeu no bagageiro", "Ele próprio — uma versão mais velha dele — o levou através da esfera", "Saltou pela janela"],
        method: 2,
        motives: &["Dívida de jogo", "Impedir o depoimento que, no futuro, mataria sua família", "Fugir da esposa", "Roubar os livros do banco"],
        motive: 1,
        anomalies: &["Nenhuma", "O relógio é gravado com 1944 — e o homem mais velho era o próprio Marsh, vindo de outra linha do tempo", "O trem nunca existiu", "O condutor é o Outro Elias"],
        anomaly: 1,
        threads: &[(0, 0), (0, 3), (0, 4), (3, 1)],
        story: "Theodore Marsh ia depor contra os contrabandistas. Numa outra linha do tempo, ele depôs, e a vingança matou sua família. Um Marsh mais velho encontrou a esfera e voltou para impedir a si mesmo. Os dois desapareceram juntos às 3:17, entre Memphis e Cairo.",
        sphere: "A esfera não transporta apenas Elias. Ela abre portas para quem perdeu tudo. Marsh foi o primeiro que você encontrou. Não será o último.",
        on_true: outcome("Você entrega a verdade ao único que acredita: Mabel. Ela queima os livros do banco. Moretti perde a testemunha e o caso cai. Mas Mabel sorri pela primeira vez.", "MISTÉRIO DO TREM: CASO ARQUIVADO", "...o Banco do Delta nega qualquer irregularidade...", vec![(4, "grateful"), (3, "criminal")]).flags(vec!["marsh_loop"]),
        on_false: outcome("O acusado foi condenado. Mabel nunca soube a verdade.", "PRESO SUSPEITO NO CASO DO TREM", "...o julgamento durou três dias...", vec![]).flags(vec!["marsh_false"]),
        digit: "0",
        reward: 60,
    }
}

pub fn case04() -> CaseDef {
    CaseDef {
        id: 4,
        title: "A Carta Negra",
        city: CityId::Chicago,
        year: 1926,
        intro: "Chicago, 1926. O vereador Harold Whitcombe recebe cartas pretas, escritas em tinta branca. Cada uma chega exatamente 24 horas antes de alguém morrer. Três cartas, três mortes. Esta manhã chegou a quarta. O nome escrito nela é ELIAS VALE.",
        brief: "Cartas pretas preveem mortes com 24h de antecedência. A quarta carta traz o seu nome.",
        start: B(BKind::Office, 0),
        cast: vec![
            person("Harold", "Whitcombe", false, 55, Job::Politician, Role::Contact, B(BKind::Mansion, 0), "Vereador. Recebe as cartas. Quer abafar o caso antes da eleição.")
                .temper(70, 30, 30, 70)
                .topics(vec![
                    topic("cartas", "Desde quando o senhor recebe as cartas?", "Desde março. Três nomes: um estivador, um fiscal da prefeitura e o meu motorista. Todos morreram no dia seguinte. Acidentes, disse a polícia.").reveals(&[1]),
                    topic("ligacao", "O que os três mortos tinham em comum?", "...Eles sabiam do incêndio do cortiço da Rua Canal. Em 1921. Morreram onze pessoas. Eu tinha mandado cortar as saídas de incêndio pra economizar. Os três assinaram o laudo falso.").lie("Nada. Eram pessoas comuns.", 5).reveals(&[6]),
                ]),
            person("Ruth", "Novak", true, 34, Job::Clerk, Role::Suspect, B(BKind::Apartment, 1), "Secretária do vereador. Perdeu a irmã no incêndio de 1921.")
                .temper(30, 70, 60, 20)
                .flees()
                .topics(vec![
                    topic("irma", "A senhora perdeu alguém no incêndio da Rua Canal?", "Minha irmã, Anna. Tinha dezenove anos. As saídas de incêndio estavam soldadas. Eu trabalho para o homem que mandou soldá-las. Todo dia eu sirvo o café dele.").reveals(&[7]),
                    topic("cartas", "A senhora escreve as cartas?", "Escrevo. Mas eu não mato ninguém! Eu só escrevo os nomes que aparecem. Eles aparecem na minha máquina de escrever, de madrugada. Às 3:17.").lie("Eu não sei de carta nenhuma.", 3).reveals(&[8]),
                ]),
            person("Casimir", "Nowak", false, 47, Job::Gangster, Role::Suspect, B(BKind::House, 3), "Irmão de Ruth. Cobrador de um sindicato. Mãos grandes.")
                .temper(15, 90, 25, 60)
                .flees()
                .topics(vec![topic("mortes", "O senhor esteve perto das vítimas?", "Estive em todos os enterros. Pra ter certeza. Minha irmã escreve os nomes. Eu faço os acidentes. Justiça que a prefeitura não fez.").lie("Nunca vi aquela gente.", 9).reveals(&[10])]),
            person("Edith", "Walsh", true, 28, Job::Journalist, Role::Contact, B(BKind::Apartment, 4), "Repórter do The Evening Ledger. Cobre a prefeitura.")
                .temper(20, 80, 80, 30)
                .soul(1)
                .topics(vec![topic("incendio", "O que você sabe do incêndio da Rua Canal?", "Que o laudo foi assinado por três homens que estão mortos. E que o vereador fez fortuna vendendo o terreno. Eu tenho o laudo original na redação.").reveals(&[5])]),
        ],
        clues: vec![
            clue("A quarta carta", ClueKind::Document, Some(B(BKind::Office, 0)), Look3d::Paper, "Papel preto, tinta branca: 'ELIAS VALE — AMANHÃ'. Não há como alguém saber seu nome.", &[1]).forensic("Datilografada. A letra 'e' da máquina tem uma falha característica."),
            testimony("Três mortes", "Um estivador, um fiscal e o motorista morreram 24h após as cartas.", &[]),
            clue("Máquina de escrever", ClueKind::Physical, Some(B(BKind::Office, 0)), Look3d::Object, "A máquina da secretária. A letra 'e' tem uma falha — a mesma das cartas.", &[1]).hidden(1),
            clue("Papel preto e tinta branca", ClueKind::Physical, Some(HomeOf(1)), Look3d::Object, "Um maço de papel preto e um frasco de tinta branca na gaveta de Ruth.", &[1]),
            clue("Freios cortados", ClueKind::Physical, Some(B(BKind::Warehouse, 0)), Look3d::Weapon, "O carro do motorista morto, no depósito da polícia. O cabo do freio foi cortado com alicate.", &[2]).forensic("Alicate grande, de mão canhota. Casimir é canhoto."),
            clue("Laudo original do incêndio", ClueKind::Document, Some(B(BKind::Newspaper, 0)), Look3d::Paper, "O laudo verdadeiro: saídas soldadas por ordem do vereador. Três assinaturas no laudo falso.", &[0]),
            testimony("A confissão do vereador", "Whitcombe mandou soldar as saídas de incêndio; os três mortos assinaram o laudo falso.", &[0]),
            testimony("A irmã de Ruth", "A irmã de Ruth morreu no incêndio de 1921.", &[1]),
            testimony("Os nomes que aparecem", "Ruth diz que os nomes aparecem sozinhos na máquina, às 3:17.", &[1]),
            clue("Luvas com graxa", ClueKind::Physical, Some(HomeOf(2)), Look3d::Object, "Luvas de couro sujas de graxa de freio, na casa de Casimir.", &[2]).hidden(1),
            testimony("Os acidentes", "Casimir confessa que 'fez os acidentes'.", &[2]),
            clue("Eco: a máquina às 3:17", ClueKind::Temporal, Some(B(BKind::Office, 0)), Look3d::None, "No eco, a máquina datilografa sozinha. Ninguém nas teclas.", &[]),
        ],
        echoes: vec![echo(B(BKind::Office, 0), Ghost::Write, &["Escritório vazio.", "As teclas descem sozinhas.", "E - L - I - A - S", "Uma mulher chora na porta."], 11)],
        culprit: 2,
        methods: &["Ruth envenenou as vítimas", "Casimir provocou os 'acidentes' depois das cartas da irmã", "O vereador matou as próprias testemunhas", "Suicídios"],
        method: 1,
        motives: &["Dinheiro do sindicato", "Vingança pelas onze mortes do incêndio da Rua Canal", "Ciúme", "Eleição"],
        motive: 1,
        anomalies: &["Nenhuma", "Os nomes surgem na máquina às 3:17 — e o seu estava lá porque em outra linha do tempo você morre amanhã", "A tinta branca é sangue", "O vereador é imortal"],
        anomaly: 1,
        threads: &[(2, 1), (1, 0), (0, 3), (1, 3)],
        story: "Ruth e Casimir vingavam a irmã morta no incêndio que o vereador provocou. Ruth escrevia os nomes; Casimir fabricava os acidentes. O vereador era o próximo da lista — ele sabia, e por isso chamou você.",
        sphere: "Ruth não mentiu: os nomes aparecem sozinhos. A esfera avisa quem vai morrer quando uma linha do tempo está prestes a fechar. O seu nome apareceu porque você quase ficou aqui.",
        on_true: outcome("Casimir foi preso. Ruth confessou as cartas e entregou o laudo à imprensa. O vereador perdeu a eleição e foi processado pelo incêndio.", "SECRETÁRIA E IRMÃO: AS CARTAS NEGRAS ERAM VINGANÇA", "...o vereador Whitcombe renunciou esta noite...", vec![(2, "jailed"), (1, "journalist"), (0, "jailed")]).flags(vec!["black_letter_true"]),
        on_false: outcome("O acusado foi condenado. As cartas pararam — por um tempo. Em 1946, uma carta preta chega a uma delegacia de Londres.", "CASO DAS CARTAS NEGRAS ENCERRADO", "...o vereador Whitcombe é reeleito...", vec![(2, "criminal")]).flags(vec!["black_letter_false"]),
        digit: "8",
        reward: 70,
    }
}

pub fn case05() -> CaseDef {
    CaseDef {
        id: 5,
        title: "O Hotel Vazio",
        city: CityId::NewOrleans,
        year: 1928,
        intro: "Nova Orleans, 1928. O Hotel Saint-Aubin amanheceu vazio. Comida quente nas mesas. Camas desfeitas. Rádios ligados. Malas abertas. Quarenta e um hóspedes e funcionários sumiram na mesma noite. Só o recepcionista da madrugada ficou — e ele não lembra de nada depois das 3:17.",
        brief: "41 pessoas sumiram do Hotel Saint-Aubin numa noite. Só o recepcionista ficou, sem memória.",
        start: B(BKind::Hotel, 0),
        cast: vec![
            person("Auguste", "Delacroix", false, 58, Job::Merchant, Role::Victim, B(BKind::Mansion, 0), "Dono do hotel. Sumiu com os hóspedes.").dead(),
            person("Felix", "Arceneaux", false, 26, Job::HotelClerk, Role::Witness, B(BKind::Apartment, 1), "Recepcionista da madrugada. Não lembra de nada depois das 3:17.")
                .temper(85, 20, 70, 30)
                .topics(vec![
                    topic("noite", "Do que você se lembra?", "Às três e quinze, um senhor de casaco pediu a chave do porão. Disse que era do governo. Às três e dezessete o rádio parou. Acordei às seis com a comida quente e ninguém.").reveals(&[4]),
                    topic("senhor", "Como era o senhor do governo?", "Tinha o seu rosto. Mais velho. E me chamou pelo nome sem eu dizer.").req(Req::Topic("noite")),
                ]),
            person("Celeste", "Delacroix", true, 30, Job::Aristocrat, Role::Suspect, B(BKind::Mansion, 0), "Filha do dono. Herda o hotel se o pai for declarado morto.")
                .temper(30, 60, 30, 80)
                .topics(vec![
                    topic("heranca", "A senhora herda o hotel?", "Herdo um prédio assombrado e as dívidas do meu pai. Parabéns pra mim.").reveals(&[6]),
                    topic("porao", "O que há no porão do hotel?", "Meu pai comprou o prédio de um laboratório do governo, em 1919. Nunca deixou ninguém descer. Dizia que 'a coisa lá embaixo' pagava as contas.").lie("Vinho. Barris de vinho.", 7),
                ]),
            person("Isaac", "Landry", false, 44, Job::Detective, Role::Contact, B(BKind::House, 9), "Detetive da polícia. Quer que o caso seja 'fuga em massa de caloteiros'.")
                .temper(40, 60, 50, 50)
                .topics(vec![topic("policia", "A polícia vai investigar?", "Investigar o quê? Quarenta e um caloteiros fugiram sem pagar. É o que vai sair no jornal. Ordem de cima. Um homem do governo esteve aqui às seis e levou tudo do porão num caminhão.").reveals(&[8])]),
            person("Pearl", "Mouton", true, 52, Job::Maid, Role::Witness, B(BKind::House, 10), "Camareira. Chegou às seis da manhã e encontrou o hotel vazio.")
                .temper(70, 40, 80, 20)
                .topics(vec![topic("vazio", "O que a senhora viu ao chegar?", "Café ainda fumegando. Um bebê de colo no berço do quarto 12 — o único que ficou. Chorava e ria ao mesmo tempo, olhando pro teto vermelho.").reveals(&[5])]),
        ],
        clues: vec![
            clue("Comida quente", ClueKind::Physical, Some(B(BKind::Hotel, 0)), Look3d::Object, "Pratos servidos, ainda mornos. Garfos no meio do caminho entre prato e boca.", &[]),
            clue("Rádio parado", ClueKind::Physical, Some(B(BKind::Hotel, 0)), Look3d::Glow, "O rádio do saguão ficou sintonizado entre estações. O ponteiro derreteu às 3:17.", &[]).hidden(1),
            clue("Livro de hóspedes", ClueKind::Document, Some(B(BKind::Hotel, 0)), Look3d::Paper, "41 nomes. O último, escrito às 3:15: 'E. Vale — governo'.", &[]),
            clue("Chave do porão", ClueKind::Physical, Some(B(BKind::Hotel, 0)), Look3d::Object, "Uma chave de ferro que não abre nenhuma porta do hotel — o porão foi murado depois.", &[2]),
            testimony("O homem do governo", "Um senhor com o rosto de Elias pediu a chave do porão às 3:15.", &[]),
            testimony("O bebê do quarto 12", "Um bebê ficou para trás, olhando o teto vermelho.", &[]),
            testimony("A herança", "Celeste herda o hotel e as dívidas.", &[2]),
            clue("Contrato de 1919", ClueKind::Document, Some(HomeOf(2)), Look3d::Paper, "Delacroix comprou o prédio do 'Projeto Vermilion', do governo. Cláusula: 'o comprador não abrirá o subsolo'.", &[2]).hidden(1),
            testimony("O caminhão do governo", "Às seis, um homem do governo levou tudo do porão num caminhão.", &[]),
            clue("Recibo do caminhão", ClueKind::Document, Some(B(BKind::Police, 0)), Look3d::Paper, "Transporte de 'um objeto esférico' para o Laboratório Delta. Assinado: Projeto Vermilion.", &[]).hidden(2).network(Job::Police),
            clue("Eco: o porão", ClueKind::Temporal, Some(B(BKind::Hotel, 0)), Look3d::None, "No eco, quarenta e uma pessoas descem a escada em fila, sonâmbulas. Lá embaixo, uma luz roxa.", &[]),
        ],
        echoes: vec![echo(B(BKind::Hotel, 0), Ghost::Walk, &["Passos na escada.", "Gente de pijama, olhos abertos, sem ver.", "Uma luz roxa pulsa no porão.", "Uma voz: 'Ainda não é a sua vez, Felix.'"], 10)],
        culprit: 2,
        methods: &["Celeste drogou os hóspedes e os levou num caminhão", "A esfera do porão os levou — e alguém abriu o porão de propósito", "Fuga em massa de caloteiros", "Envenenamento coletivo"],
        method: 1,
        motives: &["Herdar o hotel", "Vender o 'objeto' do porão ao governo pagando as dívidas do pai", "Vingança contra o pai", "Seguro de vida"],
        motive: 1,
        anomalies: &["Nenhuma", "A esfera esteve no porão do hotel antes de chegar ao Laboratório Delta — e o homem do governo era você, mais velho", "O hotel nunca existiu", "Os hóspedes eram fantasmas"],
        anomaly: 1,
        threads: &[(2, 0), (1, 2), (3, 2), (4, 1)],
        story: "Celeste, afogada nas dívidas do pai, vendeu a esfera do porão ao Projeto Vermilion. Para o comprador buscá-la, alguém precisava abrir o porão. Às 3:17 a esfera 'acordou' e levou quem estava no hotel. O comprador que assinou o livro como 'E. Vale' levou a esfera para o Laboratório Delta.",
        sphere: "Você acaba de ver o primeiro elo: a esfera passou por aqui em 1928, a caminho do lugar onde você a encontraria em 2025. Alguém com o seu nome a levou.",
        on_true: outcome("Celeste foi presa por fraude. O Projeto Vermilion negou tudo. Os quarenta e um hóspedes continuam desaparecidos — em alguma outra década.", "HERDEIRA DO SAINT-AUBIN É PRESA", "...o hotel será demolido...", vec![(2, "jailed")]).close(vec![BKind::Hotel]).flags(vec!["hotel_true"]),
        on_false: outcome("O caso foi encerrado como fuga em massa. O hotel reabriu. Hóspedes relatam passos no porão murado.", "HOTEL SAINT-AUBIN REABRE", "...hóspedes relatam ruídos no subsolo...", vec![]).flags(vec!["hotel_false"]),
        digit: "4",
        reward: 80,
    }
}
