//! ERA V — 1980–1999 — Nova York e Los Angeles.

use super::defs::*;
use crate::city::gen::CityId;
use crate::city::map::BKind;
use crate::sim::people::Job;
use Place::*;

pub fn cases() -> Vec<CaseDef> {
    vec![case21(), case22(), case23(), case24(), case25()]
}

pub fn case21() -> CaseDef {
    CaseDef {
        id: 21,
        title: "O Menino do Ponto de Ônibus",
        city: CityId::NewYork,
        year: 1981,
        intro: "Nova York, 1981. Danny Ashford, seis anos, implorou durante meses para ir sozinho até o ponto do ônibus escolar. Eram só dois quarteirões no SoHo. Na manhã em que a mãe finalmente deixou, ele saiu com um dólar no bolso para comprar um refrigerante — e nunca chegou à esquina. Agora o rosto dele está em caixas de leite no país inteiro. E, no caderno da escola, Danny desenhou o ponto de ônibus, um homem de casaco e uma bola escura no céu.",
        brief: "Danny Ashford, 6 anos, sumiu entre a porta de casa e o ponto de ônibus, às 8h05. Um faz-tudo, um andarilho e uma vizinha estavam na rua.",
        start: HomeOf(0),
        cast: vec![
            person("Danny", "Ashford", false, 6, Job::Child, Role::Victim, B(BKind::Apartment, 3), "Seis anos. Boné de piloto, tênis azul, um dólar no bolso. Primeira vez sozinho na rua.").dead(),
            person("Judith", "Ashford", true, 38, Job::Photographer, Role::Witness, B(BKind::Apartment, 3), "A mãe. Fotógrafa. A foto de Danny nas caixas de leite foi ela quem tirou.")
                .temper(75, 55, 80, 20)
                .topics(vec![
                    topic("manha", "Conte sobre a manhã em que Danny saiu.", "Eu olhei da janela até ele passar a banca de jornal. Aí o telefone tocou. Eram oito e cinco. Quando voltei à janela, a calçada estava vazia. Eu desviei os olhos por quarenta segundos, Elias. Quarenta.").reveals(&[6]),
                    topic("faz_tudo", "Alguém de fora frequentava o apartamento?", "Só o Ray, o faz-tudo do prédio da Rua Prince. Consertou nosso radiador em março. O Danny o adorava, queria ver 'o porão das máquinas'. Naquela mesma semana sumiu o colar da minha mãe. Achei que eu tinha perdido.").reveals(&[11]),
                    topic("desenho", "A senhora viu este desenho no caderno dele?", "Ele desenhava isso há semanas. A bola escura, o homem de casaco, e esses números: 3:17. Eu achei que era da televisão. Agora não consigo parar de olhar para ele.").req(Req::Clue(14)),
                ]),
            person("Ray", "Kowalski", false, 34, Job::Clerk, Role::Suspect, B(BKind::Apartment, 7), "Zelador e faz-tudo de três prédios da Rua Prince. Quieto, prestativo, sempre com a caixa de ferramentas.")
                .temper(65, 35, 20, 60)
                .flees()
                .topics(vec![
                    topic("ashford", "O senhor conhecia a família Ashford?", "Consertei o radiador deles. O menino ficava atrás de mim o dia inteiro, fazendo pergunta. Criança, sabe como é. Coitada da mãe."),
                    topic("manha", "Onde o senhor estava às oito da manhã?", "Tá bom. Eu estava na bodega da esquina. Comprei dois refrigerantes. O Danny passou, disse oi, eu dei uma lata pra ele e ele seguiu pro ônibus. Foi só isso. Eu juro que foi só isso.")
                        .lie("Dormindo. Eu só pego no serviço às dez. Pode perguntar a qualquer um.", 9),
                    topic("porao", "Por que há cimento fresco no piso da caldeira?", "...Ele viu o colar. Na minha caixa de ferramentas, no porão. Disse: 'Esse é o colar da vovó, vou contar pra mamãe.' Ele começou a gritar. Eu só queria que ele parasse de gritar.")
                        .lie("O piso rachou. O síndico mandou consertar antes do inverno.", 2)
                        .req(Req::Clue(12)),
                ]),
            person("Luther", "Graves", false, 52, Job::Drifter, Role::Suspect, B(BKind::Hotel, 2), "Andarilho do Bowery. Tira fotos Polaroid de desconhecidos no parque. A polícia adora o nome dele.")
                .temper(55, 40, 45, 30)
                .topics(vec![
                    topic("manha", "Onde o senhor estava na manhã do sumiço?", "Na cela da Delegacia do Sexto Distrito, curando um porre. Me pegaram às seis e quarenta, dormindo num degrau. Me soltaram depois do meio-dia. A melhor noite de sono do ano.")
                        .lie("No parque. Sozinho. Como sempre. Ninguém me viu, ninguém nunca me vê.", 5)
                        .reveals(&[10]),
                    topic("fotos", "E estas Polaroids de crianças?", "Eu fotografo o que é bonito e vai embora. Crianças, pombos, velhos jogando xadrez. Não é crime ter olhos, moço. Mas nessa cidade um homem como eu é culpado até provar que é invisível.").req(Req::Clue(4)),
                ]),
            person("Edna", "Hollis", true, 71, Job::Retired, Role::Witness, B(BKind::Apartment, 5), "Vizinha do segundo andar. Viúva, passa as manhãs na janela com o café e o rádio.")
                .temper(60, 45, 75, 15)
                .topics(vec![
                    topic("viu", "A senhora viu Danny naquela manhã?", "Vi. Parado na bodega, conversando com o zelador, o Kowalski. O homem tinha duas latas de refrigerante. Os dois foram andando pro lado da Rua Prince. Achei que era favor da mãe. Eu não disse nada à polícia porque ninguém me perguntou.").reveals(&[7]),
                    topic("casaco", "Havia mais alguém na rua?", "Um homem alto, de casaco comprido, parado no ponto de ônibus. Mais velho. Olhou o relógio e disse em voz alta: 'Ainda não.' Quando eu pisquei, não estava mais lá. Ele tinha o seu rosto, meu jovem. Mais cansado.").req(Req::Topic("viu")).reveals(&[8]),
                ]),
            person("Nora", "Castellano", true, 31, Job::Journalist, Role::Contact, B(BKind::Apartment, 9), "Repórter do noticiário noturno do Canal 7. Foi ela quem levou a foto de Danny para as caixas de leite.")
                .temper(25, 80, 80, 30)
                .soul(1)
                .topics(vec![
                    topic("bodega", "Você falou com o dono da bodega?", "Falei. O senhor Ortiz lembra de tudo: às oito o zelador comprou duas latas de Coca e disse 'uma é pro meu ajudante'. Ninguém da polícia voltou lá depois do primeiro dia. Estão ocupados demais com o andarilho.").reveals(&[9]),
                    topic("caixas", "Por que as caixas de leite?", "Porque a polícia desistiu em três semanas, e o leite entra em todas as cozinhas do país. Se ele estiver vivo, alguém vai olhar para ele no café da manhã."),
                    topic("voce", "Você já me viu antes, Nora?", "Não. Mas eu sonho com redações que ainda não existem. Uma máquina de escrever, cartas pretas... e você, entregando um caso para mim. É ridículo, eu sei. Me dá o próximo fio, Elias.").req(Req::Topic("bodega")),
                ]),
            person("Frank", "Mulroney", false, 46, Job::Detective, Role::Contact, B(BKind::House, 4), "Sargento-detetive do Sexto Distrito. Cansado, honesto, pressionado para prender alguém antes do Natal.")
                .temper(35, 65, 60, 35)
                .topics(vec![
                    topic("graves", "Por que a polícia persegue Luther Graves?", "Porque ele tem ficha e tira foto de criança. Os jornais querem um monstro. Mas cá entre nós: às oito da manhã o Graves estava na minha própria cela. Eu não posso dizer isso em voz alta, os chefes me comem vivo.").reveals(&[10]),
                    topic("porao", "Alguém revistou os porões da Rua Prince?", "Revistamos os prédios. Os porões... o zelador disse que estavam trancados por causa de ratos, e a gente acreditou. Droga. Me arranja um motivo e eu desço lá com uma marreta.").req(Req::Clue(3)),
                ]),
        ],
        clues: vec![
            clue("Cartaz de 'Desaparecido'", ClueKind::Document, Some(HomeOf(0)), Look3d::Photo, "DESAPARECIDO: Danny Ashford, 6 anos. Boné de piloto, tênis azul, mochila vermelha. Visto pela última vez às 8h05.", &[]),
            clue("Caixa de leite com a foto", ClueKind::Document, Some(Near(BKind::Market, 0)), Look3d::Photo, "O rosto de Danny impresso numa caixa de leite: 'VOCÊ ME VIU?'. Há milhares delas em todo o país.", &[])
                .forensic("O código do lote diz que a rotativa travou sozinha às 3:17 da madrugada, no meio da tiragem. Os técnicos não souberam explicar."),
            clue("Tênis azul infantil", ClueKind::Physical, Some(B(BKind::Warehouse, 1)), Look3d::Object, "Um tênis azul número 28, atrás da caldeira do porão da Rua Prince. No calcanhar, a caneta: D.A.", &[2])
                .forensic("A sola tem pó de cimento seco e um resto de xarope de refrigerante. O cadarço foi arrancado com força.")
                .hidden(2),
            clue("Livro de serviços do zelador", ClueKind::Document, Some(HomeOf(2)), Look3d::Paper, "Caderno de Ray Kowalski: 'Março — Ashford, apto 3 — radiador.' Na margem: 'o menino quer ver o porão'. Outra letra, mais funda: 'colar — Rua 47'.", &[2]),
            clue("Polaroids do andarilho", ClueKind::Physical, Some(HomeOf(3)), Look3d::Photo, "Dezenas de Polaroids de crianças brincando na Washington Square, guardadas numa lata de biscoito.", &[3])
                .forensic("Todas datadas no verso, a mais recente de duas semanas antes. Nenhuma mostra Danny. Nenhuma foi tirada no SoHo.")
                .herring(),
            clue("Registro da cela de detenção", ClueKind::Document, Some(B(BKind::Police, 0)), Look3d::Paper, "Livro de custódia do Sexto Distrito: 'Graves, Luther — embriaguez — entrada 6h40, saída 12h15.'", &[])
                .network(Job::Police),
            testimony("Quarenta segundos", "Judith viu Danny passar a banca às 8h05. O telefone tocou. Quando voltou à janela, a calçada estava vazia.", &[]),
            testimony("O zelador na esquina", "Edna viu Danny na bodega com Ray Kowalski, que trazia duas latas. Os dois seguiram para a Rua Prince.", &[2]),
            testimony("O homem que disse 'ainda não'", "Edna viu um homem alto de casaco no ponto de ônibus olhar o relógio, dizer 'ainda não' e sumir. Tinha o rosto de Elias, mais velho.", &[]),
            testimony("Dois refrigerantes", "O dono da bodega: às 8h, o zelador comprou duas Cocas, 'uma pro meu ajudante'.", &[2]),
            testimony("O andarilho estava preso", "Luther Graves passou a manhã inteira numa cela do Sexto Distrito.", &[]),
            testimony("O colar sumido", "Ray consertou o radiador dos Ashford em março. Danny o adorava. Na mesma semana, sumiu um colar da família.", &[2]),
            clue("Cimento fresco na caldeira", ClueKind::Physical, Some(B(BKind::Warehouse, 1)), Look3d::Object, "Um retângulo de cimento novo no piso da sala da caldeira, do tamanho de um baú de viagem.", &[2])
                .forensic("Misturado à massa, um fio de lã vermelha — da mochila. O cimento foi despejado de madrugada: ainda há marcas de uma lanterna apoiada no chão.")
                .hidden(1),
            clue("Eco: o porão da caldeira", ClueKind::Temporal, Some(B(BKind::Warehouse, 1)), Look3d::None, "No eco, Ray desce a escada com um saco nas costas. O relógio da caldeira marca 3:17 da madrugada. Ele mistura cimento chorando.", &[2]),
            clue("Desenho no caderno da escola", ClueKind::Document, Some(HomeOf(0)), Look3d::Glow, "Giz de cera: o ponto de ônibus, um homem de casaco e uma bola escura com pontos vermelhos no céu. Embaixo, com letra de criança: 3:17.", &[])
                .hidden(1),
            clue("Eco: a esquina da bodega", ClueKind::Temporal, Some(Near(BKind::General, 0)), Look3d::None, "No eco, Danny para na bodega. Ray estende uma lata. No ponto de ônibus, um homem de casaco olha o relógio e desvia o rosto.", &[2]),
        ],
        echoes: vec![
            echo(B(BKind::Warehouse, 1), Ghost::Drag, &["Uma escada de ferro range.", "Um saco pesado bate em cada degrau.", "O relógio da caldeira: 3:17.", "Uma pá raspa o cimento. Alguém soluça."], 13),
            echo(Near(BKind::General, 0), Ghost::Walk, &["Manhã de maio. Um boné de piloto.", "'Oi, Ray!' Uma lata gelada.", "No ponto de ônibus, um homem de casaco: 'Ainda não.'", "Dois pares de passos rumo à Rua Prince."], 15),
        ],
        culprit: 2,
        methods: &[
            "O andarilho o levou do parque para o Bowery",
            "Ray o atraiu com um refrigerante até o porão da caldeira e o enterrou sob cimento fresco",
            "Um carro parou no ponto de ônibus e o levou",
            "A mãe inventou o trajeto para esconder um acidente em casa",
        ],
        method: 1,
        motives: &[
            "Resgate nunca pedido",
            "Vingança contra a mãe",
            "Danny reconheceu o colar roubado da avó no porão e ameaçou contar",
            "Um ritual inspirado no desenho do menino",
        ],
        motive: 2,
        anomalies: &[
            "Nenhuma — é apenas um crime",
            "Danny desenhou o ponto, a esfera e 3:17 semanas antes — e um homem com o rosto de Elias, mais velho, esperava no ponto dizendo 'ainda não'",
            "As caixas de leite foram impressas antes do sumiço",
            "O menino nunca existiu",
        ],
        anomaly: 1,
        threads: &[(2, 0), (0, 1), (2, 1), (4, 2), (5, 4), (6, 3)],
        story: "Ray Kowalski roubava pequenas joias dos apartamentos onde fazia consertos e as guardava no porão da Rua Prince. Na primeira manhã em que Danny andou sozinho, Ray o chamou com um refrigerante e a promessa de ver 'o porão das máquinas'. Lá embaixo, o menino reconheceu o colar da avó e disse que ia contar. Ray o calou para sempre e, às 3:17 da madrugada seguinte, o cobriu de cimento.",
        sphere: "O homem de casaco no ponto de ônibus era você — o Outro Elias, mais velho. Ele sabia o que ia acontecer e disse 'ainda não'. Não para Danny. Para você. Ele está esperando que você chegue a algum lugar antes de agir.",
        on_true: outcome(
            "A marreta de Mulroney abriu o piso da caldeira. Ray Kowalski confessou diante do cimento quebrado. Judith pôde enterrar o filho. As caixas de leite saíram das prateleiras uma a uma.",
            "ZELADOR DO SOHO CONFESSA A MORTE DO MENINO DANNY ASHFORD",
            "...o corpo foi encontrado sob o porão de um prédio da Rua Prince. O Canal 7 foi o primeiro a noticiar...",
            vec![(2, "jailed"), (1, "grateful"), (3, "survived"), (5, "journalist"), (6, "police")],
        )
        .flags(vec!["danny_found"]),
        on_false: outcome(
            "Luther Graves foi preso sob aplausos. O caso esfriou. Ray Kowalski se mudou para Nova Jersey. A foto de Danny continuou nas caixas de leite por vinte anos.",
            "ANDARILHO DO BOWERY É ACUSADO NO CASO DO MENINO DO SOHO",
            "...a família Ashford ainda espera por respostas...",
            vec![(3, "jailed"), (2, "criminal"), (1, "hates_elias")],
        )
        .flags(vec!["danny_lost"]),
        digit: "9",
        reward: 95,
    }
}

pub fn case22() -> CaseDef {
    CaseDef {
        id: 22,
        title: "Os Moradores do Túnel",
        city: CityId::NewYork,
        year: 1986,
        intro: "Nova York, 1986. Debaixo de Manhattan há quilômetros de túneis abandonados, e neles vivem centenas de pessoas que a cidade prefere esquecer. Walt Brzezinski, inspetor de via do metrô, desceu na madrugada para checar uma pane na Linha 1. Foi encontrado com a cabeça aberta por um grampo de trilho, deitado sob um desenho feito a fuligem na parede: uma bola escura com pontos vermelhos. Os jornais já chamam os moradores do túnel de 'gente-toupeira'. E já escolheram um culpado.",
        brief: "O inspetor Walt Brzezinski foi morto num túnel abandonado durante uma pane. Ao lado do corpo, o desenho da esfera. A polícia culpa o 'Profeta' que vive lá embaixo.",
        start: B(BKind::Abandoned, 2),
        cast: vec![
            person("Walt", "Brzezinski", false, 49, Job::FactoryWorker, Role::Victim, B(BKind::House, 6), "Inspetor de via do metrô há vinte e dois anos. Conhecia cada curva dos túneis e cada pessoa que dormia neles.").dead(),
            person("Harold", "Pruitt", false, 53, Job::Clerk, Role::Suspect, B(BKind::House, 2), "Supervisor de manutenção da Linha 1. Carro novo, relógio novo, salário velho.")
                .temper(55, 45, 15, 85)
                .flees()
                .topics(vec![
                    topic("noite", "Onde o senhor estava durante a pane?", "Tá. Eu desci. Fui conferir o serviço, é o meu trabalho. Encontrei o Walt perto da curva e ele estava vivo, vivo e reclamando, como sempre. Voltei antes das três e meia.")
                        .lie("Em casa, no Queens, dormindo. Não era meu turno. Minha mulher confirma.", 5),
                    topic("cobre", "Cabos de cobre estão sumindo da Linha 1.", "Todo mundo tira um pouco! Aquele cobre ia apodrecer no escuro. Oitocentos quilos, e daí? Ninguém morreu por causa de cobre...")
                        .lie("Cobre? São os moradores do túnel. Eles arrancam tudo que brilha pra vender.", 4)
                        .req(Req::Clue(3)),
                    topic("desenho", "E o desenho na parede, ao lado do corpo?", "Aquilo é coisa do maluco, do Profeta. Ele pinta essa bola em todo canto. Pergunte a ele o que aconteceu."),
                ]),
            person("Amos", "Kincaid", false, 61, Job::Drifter, Role::Suspect, B(BKind::Hotel, 3), "Chamado de 'Profeta'. Vive nos túneis há quinze anos. Desenha a mesma esfera em todas as paredes.")
                .temper(30, 60, 70, 5)
                .topics(vec![
                    topic("esfera", "Por que você desenha essa bola nas paredes?", "Porque eu vi. Em 1971 eu trabalhava de faxineiro num prédio do governo, lá em cima. Tinha um porão fundo, um laboratório. Às 3:17 uma luz roxa encheu o corredor e eu vi a bola respirar. Desde então eu desço. Aqui embaixo ela fala mais alto.").reveals(&[8]),
                    topic("walt", "Você conhecia Walt?", "Walt trazia café e pilhas de lanterna pra gente. Era o único lá de cima que nos chamava pelo nome. Eu não mataria o único homem que me chamava de Amos."),
                    topic("mao", "Quem guiou a sua mão no desenho mais novo?", "Um homem de casaco. Mais velho que você, com os seus olhos. Segurou meu pulso e disse: 'Mais fundo, Amos. Ele precisa achar o mapa.' Depois foi embora pelo trilho, sem lanterna.").req(Req::Topic("esfera")),
                ]),
            person("Dante", "Reyes", false, 24, Job::Gangster, Role::Suspect, B(BKind::Apartment, 4), "Vende crack na entrada do túnel da Rua 96. Conhece todos os buracos da rede.")
                .temper(35, 70, 30, 75)
                .topics(vec![
                    topic("tunel", "Você esteve no túnel naquela noite?", "Tá, eu uso o túnel. Mas naquela noite, não. E vou te dizer mais: o chefe de capacete branco pagava a gente pra ficar longe da Linha 1 nas noites de pane. Cem dólares pra sumir. Nunca perguntei por quê.")
                        .lie("Nunca pisei lá embaixo. Tenho claustrofobia, cara.", 2)
                        .reveals(&[10]),
                    topic("noite", "Onde você estava às três da manhã?", "Carregando meu primo pro posto de saúde da rua 100. Overdose. A enfermeira japonesa viu tudo. Pergunta pra ela."),
                ]),
            person("Loretta", "Brzezinski", true, 47, Job::Housewife, Role::Witness, B(BKind::House, 6), "Viúva de Walt. Guarda o uniforme dele dobrado na cadeira da cozinha.")
                .temper(70, 50, 80, 20)
                .topics(vec![
                    topic("walt", "Walt estava preocupado com alguma coisa?", "Com o trabalho. Passava as noites escrevendo uma carta pro Inspetor-Geral do metrô. Dizia que alguém lá dentro estava vendendo a linha em pedaços. Ia entregar na segunda-feira.").reveals(&[6]),
                    topic("pruitt", "O que a senhora sabe de Harold Pruitt?", "O chefe dele. Veio ao velório de terno novo e me perguntou se o Walt tinha deixado 'papéis'. Achei estranho. Um chefe não pergunta de papéis no velório.").req(Req::Topic("walt")),
                ]),
            person("Tomás", "Vega", false, 16, Job::Student, Role::Witness, B(BKind::Hotel, 5), "Menino que fugiu de casa e vive nos túneis. Anda sem lanterna e vê tudo no escuro.")
                .temper(75, 45, 65, 30)
                .topics(vec![
                    topic("viu", "Você viu Walt naquela noite?", "Vi. Ele entrou com um homem de colete laranja e capacete branco. Capacete branco é de chefe. Os dois estavam brigando. Depois o de capacete voltou sozinho, arrastando os pés na brita.").reveals(&[7]),
                    topic("medo", "Por que você não contou à polícia?", "Polícia lá embaixo só vem pra botar fogo nos colchões. Se eu falo, sou eu que vou pro reformatório."),
                ]),
            person("Irene", "Sato", true, 33, Job::Nurse, Role::Contact, B(BKind::Apartment, 8), "Enfermeira de um posto de saúde móvel que atende os moradores dos túneis.")
                .temper(30, 75, 85, 15)
                .topics(vec![
                    topic("dante", "Dante Reyes esteve no posto?", "Esteve. Chegou às três em ponto carregando um rapaz em overdose. Ficou até as cinco segurando a mão dele. Gente pior do que o Dante mora lá em cima.").reveals(&[9]),
                    topic("tunel", "Como é a vida lá embaixo?", "Frio, rato, fogueira. E uma ordem que ninguém quebra: não se mexe com os homens do metrô, porque são eles que decidem quando a luz acaba. Ninguém de lá mataria o Walt."),
                ]),
        ],
        clues: vec![
            clue("Grampo de trilho ensanguentado", ClueKind::Physical, Some(B(BKind::Abandoned, 2)), Look3d::Weapon, "Um grampo de trilho de ferro, com sangue e cabelo na cabeça.", &[1])
                .forensic("O grampo é novo, com a pintura antiferrugem do lote de 1986 — igual aos do almoxarifado da manutenção. Os trilhos daquele túnel não são trocados desde 1950."),
            clue("Desenho da esfera na parede", ClueKind::Physical, Some(B(BKind::Abandoned, 2)), Look3d::Glow, "Uma bola escura desenhada a fuligem, com pontos vermelhos de tinta de sinalização. Embaixo, riscado: 3:17.", &[2]),
            clue("Frascos de crack", ClueKind::Physical, Some(B(BKind::Abandoned, 2)), Look3d::Object, "Frascos de tampa vermelha espalhados perto do corpo. A marca de Dante Reyes.", &[3])
                .forensic("Poeira grossa dentro dos frascos e teias nas tampas. Estão ali há semanas, não há horas.")
                .herring(),
            clue("Rascunho da carta ao Inspetor-Geral", ClueKind::Document, Some(HomeOf(0)), Look3d::Paper, "'Cabos de cobre retirados da Linha 1 com ordens de serviço falsas. Todas assinadas por H. Pruitt. Tenho as cópias.'", &[1])
                .hidden(1),
            clue("Recibos do ferro-velho", ClueKind::Document, Some(B(BKind::Warehouse, 0)), Look3d::Paper, "Ferro-velho do Bronx: 'Cobre, 800 kg — $2.300 — pago a H.P.' Seis recibos em seis meses, todos em noites de pane.", &[1])
                .network(Job::Merchant),
            clue("Escala de serviço da noite", ClueKind::Document, Some(B(BKind::Station, 0)), Look3d::Paper, "A escala da pane: energia cortada das 2h40 às 4h. Entrada assinada por H. Pruitt às 2h55, embora não fosse seu turno.", &[1])
                .forensic("A hora de saída foi rasurada. Por baixo da rasura, lê-se 3:31. A caneta usada para corrigir é outra."),
            testimony("A carta que Walt não entregou", "Loretta: Walt escrevia ao Inspetor-Geral denunciando alguém que 'vendia a linha em pedaços'.", &[1]),
            testimony("O capacete branco", "Tomás viu Walt entrar no túnel com um homem de capacete branco. O de capacete voltou sozinho.", &[1]),
            testimony("A luz de 1971", "O Profeta viu a esfera 'respirar' às 3:17, em 1971, no porão de um laboratório do governo.", &[]),
            testimony("Dante no posto de saúde", "Irene: Dante passou das três às cinco no posto, com um primo em overdose.", &[]),
            testimony("Cem dólares pra sumir", "Dante: o chefe de capacete branco pagava os traficantes para ficarem longe da Linha 1 nas noites de pane.", &[1]),
            clue("Capacete branco com fuligem", ClueKind::Physical, Some(HomeOf(1)), Look3d::Object, "Um capacete branco de supervisor, esfregado às pressas, guardado no porta-malas de um Buick novo.", &[1])
                .forensic("Fuligem idêntica à do desenho da parede na aba. Dentro do fecho, sangue seco do tipo de Walt.")
                .hidden(1),
            clue("Eco: a curva do túnel", ClueKind::Temporal, Some(B(BKind::Abandoned, 2)), Look3d::None, "No eco, Pruitt e Walt discutem sob a lanterna. Um grampo sobe. Pruitt arrasta o corpo até a parede do desenho.", &[1]),
            clue("Eco: o altar do Profeta", ClueKind::Temporal, Some(B(BKind::Abandoned, 3)), Look3d::None, "No eco, Amos desenha a esfera. Uma mão de casaco segura o pulso dele e aponta para um buraco mais fundo.", &[]),
            clue("Mapa de acesso de 1971", ClueKind::Document, Some(B(BKind::Abandoned, 3)), Look3d::Glow, "Uma planta amarelada dos túneis, escondida atrás de um tijolo solto. Um corredor marcado a lápis vermelho termina num carimbo: 'LABORATÓRIO DELTA — ACESSO DE SERVIÇO — 1971'.", &[])
                .forensic("A anotação na margem, 'descer às 3:17', está na sua letra.")
                .hidden(3),
        ],
        echoes: vec![
            echo(B(BKind::Abandoned, 2), Ghost::Struggle, &["Escuridão total. Uma lanterna acende.", "'Segunda-feira eu entrego tudo, Harold.'", "Um grampo de ferro sobe.", "Um corpo arrastado até a bola desenhada na parede."], 12),
            echo(B(BKind::Abandoned, 3), Ghost::Write, &["Fuligem na ponta dos dedos.", "A bola escura cresce na parede.", "Uma mão de casaco segura o pulso: 'Mais fundo, Amos.'", "Um tijolo solto. Um papel dobrado."], 13),
        ],
        culprit: 1,
        methods: &[
            "O Profeta o matou num surto diante do próprio desenho",
            "Pruitt o golpeou com um grampo novo durante a pane e arrastou o corpo até o desenho do Profeta",
            "Traficantes o mataram por ter visto a venda de crack",
            "Foi atingido por um trem de manutenção",
        ],
        method: 1,
        motives: &[
            "Roubo das pilhas e do café que Walt levava",
            "Briga por território de venda",
            "Walt ia denunciar ao Inspetor-Geral o roubo de cobre de Pruitt",
            "Ritual em nome da esfera",
        ],
        motive: 2,
        anomalies: &[
            "Nenhuma — é apenas um crime",
            "O Profeta viu a esfera no Laboratório Delta em 1971 — e o mapa escondido, com anotação na letra de Elias, foi deixado para ele pelo Outro Elias",
            "O grampo de trilho é de 2025",
            "Walt continua trabalhando nos túneis depois de morto",
        ],
        anomaly: 1,
        threads: &[(1, 0), (0, 4), (1, 3), (5, 1), (6, 3), (2, 0)],
        story: "Harold Pruitt vendia o cobre da Linha 1 a um ferro-velho do Bronx, sempre nas noites de pane, e pagava os traficantes para ficarem longe. Walt descobriu e escreveu ao Inspetor-Geral. Na madrugada da pane, Pruitt desceu com um grampo novo do almoxarifado, matou Walt e arrastou o corpo até o desenho do Profeta, para que a cidade culpasse a 'gente-toupeira'.",
        sphere: "Amos Kincaid viu a esfera em 1971, no prédio onde ela seria estudada. O mapa atrás do tijolo leva ao Laboratório Delta — e a letra na margem é a sua. Alguém está deixando um caminho para você seguir. Alguém que já o percorreu.",
        on_true: outcome(
            "Harold Pruitt foi preso com o capacete ainda no porta-malas. O Inspetor-Geral recebeu, enfim, a carta de Walt. A prefeitura suspendeu a limpeza dos túneis por um inverno. Amos desenhou uma última esfera e desapareceu mais fundo.",
            "SUPERVISOR DO METRÔ É PRESO PELA MORTE DE INSPETOR NO TÚNEL",
            "...a investigação revelou um esquema de roubo de cobre na Linha 1...",
            vec![(1, "jailed"), (4, "grateful"), (2, "left_city"), (3, "survived"), (5, "grateful")],
        )
        .flags(vec!["tunnel_true"]),
        on_false: outcome(
            "A polícia desceu com lança-chamas e expulsou os moradores do túnel. Amos Kincaid morreu numa cela antes do julgamento. Harold Pruitt foi promovido.",
            "'PROFETA' DOS TÚNEIS É ACUSADO DE MATAR INSPETOR DO METRÔ",
            "...operação policial remove duzentas pessoas dos túneis de Manhattan...",
            vec![(2, "dead"), (1, "criminal"), (4, "hates_elias")],
        )
        .close(vec![BKind::Abandoned])
        .flags(vec!["tunnel_false"]),
        digit: "0",
        reward: 100,
    }
}

pub fn case23() -> CaseDef {
    CaseDef {
        id: 23,
        title: "A Galeria das Molduras Vazias",
        city: CityId::NewYork,
        year: 1989,
        intro: "Nova York, 1989. Na madrugada de São Patrício, dois homens fardados de policiais tocaram a campainha do Museu Whitfield dizendo que atendiam a um chamado de distúrbio. Os guardas abriram. Foram amarrados com fita adesiva no porão. Oitenta minutos depois, às 3:17, os 'policiais' saíram com treze obras. Nas paredes ficaram as molduras vazias. Uma das telas roubadas, 'O Viajante', de 1658, mostra um homem de casaco segurando uma esfera escura. O rosto é o seu.",
        brief: "Falsos policiais renderam dois guardas e levaram treze obras do Museu Whitfield. As molduras vazias continuam na parede. Entre as telas roubadas, um retrato de Elias pintado em 1658.",
        start: B(BKind::Mansion, 0),
        cast: vec![
            person("Margaret", "Whitfield", true, 64, Job::Aristocrat, Role::Victim, B(BKind::Mansion, 2), "Diretora e herdeira do museu. Jurou nunca mudar um quadro de lugar, como mandava o testamento da avó.")
                .temper(45, 60, 70, 40)
                .topics(vec![
                    topic("chaves", "Quem conhece a planta e as chaves do museu?", "Os guardas, eu... e Vincent Carbone, nosso restaurador. Há três anos ele entra e sai com as telas para limpeza. Tem chave da ala holandesa. Eu confiava nele como num sobrinho.").reveals(&[9]),
                    topic("viajante", "O que a senhora sabe sobre 'O Viajante'?", "Pintor anônimo, Delft, 1658. Um homem de casaco segurando uma esfera escura com pontos vermelhos. Foi doado anonimamente em 1972. Eu passei a vida achando aquele rosto familiar. Agora sei por quê, senhor Vale.").reveals(&[11]),
                    topic("seguro", "O museu tinha seguro?", "Não o suficiente. A seguradora exigiu uma avaliação independente de todo o acervo. Seria na semana que vem. Agora não há mais o que avaliar.").req(Req::Clue(4)),
                ]),
            person("Kevin", "Doyle", false, 23, Job::Musician, Role::Suspect, B(BKind::Apartment, 6), "Guarda noturno de meio período. Toca guitarra numa banda de punk do East Village. Estava sozinho no balcão.")
                .temper(70, 35, 55, 35)
                .topics(vec![
                    topic("porta", "Como os ladrões entraram?", "Eu abri. Apertei o botão da porta lateral. Eles estavam de farda, disseram que era chamado de distúrbio, e eu estava meio chapado, tá? Quebrei o protocolo. Mas eu não sabia de nada. Eu juro pela minha mãe.")
                        .lie("Arrombaram a porta lateral. Eu estava fazendo a ronda lá em cima.", 2)
                        .reveals(&[8]),
                    topic("banda", "Sua banda precisa de dinheiro?", "Toda banda precisa de dinheiro. Mas se eu tivesse treze quadros, cara, eu não estaria morando num quarto com baratas na Avenida B."),
                ]),
            person("Oscar", "Menard", false, 58, Job::Retired, Role::Witness, B(BKind::House, 5), "Guarda veterano, ex-bombeiro. Foi amarrado no porão por quase uma hora e meia.")
                .temper(45, 70, 80, 15)
                .topics(vec![
                    topic("noite", "O que o senhor viu?", "Dois policiais. Bigodes falsos, eu percebi na hora, a cola brilhava. Um deles chamou o outro de 'rapaz do Sal'. Me algemaram, me enrolaram de fita e me largaram no porão.").reveals(&[7]),
                    topic("terceiro", "Havia mais alguém?", "Havia. Depois que me amarraram, entrou um terceiro. Luvas de látex, mãos de pianista. Chamava as telas pelo número do inventário: 'a 114 primeiro, cuidado com a 207'. Ladrão não sabe número de inventário.").req(Req::Topic("noite")).reveals(&[12]),
                    topic("viajante", "Viu levarem 'O Viajante'?", "Eu ouvi o terceiro dizer: 'Essa não está na lista.' E às três e dezessete o museu inteiro zumbiu, como uma geladeira velha. Quando a polícia de verdade chegou, a moldura dele também estava vazia."),
                ]),
            person("Vincent", "Carbone", false, 47, Job::Forger, Role::Suspect, B(BKind::Apartment, 2), "Restaurador do museu. Mãos finas, voz baixa, ateliê cheirando a terebintina no Upper West Side.")
                .temper(40, 45, 20, 80)
                .flees()
                .topics(vec![
                    topic("restauro", "O senhor retirava originais para restauração?", "...Eu copiava. Três anos. Levava o original 'para limpeza' e devolvia a minha cópia. Os originais foram para colecionadores que não fazem perguntas. A avaliação da seguradora ia ver o branco de titânio. Então as cópias precisavam desaparecer.")
                        .lie("Nunca tirei um original daquelas paredes. Restauro no local, com a sala fechada.", 3),
                    topic("noite", "Onde o senhor estava na madrugada do roubo?", "Em casa, lendo. Pergunte ao porteiro.").req(Req::Clue(12)),
                    topic("viajante", "Por que 'O Viajante' também sumiu?", "Não fui eu. Juro. Não estava na lista. Eu vi a moldura vazia às três e dezessete e achei que o Sal tinha mandado levar. Aquele quadro me dava calafrios. O homem pintado olhava o relógio.").req(Req::Clue(3)),
                ]),
            person("Sal", "Ferraro", false, 55, Job::Gangster, Role::Suspect, B(BKind::House, 8), "Receptador de Little Italy. Dono de uma loja de molduras que nunca vende moldura nenhuma.")
                .temper(20, 75, 25, 85)
                .topics(vec![
                    topic("uniformes", "Quem comprou as fardas de polícia?", "Tá bom, fui eu. O restaurador me procurou. Queria dois rapazes de farda e uma noite de São Patrício. Disse exatamente quais quadros levar. Pagou vinte mil, e os quadros eram dele pra vender. Eu só aluguei os rapazes.")
                        .lie("Farda? Eu vendo molduras, doutor. Não fantasia.", 5)
                        .reveals(&[10]),
                    topic("quadros", "Onde estão as telas?", "Se eu soubesse, estaria em Palermo. O Carbone levou tudo pro ateliê naquela mesma noite. Menos o tal Viajante. Esse ninguém levou. Esse... foi embora sozinho.").req(Req::Topic("uniformes")),
                ]),
            person("Dana", "Whitlock", true, 36, Job::Detective, Role::Contact, B(BKind::Apartment, 10), "Agente do FBI, divisão de crimes contra o patrimônio artístico. Sonha com molduras vazias desde criança.")
                .temper(20, 80, 80, 25)
                .soul(1)
                .topics(vec![
                    topic("caso", "O que o FBI tem até agora?", "Uma cena limpa demais e um guarda chapado. Meu chefe quer o garoto Doyle. Eu quero saber por que os cortes nas telas parecem feitos por um cirurgião."),
                    topic("sonho", "Você disse que sonha com molduras vazias?", "Desde os oito anos. Uma parede cheia de molduras e, em cada uma, um ano: 1920, 1946, 1971... Na última estava você, de costas, olhando o relógio. Não me olhe assim. Eu nunca contei isso a ninguém.").req(Req::Topic("caso")),
                ]),
        ],
        clues: vec![
            clue("Molduras vazias", ClueKind::Physical, Some(B(BKind::Mansion, 0)), Look3d::Object, "Treze molduras douradas ainda penduradas. As telas foram cortadas rente ao chassi.", &[3])
                .forensic("Cortes limpos, feitos com bisturi de restauro, exatamente onde a tela encontra o chassi. Um ladrão comum rasgaria; este sabia onde cortar."),
            clue("Fita adesiva dos guardas", ClueKind::Physical, Some(B(BKind::Mansion, 0)), Look3d::Object, "Rolos de fita prateada cortados das mãos e dos olhos dos guardas.", &[4])
                .forensic("Na cola, fibras azuis de tecido sintético barato — farda de loja de fantasias, não da polícia de Nova York."),
            clue("Registro do alarme", ClueKind::Document, Some(B(BKind::Mansion, 0)), Look3d::Paper, "Fita impressa do sistema: 'Porta lateral — liberada pelo balcão 1h24'. 'Porta lateral — saída 3h17'.", &[1])
                .forensic("Os sensores da ala holandesa registram três pessoas diferentes. Os guardas só viram dois policiais."),
            clue("Laudo de restauração", ClueKind::Document, Some(HomeOf(3)), Look3d::Paper, "Fichas de Carbone: 'nº 114 — retirada para limpeza, 1987. Devolvida.' O mesmo para as outras doze obras roubadas.", &[3])
                .forensic("Anexada, uma análise de pigmento que ele nunca entregou: branco de titânio na 'tela de 1633'. O titânio só existe em tinta desde 1921. A tela roubada já era falsa.")
                .hidden(1),
            clue("Carta da seguradora", ClueKind::Document, Some(HomeOf(0)), Look3d::Paper, "'Avaliação independente do acervo marcada para 25 de março, com atenção às obras restauradas por V. Carbone desde 1986.'", &[3]),
            clue("Nota de venda de duas fardas", ClueKind::Document, Some(B(BKind::Clothing, 0)), Look3d::Paper, "Loja de fantasias da Rua Canal: duas fardas de policial, dois bigodes, pago em dinheiro. Assinado: 'S. F.'.", &[4])
                .network(Job::Police),
            clue("Baseado na guarita", ClueKind::Physical, Some(B(BKind::Mansion, 0)), Look3d::Object, "Um cigarro de maconha apagado no cinzeiro do balcão de segurança.", &[1])
                .forensic("Kevin fumou enquanto estava sozinho. Não há mais nada: nenhum bilhete, nenhum telefone anotado, nenhum dinheiro.")
                .herring(),
            testimony("'O rapaz do Sal'", "Oscar: os policiais tinham bigodes falsos. Um chamou o outro de 'rapaz do Sal'.", &[4]),
            testimony("Kevin abriu a porta", "Kevin apertou o botão da porta lateral para os falsos policiais. Estava chapado e quebrou o protocolo.", &[]),
            testimony("As chaves do restaurador", "Margaret: só Vincent Carbone, além dos guardas, conhece as chaves e a planta da ala holandesa.", &[3]),
            testimony("Vinte mil pelos rapazes", "Sal: Carbone pagou vinte mil por dois homens de farda e disse exatamente quais quadros levar.", &[3, 4]),
            testimony("O rosto do Viajante", "Margaret: 'O Viajante', Delft, 1658, doado anonimamente em 1972. O rosto é o de Elias.", &[]),
            testimony("O terceiro homem", "Oscar: um terceiro homem de luvas de látex chamava as telas pelo número do inventário.", &[3]),
            clue("Eco: a ala holandesa", ClueKind::Temporal, Some(B(BKind::Mansion, 0)), Look3d::None, "No eco, mãos de luvas de látex cortam as telas com bisturi. Uma voz baixa: 'A 114 primeiro.' É a voz de Carbone.", &[3]),
            clue("Eco: o ateliê", ClueKind::Temporal, Some(HomeOf(3)), Look3d::None, "No eco, em 1987, Carbone pinta uma cópia sob a lâmpada. Num canto do ateliê, um homem de casaco observa, imóvel, e olha o relógio.", &[3]),
            clue("Etiqueta atrás da moldura do Viajante", ClueKind::Document, Some(B(BKind::Mansion, 0)), Look3d::Glow, "Colada no verso da moldura vazia: 'Doação anônima, 1972. Remetente: Laboratório Delta, Caixa Postal 317.'", &[])
                .forensic("A tinta da etiqueta tem o mesmo pigmento roxo encontrado no porão do Hotel Saint-Aubin, em 1928.")
                .hidden(2),
        ],
        echoes: vec![
            echo(B(BKind::Mansion, 0), Ghost::Hide, &["Luzes apagadas. Passos de sola macia.", "Um bisturi corre rente à moldura.", "'A 114 primeiro. Cuidado com a 207.'", "Às 3:17, a moldura do Viajante fica vazia sozinha."], 13),
            echo(HomeOf(3), Ghost::Wait, &["Cheiro de terebintina, 1987.", "Um pincel copia um rosto antigo.", "No canto, um homem de casaco olha o relógio.", "'Essa não é sua, Vincent.'"], 14),
        ],
        culprit: 3,
        methods: &[
            "Kevin Doyle deixou a quadrilha entrar e dividiu o dinheiro",
            "Carbone e dois homens de Sal, disfarçados de policiais, renderam os guardas; ele mesmo cortou as telas com bisturi",
            "A máfia de Boston roubou as obras para usá-las como moeda de troca",
            "A própria diretora simulou o roubo pelo seguro",
        ],
        method: 1,
        motives: &[
            "Pagar as dívidas da banda de punk",
            "Vingança contra a família Whitfield",
            "Esconder que as telas já eram falsificações suas antes da avaliação da seguradora",
            "Recuperar 'O Viajante' para um colecionador ocultista",
        ],
        motive: 2,
        anomalies: &[
            "Nenhuma — é apenas um crime",
            "'O Viajante', pintado em 1658, mostra Elias com a esfera; foi doado pelo Laboratório Delta e desapareceu sozinho às 3:17, fora da lista dos ladrões",
            "As molduras são de 2025",
            "Os falsos policiais eram policiais verdadeiros de outra década",
        ],
        anomaly: 1,
        threads: &[(3, 4), (3, 0), (1, 2), (4, 2), (5, 1), (0, 5)],
        story: "Havia três anos que Vincent Carbone trocava os originais do Museu Whitfield por cópias perfeitas e vendia as telas verdadeiras. Quando a seguradora marcou uma avaliação independente, as cópias precisavam sumir. Carbone contratou dois homens de Sal Ferraro, fardados de policiais, entrou atrás deles e cortou ele mesmo as próprias falsificações. Às 3:17, uma décima quarta moldura ficou vazia sem que ninguém tocasse nela.",
        sphere: "'O Viajante' nunca foi pintado em 1658 por acaso: alguém com o seu rosto posou para ele, e o Laboratório Delta o doou em 1972. Ele não foi roubado. Foi recolhido às 3:17, como uma peça que já cumpriu sua função. Você o verá de novo.",
        on_true: outcome(
            "Vincent Carbone foi preso no aeroporto Kennedy com uma lista de compradores. Sal Ferraro fez acordo. Dezenove originais voltaram de coleções particulares. As molduras continuam na parede, mas agora cheias. Só a do Viajante segue vazia.",
            "RESTAURADOR DO MUSEU WHITFIELD É PRESO: 'ROUBO' ESCONDIA FALSIFICAÇÕES",
            "...o FBI recuperou dezenove obras originais. Uma tela do século XVII continua desaparecida...",
            vec![(3, "jailed"), (4, "jailed"), (1, "survived"), (0, "grateful"), (5, "police")],
        )
        .flags(vec!["frames_true"]),
        on_false: outcome(
            "Kevin Doyle foi condenado como cúmplice. Nenhuma tela apareceu. As molduras vazias viraram atração turística. Vincent Carbone ganhou o contrato para restaurar as que sobraram.",
            "GUARDA DO MUSEU WHITFIELD É CONDENADO. OBRAS CONTINUAM SUMIDAS",
            "...a recompensa de cinco milhões de dólares segue em aberto...",
            vec![(1, "jailed"), (3, "criminal"), (0, "hates_elias")],
        )
        .flags(vec!["frames_false"]),
        digit: "0",
        reward: 105,
    }
}

pub fn case24() -> CaseDef {
    CaseDef {
        id: 24,
        title: "Noite de Fogo",
        city: CityId::LosAngeles,
        year: 1992,
        intro: "Los Angeles, 1992. O júri absolveu os policiais do vídeo, e a cidade pegou fogo. Na primeira madrugada, com a polícia recuada e os helicópteros das emissoras girando no céu, a Casa de Penhores Oh queimou até a estrutura. Daniel Oh foi encontrado nos fundos da loja com uma bala no peito, antes que as chamas o alcançassem. Todos dizem que foram os saqueadores. Mas alguém, num telhado do outro lado da rua, deixou uma filmadora ligada a noite inteira.",
        brief: "Durante os distúrbios, o penhorista Daniel Oh foi baleado dentro da própria loja em chamas. Saqueadores, um policial e uma fita de filmadora.",
        start: B(BKind::Pawn, 0),
        cast: vec![
            person("Daniel", "Oh", false, 56, Job::Pawnbroker, Role::Victim, B(BKind::House, 3), "Dono da Casa de Penhores Oh há dezoito anos. Chegou de Seul com duas malas e nenhum inglês.").dead(),
            person("Grace", "Oh", true, 24, Job::Student, Role::Witness, B(BKind::House, 3), "Filha de Daniel. Estudante de direito na UCLA. Implorou para o pai não ficar na loja naquela noite.")
                .temper(55, 70, 85, 15)
                .topics(vec![
                    topic("pai", "Seu pai tinha medo de alguém?", "De um policial. Ele nunca disse o nome. Só que um homem de farda o obrigava a comprar e vender pistolas sem número de série havia dois anos. Depois do veredito, ele me disse: 'Grace, amanhã eu vou ao jornal. Chega de polícia mandando em mim.'").reveals(&[7]),
                    topic("noite", "Por que ele ficou na loja?", "Pra proteger. Todos os comerciantes da Koreatown subiram nos telhados com espingardas, porque a polícia não vinha. Ele me mandou pra casa às onze. Foi a última vez."),
                ]),
            person("Marcus", "Tillman", false, 17, Job::Student, Role::Suspect, B(BKind::Apartment, 5), "Estudante do ensino médio. Foi pego na manhã seguinte com três caixas de relógios da loja.")
                .temper(70, 50, 55, 50)
                .topics(vec![
                    topic("loja", "Você esteve na loja do senhor Oh?", "Tá, eu peguei os relógios. A vitrine já estava quebrada, todo mundo pegando. Era meia-noite, por aí. O velho estava lá dentro, vivo, gritando com a espingarda, mas não atirou. Eu corri. Não tinha fogo nenhum ainda.")
                        .lie("Nunca cheguei perto daquela loja. Esses relógios eu achei na rua.", 3)
                        .reveals(&[8]),
                    topic("medo", "Por que você não se entrega e conta isso?", "Porque sou um moleque preto de South Central com relógios roubados, cara. Você viu o que o júri fez essa semana."),
                ]),
            person("Brent", "Kohler", false, 38, Job::Police, Role::Suspect, B(BKind::House, 9), "Policial da LAPD, divisão da Rua 77. Doze anos de farda, três queixas arquivadas.")
                .temper(25, 75, 10, 85)
                .flees()
                .topics(vec![
                    topic("noite", "Onde o senhor estava às três da manhã?", "Passei na loja, tá bom? Fui ver se o velho estava bem. Ele era... um conhecido. Estava vivo quando eu saí. A loja nem estava pegando fogo.")
                        .lie("No posto de comando da Rua 54, a noite toda. Duzentos policiais me viram.", 4),
                    topic("armas", "A cápsula na loja é de munição da polícia.", "Ele comprava umas armas de mim, e daí? Armas apreendidas que iam ser destruídas mesmo. Um negócio. Ninguém se machucava... até ele resolver virar herói.")
                        .lie("Nunca comprei nem vendi nada àquele coreano. Metade da cidade tem munição de polícia hoje.", 2)
                        .req(Req::Clue(0)),
                ]),
            person("Luis", "Ruiz", false, 29, Job::Vendor, Role::Witness, B(BKind::Apartment, 7), "Vende tacos num carrinho. Filma tudo com uma filmadora Sony comprada a prestação.")
                .temper(80, 40, 70, 40)
                .topics(vec![
                    topic("fita", "Você filmou alguma coisa naquela noite?", "Filmei do telhado, a noite inteira. Às 3:17, pelo relógio da câmera, uma viatura chegou de faróis apagados, sem sirene, e parou atrás da loja do senhor Oh. Um policial entrou pelos fundos. Um clarão. Dois minutos depois, fogo nos fundos. Eu estava com medo, mano. É a palavra de um vendedor de taco contra um distintivo.")
                        .lie("Não filmei nada. A câmera quebrou no primeiro dia.", 11)
                        .reveals(&[9]),
                ]),
            person("Earl", "Jenkins", false, 61, Job::Gunsmith, Role::Witness, B(BKind::House, 12), "Dono da loja de armas ao lado. Veterano do Vietnã. Não gostava de Oh, mas o respeitava.")
                .temper(30, 75, 60, 45)
                .topics(vec![
                    topic("kohler", "Conhece um policial chamado Kohler?", "Conheço. Me ofereceu pistolas sem número em 1990. Mandei ele pro inferno. Aí ele foi bater na porta do Oh, que tinha menos inglês e mais medo. Todo mês, uma sacola de lona.").reveals(&[10]),
                    topic("oh", "Como era Daniel Oh?", "Teimoso. Dormia com a espingarda descarregada, porque tinha medo de acertar alguém. Isso diz tudo sobre ele."),
                ]),
            person("Hattie", "Monroe", true, 44, Job::Detective, Role::Contact, B(BKind::House, 10), "Detetive de homicídios da LAPD. Tem quarenta e dois corpos da semana dos distúrbios para investigar e ninguém para ajudar.")
                .temper(30, 75, 75, 25)
                .topics(vec![
                    topic("recuo", "Havia policiais na Koreatown naquela madrugada?", "Nenhum. A ordem foi recuar pra Rua 54 às sete da noite. Nenhuma viatura tinha autorização pra entrar naquelas ruas até o amanhecer. Então, se alguém viu uma viatura lá, era alguém sem ordem nenhuma.").reveals(&[11]),
                    topic("caso", "Por que ninguém investiga a morte de Oh?", "Porque é mais fácil dizer 'saqueadores' e fechar a pasta. Me traga algo que um promotor não possa ignorar e eu arrombo essa pasta."),
                ]),
        ],
        clues: vec![
            clue("Cápsula 9mm no chão da loja", ClueKind::Physical, Some(B(BKind::Pawn, 0)), Look3d::Weapon, "Uma cápsula deflagrada, meio derretida, perto do corpo.", &[3])
                .forensic("Munição de serviço: o carimbo de lote é o que a LAPD distribuiu às divisões em 1991."),
            clue("Cofre aberto sem arrombamento", ClueKind::Physical, Some(B(BKind::Pawn, 0)), Look3d::Object, "O cofre dos fundos está aberto e vazio. Não há marca de ferramenta.", &[])
                .forensic("Aberto pela combinação. Oh o abriu para alguém que conhecia — ou sob a mira de uma arma."),
            clue("Livro de penhores queimado", ClueKind::Document, Some(B(BKind::Pawn, 0)), Look3d::Paper, "Um livro de capa preta, meio carbonizado, esquecido no chão atrás do cofre.", &[3])
                .forensic("Nas páginas que sobraram: 'K. — 14 pistolas — sem nº de série — $3.500'. Entradas mensais desde março de 1990.")
                .hidden(1),
            clue("Caixas de relógios saqueadas", ClueKind::Physical, Some(HomeOf(2)), Look3d::Object, "Três caixas de relógios da Casa de Penhores Oh, embaixo da cama de Marcus.", &[2])
                .forensic("As caixas têm fuligem por fora, mas não por dentro: foram levadas pela vitrine antes do incêndio, e não dos fundos, onde Oh morreu.")
                .herring(),
            clue("Fita da filmadora", ClueKind::Physical, Some(HomeOf(4)), Look3d::Photo, "Uma fita VHS-C escondida numa caixa de sapatos. Na etiqueta: '29-30 ABRIL — TELHADO'.", &[3])
                .forensic("Quadro a quadro, às 3:17: uma viatura de faróis apagados. No teto, o número 3A-41.")
                .network(Job::Journalist),
            clue("Registro de viaturas da divisão", ClueKind::Document, Some(B(BKind::Police, 0)), Look3d::Paper, "Viatura 3A-41 — Kohler, B. — 'Patrulha, Koreatown, 3h00–3h40'. Nenhuma comunicação por rádio no período.", &[3])
                .network(Job::Police)
                .hidden(2),
            clue("Galão de gasolina", ClueKind::Physical, Some(Near(BKind::Pawn, 0)), Look3d::Object, "Um galão vermelho vazio, largado no beco atrás da loja.", &[3])
                .forensic("Na alça, uma etiqueta raspada da frota municipal. É o modelo que as viaturas carregam no porta-malas."),
            testimony("'Amanhã eu vou ao jornal'", "Grace: um policial obrigava Oh a vender armas sem número. Depois do veredito, ele decidiu denunciar.", &[3]),
            testimony("O velho estava vivo à meia-noite", "Marcus pegou os relógios pela vitrine à meia-noite. Oh estava vivo, gritando, e não havia fogo.", &[]),
            testimony("A viatura sem sirene", "Luis filmou, às 3:17, uma viatura de faróis apagados parar atrás da loja. Um policial entrou pelos fundos. Um clarão, e depois fogo.", &[3]),
            testimony("A sacola de lona", "Earl: Kohler ofereceu pistolas sem número. Earl recusou; Oh aceitou por medo. Todo mês, uma sacola de lona.", &[3]),
            testimony("A polícia recuou", "Hattie: nenhuma viatura tinha ordem para entrar na Koreatown naquela madrugada.", &[3]),
            clue("Eco: os fundos da loja", ClueKind::Temporal, Some(B(BKind::Pawn, 0)), Look3d::None, "No eco, Kohler e Oh discutem diante do cofre aberto. Um tiro. Kohler espalha gasolina sobre o livro preto.", &[3]),
            clue("Espelho derretido", ClueKind::Physical, Some(B(BKind::Pawn, 0)), Look3d::Glow, "Um espelho de vitrine, deformado pelo calor. No reflexo que o vidro guardou, um homem de casaco comprido está parado no meio das chamas, sem queimar.", &[])
                .hidden(3),
            clue("Cautela de penhor nº 317", ClueKind::Document, Some(B(BKind::Pawn, 0)), Look3d::Glow, "Uma cautela de 1971 que sobreviveu dentro de uma lata: 'Objeto: esfera de vidro escuro com pontos vermelhos. Penhorada por E. Vale. Nunca resgatada.'", &[])
                .hidden(2),
            clue("Espingarda de Oh", ClueKind::Physical, Some(B(BKind::Pawn, 0)), Look3d::Weapon, "Uma espingarda calibre 12 embaixo do balcão.", &[])
                .forensic("Descarregada. Nunca foi disparada. Oh ameaçava, mas não atirava."),
        ],
        echoes: vec![
            echo(B(BKind::Pawn, 0), Ghost::Argue, &["Helicópteros. Sirenes longe, nenhuma perto.", "'Amanhã eu vou ao jornal, Kohler.'", "Um tiro seco. O relógio da parede: 3:17.", "Gasolina sobre um livro preto. Um fósforo."], 12),
        ],
        culprit: 3,
        methods: &[
            "Saqueadores o balearam ao invadir a loja",
            "Kohler entrou pelos fundos de viatura apagada, atirou com a arma de serviço e incendiou a loja para apagar o livro de penhores",
            "Oh se feriu com a própria espingarda",
            "O vizinho da loja de armas atirou por rivalidade",
        ],
        method: 1,
        motives: &[
            "Roubo dos relógios",
            "Rivalidade entre comerciantes",
            "Silenciar Oh, que ia denunciar ao jornal o tráfico de armas apreendidas",
            "Ódio racial durante os distúrbios",
        ],
        motive: 2,
        anomalies: &[
            "Nenhuma — é apenas um crime",
            "Na loja havia uma cautela de 1971: a esfera foi penhorada por 'E. Vale' — e um homem de casaco ficou parado no meio do fogo sem queimar",
            "O fogo começou antes de o veredito sair",
            "A fita da filmadora gravou o futuro",
        ],
        anomaly: 1,
        threads: &[(3, 0), (0, 1), (3, 5), (4, 3), (6, 3), (2, 0)],
        story: "Havia dois anos que o policial Brent Kohler desviava armas apreendidas e obrigava Daniel Oh a vendê-las pela casa de penhores. Depois do veredito, Oh decidiu contar tudo ao jornal. Na primeira madrugada dos distúrbios, com a polícia recuada, Kohler chegou de viatura apagada, fez Oh abrir o cofre, atirou nele com a arma de serviço e pôs fogo nos fundos para queimar o livro. Os saqueadores levariam a culpa.",
        sphere: "Em 1971 alguém com o seu nome penhorou a esfera nesta mesma loja e nunca voltou para buscá-la. O homem no meio do fogo não queimou porque não pertence a esta noite. O Outro Elias está recolhendo os rastros que você deixou — ou deixará.",
        on_true: outcome(
            "A fita de Luis foi ao ar no noticiário das onze. Brent Kohler foi preso por homicídio e tráfico de armas. Marcus devolveu os relógios a Grace, que reabriu a loja com o nome do pai.",
            "POLICIAL DA LAPD É PRESO PELA MORTE DE PENHORISTA NA NOITE DOS DISTÚRBIOS",
            "...a fita de um vendedor ambulante mostra uma viatura atrás da loja às 3:17 da madrugada...",
            vec![(3, "jailed"), (1, "grateful"), (2, "survived"), (4, "grateful"), (6, "police")],
        )
        .flags(vec!["fire_true"]),
        on_false: outcome(
            "Marcus Tillman foi julgado como adulto e condenado. A fita de Luis foi apagada por cima com um jogo dos Lakers. Brent Kohler foi promovido a sargento.",
            "SAQUEADOR DE 17 ANOS É CONDENADO PELA MORTE DE PENHORISTA",
            "...a Koreatown ainda conta os prejuízos da semana dos distúrbios...",
            vec![(2, "jailed"), (3, "police"), (1, "hates_elias")],
        )
        .close(vec![BKind::Pawn])
        .flags(vec!["fire_false"]),
        digit: "7",
        reward: 110,
    }
}

pub fn case25() -> CaseDef {
    CaseDef {
        id: 25,
        title: "O Carro Branco no Wilshire",
        city: CityId::LosAngeles,
        year: 1997,
        intro: "Los Angeles, 1997. D-Royal era o rapper mais tocado da Costa Oeste, e todo mundo sabia que a guerra entre gravadoras ia acabar mal. Às 3:17 da madrugada, saindo de uma festa no Wilshire, o Suburban dele parou num sinal vermelho. Um carro branco emparelhou à direita. A janela desceu. Onze tiros. O carro sumiu na avenida. Dois seguranças, uma gravadora rival, policiais de folga fazendo bico — e uma testemunha jura que, no banco do passageiro do carro branco, havia um homem de casaco olhando o relógio.",
        brief: "O rapper D-Royal foi morto num sinal vermelho do Wilshire por alguém num carro branco. Rixa de gravadoras, policiais corruptos, um segurança nervoso.",
        start: Near(BKind::Bank, 0),
        cast: vec![
            person("Darnell", "Hayes", false, 24, Job::Musician, Role::Victim, B(BKind::Mansion, 1), "D-Royal. Dois discos de platina. Cresceu em Compton e prometia contar 'a verdade da rua' no próximo disco.").dead(),
            person("Cyrus", "Drummond", false, 33, Job::Gangster, Role::Suspect, B(BKind::Mansion, 3), "Dono da Sepulcro Records, a gravadora rival. Contrata policiais de folga como seguranças.")
                .temper(20, 80, 25, 90)
                .topics(vec![
                    topic("rixa", "A Sepulcro estava em guerra com D-Royal?", "Guerra vende disco, irmão. Diss track, entrevista, capa de revista. Eu queria ele vivo e com raiva de mim. Morto, ele vende mais pra outra gravadora."),
                    topic("mercer", "O que o senhor sabe de Dwayne Mercer?", "Mercer faz a minha segurança há dois anos. Semana passada me pediu dez mil em dinheiro vivo, adiantado. Disse que tinha 'um problema com um cantor que falou demais'. Eu achei que era algum dos meus. Juro que eu não mandei ninguém atirar.")
                        .lie("Mercer? Não conheço nenhum Mercer. Minha segurança é terceirizada.", 3)
                        .reveals(&[11]),
                ]),
            person("Dwayne", "Mercer", false, 31, Job::Police, Role::Suspect, B(BKind::House, 6), "Policial da divisão Rampart da LAPD. Faz bico de segurança para a Sepulcro Records. Atirador de competição.")
                .temper(15, 85, 10, 85)
                .flees()
                .topics(vec![
                    topic("noite", "Onde o senhor estava às três da manhã?", "Tá. Fiz segurança extra na festa, bico, fora dos livros. Saí à meia-noite. Não tenho nada a ver com o que aconteceu no sinal.")
                        .lie("Doente, em casa. Liguei pro plantão às seis da tarde. Está no registro.", 3),
                    topic("carro", "O senhor tem um Impala branco?", "Tinha um preto. Mandei pintar. Não é crime gostar de branco. Metade de Los Angeles dirige carro branco.").req(Req::Clue(4)),
                    topic("banco", "D-Royal ia falar no rádio sobre o assalto ao banco do Wilshire.", "...Esse moleque estava na fila do caixa, de boné, quando eu entrei. Me reconheceu da festa. Falou no telefone com a namorada, na minha frente. Ia contar pra cidade inteira. Você sabe o que acontece com policial na cadeia?").req(Req::Clue(12)),
                ]),
            person("Reggie", "Coles", false, 41, Job::Driver, Role::Suspect, B(BKind::Apartment, 4), "Chefe de segurança e motorista de D-Royal. Ex-fuzileiro. Estava no carro de trás.")
                .temper(70, 45, 40, 60)
                .topics(vec![
                    topic("sinal", "Por que o comboio parou no sinal?", "...Porque eu mandei. E mandei o Darnell passar pro carro da frente. Um policial, o Mercer, me deu cinco mil pra saber em qual carro ele ia. Disse que era só pra dar um susto, uma mensagem da Sepulcro. Eu juro que achei que era um susto.")
                        .lie("Eu não dei ordem nenhuma. O motorista parou sozinho no amarelo.", 6)
                        .reveals(&[8]),
                    topic("darnell", "Como era Darnell fora do palco?", "Um garoto que lia a Bíblia no camarim e ligava pra avó todo domingo. Eu devia ter morrido no lugar dele."),
                ]),
            person("Keisha", "Lowe", true, 23, Job::Dancer, Role::Witness, B(BKind::Apartment, 9), "Namorada de D-Royal. Dançarina dos clipes. Estava no carro de trás.")
                .temper(65, 55, 75, 30)
                .topics(vec![
                    topic("noite", "O que aconteceu antes da saída?", "O Reggie ficou esquisito. Recebeu um bipe, olhou pra mim e mandou o Darnell trocar de carro, ir no da frente, sozinho. E disse pro motorista: 'No sinal do Wilshire, para. Não fura.' O Reggie sempre mandava furar sinal.").reveals(&[7]),
                    topic("carro", "Você viu o carro branco?", "Vi. Um Impala branco, novo. O motorista de gravata-borboleta. E no banco do lado... um homem mais velho, de casaco, olhando o relógio como se estivesse atrasado. Ele olhou pra mim. Tinha os seus olhos, moço.").req(Req::Topic("noite")),
                ]),
            person("Simone", "Achebe", true, 35, Job::RadioHost, Role::Contact, B(BKind::Apartment, 2), "Apresenta o programa de hip-hop da madrugada numa rádio de South Central. Darnell ia ao ar com ela no dia seguinte.")
                .temper(25, 80, 80, 25)
                .soul(1)
                .topics(vec![
                    topic("sepulcro", "Qual a ligação da Sepulcro com a polícia?", "A Sepulcro contrata policiais da Rampart de folga como segurança. Metade anda armada com arma de serviço. O preferido do Drummond é um tal de Mercer, atirador de competição.").reveals(&[9]),
                    topic("entrevista", "O que Darnell ia dizer no seu programa?", "Ele me ligou às duas da manhã, rindo de nervoso: 'Simone, amanhã eu conto quem roubou o banco do Wilshire mês passado. O cara tem distintivo. E estava na festa hoje.' Uma hora depois, ele estava morto.").reveals(&[12]),
                    topic("voce", "Nós já nos encontramos, Simone?", "Eu não sei. Às vezes, no ar, às 3:17, a estática da rádio forma uma voz dizendo o seu nome. Já dei outros nomes, em outras cidades. Jornalista, enfermeira... É como se eu fosse sempre a mesma mulher esperando você terminar alguma coisa.").req(Req::Topic("entrevista")),
                ]),
            person("Frank", "Russo", false, 50, Job::Detective, Role::Contact, B(BKind::House, 11), "Detetive de homicídios. Honesto demais para a divisão onde trabalha.")
                .temper(35, 70, 80, 20)
                .topics(vec![
                    topic("travado", "Por que a investigação não anda?", "Porque tiraram de mim. Um capitão da Rampart levou a pasta e o nome do Mercer sumiu do relatório de campo. Eu tenho a cópia carbono. Não posso usar. Você pode.").reveals(&[10]),
                    topic("tiros", "O que a perícia diz dos tiros?", "Onze tiros, agrupamento de estande. Não foi moleque de gangue. Foi alguém que treina toda semana, com munição que não se compra em loja."),
                ]),
        ],
        clues: vec![
            clue("Cápsulas no asfalto", ClueKind::Physical, Some(Near(BKind::Bank, 0)), Look3d::Weapon, "Onze cápsulas 9mm espalhadas na faixa da direita do Wilshire.", &[2])
                .forensic("Munição alemã Gecko, rara nos Estados Unidos. Um lote idêntico foi apreendido pela Rampart em 1996 e 'extraviado' do depósito de provas."),
            clue("Vidro estilhaçado do Suburban", ClueKind::Physical, Some(Near(BKind::Bank, 0)), Look3d::Blood, "Cacos do vidro do passageiro e sangue no meio-fio.", &[2])
                .forensic("Os tiros vieram de um carro mais baixo, emparelhado à direita, disparados pela janela do motorista. Atirador destro e treinado."),
            clue("Registro de ponto da Rampart", ClueKind::Document, Some(B(BKind::Police, 0)), Look3d::Paper, "Mercer, D. — 'licença médica' na noite do crime. Veículo particular registrado: Chevrolet Impala 1997, cor branca.", &[2])
                .network(Job::Police)
                .hidden(2),
            clue("Lista de segurança da festa", ClueKind::Document, Some(B(BKind::Club, 0)), Look3d::Paper, "Lista da equipe da festa no Wilshire: 'Segurança Sepulcro Records — D. Mercer (LAPD, fora de serviço)'.", &[2, 1]),
            clue("Impala branco na oficina", ClueKind::Physical, Some(B(BKind::Warehouse, 2)), Look3d::Object, "Um Impala 1997 branco numa oficina de funilaria, com a placa retirada.", &[2])
                .forensic("Pintado de branco sobre preto há três semanas. Resíduo de pólvora no painel do motorista. No banco do passageiro, o cinto está afivelado — e não há uma única fibra, nem poeira, como se ninguém jamais tivesse sentado ali.")
                .hidden(1),
            clue("Fita da faixa 'Sinal Vermelho'", ClueKind::Document, Some(B(BKind::Radio, 0)), Look3d::Paper, "Uma demo da Sepulcro Records em que um rapper promete 'apagar D-Royal no sinal vermelho'.", &[1])
                .forensic("A faixa foi gravada há seis meses. O rapper que canta está preso em Chino desde março. É marketing, não confissão.")
                .herring(),
            clue("Extrato bancário de Reggie", ClueKind::Document, Some(HomeOf(3)), Look3d::Paper, "Depósito em dinheiro de $5.000, feito no dia anterior à festa.", &[3])
                .hidden(1),
            testimony("'No sinal, para'", "Keisha: Reggie recebeu um bipe, mandou Darnell trocar para o carro da frente e ordenou parar no sinal do Wilshire.", &[3]),
            testimony("Cinco mil por um carro", "Reggie: Mercer pagou cinco mil dólares para saber em qual carro Darnell iria. 'Era só um susto.'", &[2]),
            testimony("Policiais de aluguel", "Simone: a Sepulcro contrata policiais da Rampart de folga. O favorito de Drummond é Mercer.", &[1, 2]),
            testimony("O nome que sumiu", "Russo: um capitão da Rampart levou a pasta e o nome de Mercer sumiu do relatório.", &[2]),
            testimony("Dez mil adiantados", "Drummond: Mercer pediu dez mil em dinheiro por 'um problema com um cantor que falou demais'.", &[2]),
            testimony("O assalto ao banco do Wilshire", "Simone: Darnell ia revelar no rádio que o assaltante do banco do Wilshire tinha distintivo e estava na festa.", &[2]),
            clue("Eco: o sinal vermelho", ClueKind::Temporal, Some(Near(BKind::Bank, 0)), Look3d::None, "No eco, o Impala branco emparelha. Mercer, de gravata-borboleta, baixa o vidro. No banco do passageiro, o Outro Elias olha o relógio: 3:17.", &[2]),
            clue("Eco: o camarim", ClueKind::Temporal, Some(B(BKind::Club, 0)), Look3d::None, "No eco, Reggie lê um bipe e empalidece. 'Carro da frente, Darnell. Hoje você vai sozinho.'", &[3]),
            clue("Bipe de Darnell", ClueKind::Physical, Some(HomeOf(0)), Look3d::Glow, "O pager de Darnell, devolvido à família. A última mensagem chegou às 3:17: '317 — NÃO PARE NO SINAL — E.V.'", &[])
                .forensic("A mensagem não aparece nos registros da operadora. Foi enviada de um número que só será criado em 2025.")
                .hidden(3),
        ],
        echoes: vec![
            echo(Near(BKind::Bank, 0), Ghost::Wait, &["Luz vermelha refletida no capô.", "Um Impala branco para à direita.", "No banco do passageiro, um homem de casaco olha o relógio.", "A janela desce. Onze clarões."], 13),
            echo(B(BKind::Club, 0), Ghost::Argue, &["Baixo pesado atrás da parede.", "Um bipe vibra.", "'Carro da frente, Darnell. Confia em mim.'", "Uma Bíblia fica esquecida no sofá."], 14),
        ],
        culprit: 2,
        methods: &[
            "Um atirador da gangue rival disparou de um carro roubado",
            "Mercer emparelhou um Impala branco no sinal vermelho e atirou pela janela do motorista, sabendo o carro pelo segurança subornado",
            "Reggie atirou do carro de trás",
            "Drummond contratou um pistoleiro de fora da cidade",
        ],
        method: 1,
        motives: &[
            "A guerra entre as gravadoras",
            "Ciúme por causa de Keisha",
            "Darnell o reconheceu no assalto ao banco do Wilshire e ia denunciá-lo no rádio",
            "Uma dívida de Darnell com a Sepulcro",
        ],
        motive: 2,
        anomalies: &[
            "Nenhuma — é apenas um crime",
            "O Outro Elias estava no banco do passageiro do carro branco às 3:17 — e o bipe de Darnell recebeu um aviso assinado 'E.V.', vindo de um número que ainda não existe",
            "O carro branco não existia: foi uma alucinação coletiva",
            "D-Royal fingiu a própria morte",
        ],
        anomaly: 1,
        threads: &[(2, 0), (2, 1), (3, 2), (4, 3), (5, 0), (6, 2)],
        story: "Um mês antes, o policial Dwayne Mercer assaltou uma agência bancária no Wilshire — e Darnell estava na fila do caixa. Na festa, o rapper o reconheceu e marcou para contar tudo no programa de Simone. Mercer subornou Reggie para saber em que carro Darnell iria, esperou no sinal com seu Impala recém-pintado de branco e atirou com munição desviada do depósito de provas. A rixa das gravadoras era a cortina perfeita.",
        sphere: "O Outro Elias estava sentado ao lado do assassino e não impediu nada. Mas alguém mandou o bipe: 'NÃO PARE NO SINAL — E.V.'. Ou existe mais de um Elias nesta noite além de você, ou o Outro Elias está tentando desfazer algo que ele próprio deixou acontecer. Faltam poucos anos para 2025.",
        on_true: outcome(
            "Dwayne Mercer foi preso com o Impala ainda na oficina. A cópia carbono de Russo derrubou um capitão da Rampart. Drummond foi indiciado por contratar policiais armados. Simone dedicou um programa inteiro a Darnell, às 3:17 da madrugada.",
            "POLICIAL DA RAMPART É PRESO PELA MORTE DO RAPPER D-ROYAL",
            "...o caso revelou um esquema de policiais corruptos fazendo bico para gravadoras de rap...",
            vec![(2, "jailed"), (1, "jailed"), (3, "left_city"), (5, "grateful"), (6, "police"), (4, "grateful")],
        )
        .flags(vec!["white_car_true"]),
        on_false: outcome(
            "A polícia culpou a guerra das gravadoras e ninguém foi preso. Dwayne Mercer continuou na Rampart. O carro branco virou lenda. Todo ano, às 3:17, alguém deixa flores no sinal do Wilshire.",
            "MORTE DE D-ROYAL CONTINUA SEM SOLUÇÃO",
            "...a polícia atribui o crime à rivalidade entre gravadoras da Costa Leste e da Costa Oeste...",
            vec![(2, "police"), (1, "criminal"), (5, "hates_elias")],
        )
        .flags(vec!["white_car_false"]),
        digit: "1",
        reward: 120,
    }
}
