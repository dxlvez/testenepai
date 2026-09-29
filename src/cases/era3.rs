//! ERA III — 1946–1966 — Adelaide, Berlim, Bergen.

use super::defs::*;
use crate::city::gen::CityId;
use crate::city::map::BKind;
use crate::sim::people::Job;
use Place::*;

pub fn cases() -> Vec<CaseDef> {
    vec![case11(), case12(), case13(), case14(), case15()]
}

pub fn case11() -> CaseDef {
    CaseDef {
        id: 11,
        title: "Tamam Shud",
        city: CityId::Adelaide,
        year: 1948,
        intro: "Adelaide, 1948. Na manhã de 1º de dezembro, um homem de terno amanheceu morto na areia da Praia, a cabeça apoiada no muro do calçadão, um cigarro apagado na gola. Não tinha documentos, carteira nem nome. Alguém cortou todas as etiquetas das roupas dele. No bolso secreto da calça, um papel enrolado com duas palavras em persa: Tamám Shud — 'está terminado'.",
        brief: "Homem desconhecido morto na praia, sem etiquetas nas roupas. No bolso, o fim de um poema persa. O legista suspeita de veneno, mas não o encontra.",
        start: Woods,
        cast: vec![
            person("Desconhecido", "da Praia", false, 45, Job::None, Role::Victim, B(BKind::Hotel, 2), "Um homem de uns quarenta e cinco anos, bem barbeado, mãos sem calos, panturrilhas de bailarino. Ninguém em Adelaide diz conhecê-lo.").dead(),
            person("Esther", "Lowell", true, 27, Job::Nurse, Role::Witness, B(BKind::House, 4), "Enfermeira do Hospital Real. Casada com o farmacêutico Leonard Prosser, tem um filho pequeno. Quase desmaiou ao ver o molde de gesso do rosto do morto.")
                .temper(75, 45, 70, 20)
                .soul(1)
                .topics(vec![
                    topic("livro", "A senhora conhecia este homem?", "...Eu dei aquele livro a ele. Sydney, 1945, no fim da guerra. Ele dizia se chamar Alf. Na semana passada bateu na minha porta, depois de três anos. Eu disse que estava casada. Ele olhou para o meu filho por muito tempo e foi embora sem dizer nada.")
                        .lie("Nunca vi esse homem. Não sei por que o meu telefone estava no livro dele. Muita gente tem o meu número, sou enfermeira.", 3)
                        .reveals(&[9]),
                    topic("marido", "Como o seu marido reagiu à visita?", "Leonard viu ele no portão. Não disse nada. Passou a noite na farmácia, contando frascos. No dia seguinte me perguntou, muito calmo, se o menino tinha os olhos do pai. Eu não respondi. Foi a pior coisa que eu podia ter feito.")
                        .req(Req::Topic("livro"))
                        .reveals(&[10]),
                    topic("voce", "Por que a senhora olha para mim desse jeito?", "Porque eu já sonhei com você, Elias. Numa cidade de jazz, eu escrevia para um jornal e você fazia perguntas demais. Às vezes acordo às três e dezessete com cheiro de tinta de máquina de escrever. Isso tem nome?"),
                ]),
            person("Leonard", "Prosser", false, 38, Job::Pharmacist, Role::Suspect, B(BKind::House, 4), "Farmacêutico da Rua Jetty. Metódico, gentil com os fregueses, casado com Esther. Tem as unhas roídas até a carne.")
                .temper(40, 45, 40, 30)
                .flees()
                .topics(vec![
                    topic("noite", "Onde o senhor estava na noite de 30 de novembro?", "...Fui à praia. Queria olhar a cara dele. Levei um pastel e um café na garrafa térmica, como se fôssemos dois homens civilizados. Ele comeu. Não falamos de Esther. Quando fui embora, ele estava vivo, olhando o mar.")
                        .lie("Na farmácia, fechando o caixa até a meia-noite. Depois, em casa, com a minha esposa.", 8),
                    topic("veneno", "Falta digitalina no seu estoque.", "Os legistas de Adelaide procuram arsênico e estricnina. Digitalina some no sangue em poucas horas e parece coração fraco. Todo farmacêutico sabe disso... Eu não disse que usei. Eu disse que todo farmacêutico sabe.")
                        .lie("Um frasco quebrou na semana passada. Acontece. Joguei fora.", 6)
                        .req(Req::Clue(5)),
                    topic("carro", "O livro estava num carro estacionado perto da sua farmácia.", "O carro é do meu cunhado. Fica destrancado, qualquer um joga um livro no banco de trás. Qualquer um.").req(Req::Clue(2)),
                ]),
            person("Frank", "Doyle", false, 51, Job::Detective, Role::Contact, B(BKind::House, 9), "Sargento-detetive da polícia de Adelaide. Convencido de que o morto é espião.")
                .temper(30, 70, 55, 40)
                .topics(vec![
                    topic("caso", "O que a polícia tem até agora?", "Um morto sem nome, sem etiquetas, com a arcada dentária que não bate com ficha nenhuma do país. O legista jura que foi veneno, mas não acha o veneno. E uma mala no guarda-volumes da estação, despachada na manhã do dia 30."),
                    topic("espiao", "De quem o senhor suspeita?", "Dos russos. Woomera está cheia de segredos de foguete e aquele código no livro não é de gente comum. Tem um estivador, Volkov, que escreve cartas em cirílico toda semana. Estou de olho nele.").reveals(&[11]),
                ]),
            person("Olive", "Pratt", true, 56, Job::Housewife, Role::Witness, B(BKind::House, 6), "Mora de frente para o calçadão da Praia. Passeia com o marido todo fim de tarde.")
                .temper(50, 50, 75, 20)
                .topics(vec![
                    topic("praia", "A senhora viu o homem na praia?", "Às sete da noite ele estava deitado contra o muro. Levantou o braço devagar e deixou cair. Pensei que era um bêbado. Mais tarde, já escuro, vi outro homem curvado sobre ele, de jaleco claro por baixo do casaco. Parecia ajeitar o cigarro na gola dele.").reveals(&[8]),
                    topic("luz", "Viu mais alguma coisa naquela noite?", "Acordei de madrugada sem motivo. O relógio da sala marcava três e dezessete. Na areia, onde ele estava, havia uma luz roxa, baixinha, como um lampião debaixo d'água. Meu marido diz que eu sonhei.").req(Req::Topic("praia")),
                ]),
            person("Mikhail", "Volkov", false, 34, Job::Dockworker, Role::Suspect, B(BKind::Apartment, 3), "Estivador ucraniano, refugiado de guerra. Fala inglês com sotaque pesado e desconfia de policiais.")
                .temper(55, 60, 65, 35)
                .flees()
                .topics(vec![
                    topic("cartas", "O senhor escreve cartas em cirílico?", "Para minha mãe, em Odessa. Ela não responde desde 1946. O seu sargento acha que tudo que é russo é espião. Eu carrego sacos de trigo, moço. O único segredo que eu tenho é que choro no porão do navio."),
                    topic("morto", "O senhor viu o morto antes?", "Na estação, de manhã. Ele despachou uma mala marrom e comprou bilhete para a praia. Um homem de camisa branca, com cheiro de éter, veio falar com ele na plataforma. Falaram baixo. O de camisa branca tremia.").reveals(&[12]),
                ]),
        ],
        clues: vec![
            clue("O corpo na areia", ClueKind::Physical, Some(Woods), Look3d::Object, "Terno bom, sapatos engraxados demais para quem andou na areia. Todas as etiquetas das roupas foram cortadas. Um cigarro apagado na gola.", &[])
                .forensic("Baço inchado, sangue no estômago, pupilas contraídas: sinais de um glicosídeo cardíaco. No estômago, restos de um pastel comido umas quatro horas antes da morte. Numa costura interna do cinto, uma etiqueta esquecida: 'Fabricado em 1969'."),
            clue("Fragmento 'Tamám Shud'", ClueKind::Physical, Some(Woods), Look3d::Paper, "Um papel minúsculo, enrolado no bolso secreto da calça, com as últimas palavras do Rubaiyat de Omar Khayyam: Tamám Shud.", &[])
                .hidden(1)
                .forensic("O papel foi rasgado de um exemplar da edição neozelandesa do Rubaiyat. A borda do rasgo encaixaria perfeitamente na última página do livro certo."),
            clue("O livro com o código", ClueKind::Document, Some(Near(BKind::Pharmacy, 0)), Look3d::Paper, "Um Rubaiyat jogado no banco de trás de um carro destrancado, perto da farmácia da Rua Jetty. Falta a última página. No verso, cinco linhas de letras a lápis: um código.", &[2])
                .forensic("O rasgo da última página casa com o fragmento do bolso do morto. O código parece uma lista de iniciais — e a segunda linha repete três vezes o mesmo nome abreviado: L.P."),
            clue("Número de telefone no livro", ClueKind::Document, Some(Near(BKind::Pharmacy, 0)), Look3d::Paper, "Na contracapa do Rubaiyat, a lápis, um número de telefone de Glenelg. É o telefone da casa de Esther Lowell.", &[1]),
            clue("A mala da estação", ClueKind::Physical, Some(B(BKind::Station, 0)), Look3d::Object, "Uma mala marrom deixada no guarda-volumes da estação de Adelaide. Etiquetas cortadas. Dentro: linha de costura laranja, um pincel de marcar carga, uma faca de mesa serrada.", &[])
                .forensic("A linha laranja não é vendida na Austrália. O pincel é de marcador de carga de navio — o morto trabalhou em porões. Nenhum espião carrega um pincel de estiva.")
                .network(Job::Police),
            clue("Frasco de digitalina", ClueKind::Physical, Some(WorkOf(2)), Look3d::Object, "Um frasco de tintura de digitalina, quase vazio, escondido atrás das caixas de xarope na farmácia de Leonard Prosser.", &[2])
                .hidden(2)
                .forensic("Faltam cerca de trinta mililitros: dose suficiente para parar o coração de um homem adulto em poucas horas. Há resíduo de café na borda do conta-gotas."),
            clue("Livro de venenos da farmácia", ClueKind::Document, Some(WorkOf(2)), Look3d::Paper, "O registro obrigatório de venenos. A página de 30 de novembro foi arrancada com régua. A de 29 registra o estoque de digitalina cheio.", &[2])
                .network(Job::Pharmacist),
            clue("Cartas em cirílico", ClueKind::Document, Some(HomeOf(5)), Look3d::Paper, "Um maço de cartas em alfabeto russo, nunca enviadas, no quarto de Mikhail Volkov. O sargento Doyle já as chamou de 'material de espionagem'.", &[5])
                .forensic("Traduzidas: perguntas a uma mãe sobre a saúde, sobre a neve, sobre um irmão. Nenhuma cifra, nenhum número.")
                .herring(),
            testimony("O homem de jaleco", "Olive Pratt viu, à noite, um homem de jaleco claro curvado sobre o morto, ajeitando o cigarro na gola dele.", &[2]),
            testimony("O livro que Esther deu", "Esther deu o Rubaiyat ao morto em Sydney, em 1945. Ele voltou na semana passada e olhou longamente para o filho dela.", &[1]),
            testimony("O ciúme de Leonard", "Leonard viu o homem no portão e perguntou a Esther se o menino tinha os olhos do pai.", &[2]),
            testimony("A teoria do espião", "O sargento Doyle acredita que o morto era espião soviético e desconfia do estivador Volkov.", &[5]),
            testimony("O homem de camisa branca", "Volkov viu o morto na estação falando com um homem de camisa branca, com cheiro de éter, que tremia.", &[2]),
            clue("Eco: a praia ao entardecer", ClueKind::Temporal, Some(Woods), Look3d::None, "No eco, dois homens sentados na areia. Um oferece um pastel e uma garrafa térmica. O outro come olhando o mar. Muito depois, um relógio distante marca 3:17.", &[2]),
            clue("A última linha do código", ClueKind::Document, Some(Near(BKind::Pharmacy, 0)), Look3d::Glow, "Sob o código, quase apagada, uma sexta linha a lápis que ninguém da polícia viu: 'DELTA 1971 — 3:17'. A caligrafia é a sua.", &[])
                .hidden(3),
            clue("Eco: o balcão da farmácia", ClueKind::Temporal, Some(WorkOf(2)), Look3d::None, "No eco, um homem de jaleco pinga gotas de um frasco escuro numa garrafa térmica de café. Conta em voz alta. Para em trinta.", &[2]),
        ],
        echoes: vec![
            echo(Woods, Ghost::Wait, &["O mar escurece.", "Dois homens na areia, lado a lado.", "'Coma. É por conta da casa.'", "Um braço sobe devagar e cai.", "Um relógio distante: 3:17."], 13),
            echo(WorkOf(2), Ghost::Write, &["Uma farmácia fechada, luz de abajur.", "Um frasco escuro, um conta-gotas.", "'...vinte e oito, vinte e nove, trinta.'", "Uma página arrancada com régua."], 15),
        ],
        culprit: 2,
        methods: &[
            "Estrangulado em outro lugar e deixado na praia",
            "Digitalina da farmácia pingada no café e servida com um pastel, na própria praia",
            "Suicídio com um veneno que ele mesmo carregava",
            "Um agente soviético o envenenou com uma seringa na estação",
        ],
        method: 1,
        motives: &[
            "Espionagem — o morto levava segredos do campo de foguetes de Woomera",
            "Ciúme — o morto era o pai do filho de Esther e voltou por ela",
            "Roubo da mala da estação",
            "Uma dívida antiga de guerra",
        ],
        motive: 1,
        anomalies: &[
            "Nenhuma — é apenas um crime",
            "Há uma etiqueta de 1969 no cinto e uma linha do código escrita com a sua letra: 'DELTA 1971 — 3:17' — o morto veio de mais adiante na linha do tempo",
            "O Rubaiyat foi impresso depois da morte do homem",
            "O morto ainda está vivo em Sydney",
        ],
        anomaly: 1,
        threads: &[(2, 0), (1, 0), (1, 2), (3, 5), (5, 0)],
        story: "O morto conheceu Esther em Sydney em 1945; o filho dela era dele. Voltou a Adelaide para vê-lo uma única vez e cortou as etiquetas das roupas para não deixar rastro. Leonard Prosser o viu no portão, tirou trinta gotas de digitalina da própria farmácia, arrancou a página do registro e foi à praia com um pastel e um café 'de homem para homem'. Depois jogou o Rubaiyat no carro do cunhado.",
        sphere: "Tamám Shud: está terminado. O morto não cortou as etiquetas para esconder o nome, mas o ano — as roupas dele são de depois de 1969. Ele já tinha tocado a esfera uma vez. Voltou para ver o filho, e a linha do tempo o fechou às 3:17. A última linha do código é sua: um recado que você ainda vai escrever.",
        on_true: outcome(
            "Leonard Prosser confessou diante do frasco e da página arrancada. Esther ficou em Adelaide com o menino e nunca mais atendeu o telefone depois da meia-noite. O morto foi enterrado sem nome, com uma lápide que diz apenas: 'O Desconhecido'.",
            "FARMACÊUTICO DE GLENELG É PRESO PELA MORTE DO HOMEM DA PRAIA",
            "...o homem da praia continua sem nome, mas a polícia diz ter encontrado o assassino...",
            vec![(2, "jailed"), (1, "survived"), (3, "police")],
        )
        .flags(vec!["tamam_shud_true"]),
        on_false: outcome(
            "Mikhail Volkov foi deportado como espião. O caso virou lenda: o código nunca foi decifrado. Toda noite de 30 de novembro, alguém deixa um cigarro apagado no muro da Praia.",
            "ESTIVADOR ESTRANGEIRO É ACUSADO NO CASO DO HOMEM DA PRAIA",
            "...o código do Rubaiyat segue sem solução, afirma a polícia de Adelaide...",
            vec![(5, "jailed"), (2, "criminal")],
        )
        .flags(vec!["tamam_shud_false"]),
        digit: "5",
        reward: 85,
    }
}

pub fn case12() -> CaseDef {
    CaseDef {
        id: 12,
        title: "As Crianças da Praia",
        city: CityId::Adelaide,
        year: 1956,
        intro: "Adelaide, 1956. Dia de calor, feriado de janeiro. Os três irmãos Whitlock — Nora, nove anos, Peter, sete, e Tommy, quatro — pegaram o ônibus das dez para a Praia e deviam voltar no do meio-dia. O ônibus voltou sem eles. Na areia ficaram três toalhas dobradas e uma quarta, de adulto, que ninguém da família reconhece.",
        brief: "Três irmãos sumiram depois de um dia na praia. Foram vistos com um homem alto, magro e loiro. Na loja do calçadão, pagaram com uma nota de uma libra que não tinham.",
        start: HomeOf(1),
        cast: vec![
            person("Nora", "Whitlock", true, 9, Job::Child, Role::Victim, B(BKind::House, 5), "Nove anos. Cuidava dos irmãos menores. Desenhava bolas escuras com pontos vermelhos no caderno da escola.").dead(),
            person("Margaret", "Whitlock", true, 38, Job::Housewife, Role::Witness, B(BKind::House, 5), "A mãe. Não sai da janela desde o dia do sumiço. O marido viaja vendendo máquinas de costura.")
                .temper(80, 40, 80, 15)
                .topics(vec![
                    topic("dia", "Como foi a manhã em que eles saíram?", "Dei a Nora oito xelins e seis pence: ônibus, tortas e um refrigerante. Tommy chorou porque queria levar o baldinho vermelho. Eles iam voltar no ônibus do meio-dia. O ônibus chegou vazio, e eu soube. Mãe sabe.").reveals(&[7]),
                    topic("nota", "Nora pagou a loja com uma nota de uma libra.", "Uma libra? Eu nunca dei uma libra a Nora. Nós não temos uma libra sobrando nesta casa! Alguém deu aquela nota a ela. Alguém que ela conhecia, porque Nora não aceitava nada de estranhos.").req(Req::Clue(1)).reveals(&[8]),
                    topic("homem", "As crianças conheciam algum homem alto e loiro?", "Nora falava de um 'professor de natação' que ensinava as crianças no raso, aos sábados. Achei bonito, alguém ensinando de graça. Meu Deus. Eu achei bonito."),
                ]),
            person("Gerald", "Ashby", false, 34, Job::Teacher, Role::Suspect, B(BKind::House, 8), "Professor primário. Alto, magro, loiro, sempre bronzeado. Dá aulas de natação de graça às crianças na Praia.")
                .temper(35, 40, 20, 30)
                .flees()
                .topics(vec![
                    topic("praia", "Onde o senhor estava na manhã de 26 de janeiro?", "...Estava na praia. Como todo feriado. Brinquei com eles, sequei o menorzinho com a minha toalha. Isso é crime agora? Eles gostam de mim. As crianças sempre gostam de mim.")
                        .lie("Em casa, corrigindo provas. Com esse calor eu nem saio.", 3),
                    topic("fazenda", "O que há na fazenda da sua família?", "Era da minha mãe. Ninguém pisa lá desde 1950... Eu vou lá às vezes. Para lembrar. Tinha um quarto de brinquedos lá embaixo. Era dos meus irmãos, antes do mar levar os dois.")
                        .lie("Vendemos a fazenda há anos. Não sei nem quem é o dono agora.", 5),
                    topic("irmaos", "O que aconteceu com os seus irmãos?", "Afogaram na Praia, em 1936. Eu tinha catorze anos e estava olhando os dois. Minha mãe nunca mais falou comigo. Três crianças na areia, sozinhas... alguém tem que cuidar delas, entende? Alguém tem que cuidar.").req(Req::Topic("fazenda")),
                ]),
            person("Doris", "Keel", true, 51, Job::Vendor, Role::Witness, B(BKind::House, 2), "Dona da lojinha de tortas e refrigerantes do calçadão. Conhece metade das crianças de Glenelg pelo nome.")
                .temper(40, 55, 75, 40)
                .topics(vec![
                    topic("compra", "As crianças Whitlock compraram algo na loja?", "Às onze e pouco. A Nora pagou com uma nota de uma libra — nunca tinham comprado com nota. E pediu tortas para quatro. Quatro! Eu perguntei para quem era a quarta e ela disse: 'Para o nosso amigo'.").reveals(&[9]),
                ]),
            person("Stan", "Hollis", false, 60, Job::Photographer, Role::Witness, B(BKind::Apartment, 2), "Fotógrafo de praia. Tira retratos de banhistas e vende as cópias no dia seguinte.")
                .temper(45, 50, 65, 50)
                .topics(vec![
                    topic("fotos", "O senhor fotografou a praia naquele dia?", "O dia todo. Nas fotos do fim da manhã aparecem três crianças brincando com um homem alto, magro, loiro, de calção azul. Ele secava o menorzinho com uma toalha. Depois os quatro foram para o vestiário. Eu revelei ontem. Não dormi.").reveals(&[10]),
                ]),
            person("Cyril", "Dunn", false, 47, Job::Drifter, Role::Suspect, B(BKind::Hotel, 4), "Vagabundo que dorme debaixo do píer. Ex-marinheiro, bebe vinho barato, fala sozinho.")
                .temper(70, 30, 55, 45)
                .flees()
                .topics(vec![
                    topic("praia", "O senhor viu as três crianças?", "Vi, patrão. Lá pelo meio-dia, seguindo um homem alto até a rua de trás. O menorzinho ia no colo dele, dormindo. A menina segurava a mão do irmão. Ninguém olha pra um vagabundo, mas o vagabundo olha pra todo mundo.")
                        .lie("Não vi nada, patrão. Eu tava bêbado. Eu sempre tô bêbado.", 4)
                        .reveals(&[11]),
                    topic("sandalia", "Por que havia uma sandália de menina no seu casaco?", "Achei na areia, faz meses. Guardo coisas. Tenho uma coleção. Olha bem, patrão: é velha, o couro tá gasto de anos. Não é da menina.").req(Req::Clue(4)),
                ]),
            person("Walter", "Crane", false, 49, Job::Detective, Role::Contact, B(BKind::House, 10), "Inspetor da polícia de Adelaide. Pressionado pelos jornais, quer um culpado até o fim da semana.")
                .temper(35, 60, 50, 45)
                .topics(vec![
                    topic("caso", "O que a polícia descobriu?", "Nada que preste. Toalhas na areia, uma loja, uma nota de uma libra. Os jornais estão acampados na minha porta. Tem um vagabundo, Dunn, que dorme debaixo do píer e foi visto com uma sandália de criança. Para mim, basta.").reveals(&[12]),
                    topic("nota", "Dá para rastrear a nota de uma libra?", "O número de série, talvez. O banco guarda os lotes que entrega nas folhas de pagamento. Mas quem paga um salário com nota nova?").req(Req::Clue(1)),
                ]),
        ],
        clues: vec![
            clue("Toalhas na areia", ClueKind::Physical, Some(Woods), Look3d::Object, "Três toalhas pequenas, dobradas com capricho. Ao lado, o baldinho vermelho de Tommy. E uma toalha grande, de adulto, bordada com um brasão escolar.", &[2])
                .forensic("O brasão é da Escola Primária de St. Leonards. Fios de cabelo loiro, fino e comprido, presos no bordado."),
            clue("A nota de uma libra", ClueKind::Physical, Some(WorkOf(3)), Look3d::Paper, "A nota que Nora usou na loja, guardada separada na gaveta do caixa porque Doris achou estranha.", &[2])
                .forensic("Nota nova, sem dobras. O número de série é de um lote recém-saído do Banco Comercial."),
            clue("Livro de pagamentos do banco", ClueKind::Document, Some(B(BKind::Bank, 0)), Look3d::Paper, "O lote de notas novas da série encontrada na loja foi entregue ao Departamento de Educação — folha de pagamento dos professores de St. Leonards.", &[2])
                .hidden(1)
                .network(Job::Banker),
            clue("Fotografia da praia", ClueKind::Document, Some(WorkOf(4)), Look3d::Photo, "Uma cópia revelada ontem: três crianças e um homem alto e loiro, de calção azul, secando o menorzinho com uma toalha grande.", &[2])
                .forensic("Ampliada, a toalha mostra o mesmo brasão escolar da toalha esquecida na areia. O relógio do pavilhão, ao fundo, marca 3:17 — em plena manhã."),
            clue("Sandália no casaco do vagabundo", ClueKind::Physical, Some(HomeOf(5)), Look3d::Object, "Uma sandália de menina escondida no forro do casaco de Cyril Dunn.", &[5])
                .forensic("O couro está gasto de anos e o número é menor que o de Nora. A sandália não é dela.")
                .herring(),
            clue("Chave da fazenda", ClueKind::Physical, Some(HomeOf(2)), Look3d::Object, "Uma chave de ferro com etiqueta de papel: 'Fazenda Ashby — porão'. Está brilhando de uso recente.", &[2])
                .hidden(1),
            clue("O quarto de brinquedos", ClueKind::Physical, Some(B(BKind::Farmhouse, 1)), Look3d::Object, "No porão da fazenda, um quarto de brinquedos dos anos 30. Três pratos com tortas pela metade, um baldinho de areia, riscos na parede contando dias.", &[2])
                .hidden(2)
                .forensic("As tortas são da loja de Doris Keel. Os riscos na parede estão em grupos de três. Há areia fresca da Praia no chão — e marcas pequenas de pés descalços."),
            testimony("Oito xelins e seis pence", "Margaret deu a Nora oito xelins e seis pence. Nenhuma nota de uma libra.", &[]),
            testimony("A nota que ela não tinha", "Margaret nunca deu uma libra a Nora. Alguém que Nora conhecia deu a nota — ela não aceitava nada de estranhos.", &[2]),
            testimony("Tortas para quatro", "Doris: Nora pagou com nota de uma libra e pediu tortas para quatro, 'para o nosso amigo'.", &[2]),
            testimony("O homem de calção azul", "Stan fotografou as crianças com um homem alto, magro, loiro, de calção azul. Os quatro foram para o vestiário.", &[2]),
            testimony("Seguiam o homem alto", "Cyril viu as crianças seguindo um homem alto até a rua de trás, o menor no colo dele.", &[2]),
            testimony("O vagabundo do píer", "O inspetor Crane quer acusar Cyril Dunn, o vagabundo do píer, por causa de uma sandália.", &[5]),
            clue("Eco: o vestiário da praia", ClueKind::Temporal, Some(Woods), Look3d::None, "No eco, um homem alto se ajoelha diante de três crianças. 'Vamos ver o quarto de brinquedos? Sua mãe deixou.' A menina hesita. O menor já está no colo dele.", &[2]),
            clue("O relógio do pavilhão", ClueKind::Physical, Some(Woods), Look3d::Glow, "O relógio do pavilhão da praia está parado às 3:17. O zelador jura que ele marcou essa hora por três horas seguidas naquela manhã, e depois voltou a andar sozinho.", &[])
                .hidden(3),
            clue("Eco: o porão", ClueKind::Temporal, Some(B(BKind::Farmhouse, 1)), Look3d::None, "No eco, três crianças sentadas no chão de um porão. A menina desenha uma bola escura com pontos vermelhos. 'Quando o moço de casaco vier, a gente volta pra casa', ela diz aos irmãos.", &[2]),
        ],
        echoes: vec![
            echo(Woods, Ghost::Walk, &["Sol forte, gritos de banhistas.", "Um homem alto, loiro, de calção azul.", "'Vamos ver o quarto de brinquedos?'", "Três sombras pequenas atrás de uma grande."], 13),
            echo(B(BKind::Farmhouse, 1), Ghost::Hide, &["Escuro. Cheiro de terra.", "Uma menina conta até três e começa de novo.", "'Ele disse que agora a gente é irmão dele.'", "Um risco na parede."], 15),
        ],
        culprit: 2,
        methods: &[
            "O vagabundo do píer os levou para debaixo do cais",
            "Atraiu as crianças com a nota e as tortas, levou-as de carro ao porão da fazenda da mãe",
            "Afogaram-se no mar e a correnteza levou os corpos",
            "O pai as levou para outro estado",
        ],
        method: 1,
        motives: &[
            "Resgate",
            "Substituir os irmãos que ele deixou afogar em 1936 — 'alguém tem que cuidar delas'",
            "Vingança contra Margaret Whitlock",
            "Dívida do pai das crianças",
        ],
        motive: 1,
        anomalies: &[
            "Nenhuma — é apenas um crime",
            "O relógio da praia marcou 3:17 por três horas em plena manhã — e Nora desenhava a esfera e esperava 'o moço de casaco'",
            "As crianças eram adultas vindas do futuro",
            "A fazenda Ashby não existe nos mapas",
        ],
        anomaly: 1,
        threads: &[(2, 0), (1, 0), (3, 2), (4, 2), (6, 5)],
        story: "Gerald Ashby viu os irmãos morrerem afogados na Praia em 1936 e nunca se perdoou. Durante meses, ensinou natação às crianças Whitlock para ganhar a confiança delas. No feriado, deu a Nora uma nota de uma libra do seu salário de professor, pediu tortas 'para o nosso amigo' e as levou ao quarto de brinquedos no porão da fazenda da mãe, onde pretendia criá-las como os irmãos que perdeu.",
        sphere: "Nora desenhou a esfera antes de sumir. Ela esperava 'o moço de casaco' — você, ou o outro você. Onde as crianças somem, a esfera abre uma fresta: o relógio da praia parou às 3:17 porque, por três horas, aquela manhã pertenceu a duas linhas do tempo ao mesmo tempo.",
        on_true: outcome(
            "A polícia arrombou o porão da fazenda Ashby ao amanhecer. Nora, Peter e Tommy estavam vivos, com fome e com medo, abraçados num canto. Gerald Ashby foi preso na estação, com uma passagem para Melbourne. Nora perguntou pelo 'moço de casaco'.",
            "ENCONTRADAS VIVAS AS CRIANÇAS WHITLOCK",
            "...o professor Gerald Ashby foi detido na estação de Adelaide esta manhã...",
            vec![(2, "jailed"), (0, "survived"), (1, "grateful"), (5, "survived")],
        )
        .flags(vec!["beach_children_found"]),
        on_false: outcome(
            "Cyril Dunn foi condenado com base numa sandália velha. As crianças Whitlock nunca foram encontradas. Margaret deixou a luz da varanda acesa por quarenta anos.",
            "VAGABUNDO DO PÍER É CONDENADO NO CASO WHITLOCK",
            "...as buscas nas dunas foram encerradas...",
            vec![(5, "jailed"), (2, "criminal"), (1, "hates_elias")],
        )
        .flags(vec!["beach_children_lost"]),
        digit: "3",
        reward: 85,
    }
}

pub fn case13() -> CaseDef {
    CaseDef {
        id: 13,
        title: "O Túnel",
        city: CityId::Berlin,
        year: 1957,
        intro: "Berlim, 1957. Sob a fronteira entre o setor americano e o soviético corre um túnel de quatrocentos e cinquenta metros que oficialmente não existe. Dentro dele, técnicos de fones no ouvido gravam as linhas telefônicas do Exército Vermelho. Esta madrugada, o técnico Karl Brenner foi encontrado morto no posto de escuta, ainda de fones. O relógio da parede parou às 3:17.",
        brief: "Karl Brenner, técnico de telefonia, morreu de fones no ouvido no túnel de espionagem sob a fronteira. Americanos culpam a Stasi. A Stasi não sabe de nada — ou finge não saber.",
        start: B(BKind::Warehouse, 0),
        cast: vec![
            person("Karl", "Brenner", false, 41, Job::FactoryWorker, Role::Victim, B(BKind::Apartment, 4), "Técnico de telefonia dos Correios de Berlim Ocidental, contratado em segredo para o túnel. Tinha ouvido absoluto.").dead(),
            person("Liesel", "Brenner", true, 36, Job::Housewife, Role::Witness, B(BKind::Apartment, 4), "A viúva. Costura para fora num apartamento de Neukölln. Não sabia do túnel — ou diz que não sabia.")
                .temper(70, 45, 70, 35)
                .topics(vec![
                    topic("karl", "Karl andava diferente nos últimos dias?", "Chegou em casa na terça, branco. Disse: 'Ouvi na linha uma voz que eu conheço. Uma voz que não devia estar do outro lado.' Não quis dizer de quem. Karl nunca esquecia uma voz. Nunca.").reveals(&[8]),
                    topic("dinheiro", "De onde vêm os dois mil marcos no envelope?", "...Um americano trouxe na quarta. Disse que era adiantamento por 'um relatório especial'. Karl ia contar a eles de quem era a voz. Ele morreu antes de entregar.")
                        .lie("Economias. Karl guardava um pouco toda semana.", 4)
                        .reveals(&[9]),
                ]),
            person("Julian", "Ashcombe", false, 36, Job::Politician, Role::Suspect, B(BKind::Hotel, 1), "Oficial de ligação britânico do túnel. Educado em Oxford, fala russo fluente, toca piano no bar do hotel.")
                .temper(15, 70, 15, 50)
                .flees()
                .topics(vec![
                    topic("tunel", "O senhor esteve no túnel naquela madrugada?", "Estive, sim. Às duas e quarenta, inspeção de rotina. Brenner estava vivo e mal-humorado. Os técnicos alemães são sempre mal-humorados.")
                        .lie("Nunca desço ao túnel. Não é da minha alçada. Eu cuido de papéis.", 5),
                    topic("solda", "Este ferro de solda estava no seu quarto de hotel.", "Conserto rádios. É um passatempo inglês, como jardinagem e traição... uma piada, detetive. Só uma piada.").req(Req::Clue(7)),
                    topic("fita", "A última gravação de Karl tem uma voz com sotaque de Oxford.", "Metade do serviço secreto britânico estudou em Oxford. Vai prender todos nós?").req(Req::Clue(3)),
                ]),
            person("Frank", "Kowalski", false, 44, Job::Detective, Role::Contact, B(BKind::House, 6), "Chefe da segurança americana do túnel. Fuma sem parar e desconfia de todos, menos dos ingleses.")
                .temper(30, 70, 50, 40)
                .topics(vec![
                    topic("vazamento", "O túnel tem um vazamento?", "Tem. Faz meses que os russos falam na linha como se soubessem que estamos ouvindo. Karl me procurou na quarta: disse que tinha reconhecido uma voz. Paguei dois mil marcos adiantados pelo nome. Nunca recebi.").reveals(&[11]),
                    topic("riegel", "De quem o senhor suspeita?", "Otto Riegel, o dono do bar da esquina do posto. Informante da Stasi, todo mundo sabe. Karl bebia lá. Aposto meu distintivo nele.").reveals(&[12]),
                ]),
            person("Hanne", "Weiss", true, 30, Job::RadioHost, Role::Contact, B(BKind::Apartment, 7), "Locutora da RIAS, a rádio americana de Berlim. Apresenta o programa da madrugada.")
                .temper(25, 80, 75, 25)
                .soul(1)
                .topics(vec![
                    topic("ingles", "Você conhece Julian Ashcombe?", "Deu uma entrevista no meu programa sobre a 'amizade anglo-alemã'. No intervalo, cantarolou uma canção do Exército Vermelho sem perceber. E perguntou o nome do técnico que 'tem ouvido absoluto'. Eu achei estranho que ele soubesse disso.").reveals(&[13]),
                    topic("madrugada", "O que se ouve na rádio às três da manhã?", "Às três e dezessete, toda noite, uma interferência atravessa o transmissor. Parece uma voz de homem, em português, dizendo nomes de cidades. Nova Orleans, Londres, Adelaide... Você fala português, não fala, Elias? Não sei por que eu sei disso."),
                ]),
            person("Otto", "Riegel", false, 52, Job::Bartender, Role::Suspect, B(BKind::Apartment, 2), "Dono de um bar na esquina do posto americano. Informante da Stasi de quinta categoria.")
                .temper(65, 35, 35, 75)
                .flees()
                .topics(vec![
                    topic("stasi", "O senhor trabalha para a Stasi?", "Todo mundo em Berlim trabalha para alguém. Eu conto à Stasi quem bebe demais e à polícia quem bebe de menos. Mas aquele bilhete... é caviar, está bem? Caviar de contrabando. Encomenda do túnel da cervejaria, não do seu túnel.")
                        .lie("Sou um honesto dono de bar. Não sei de Stasi nenhuma.", 6)
                        .reveals(&[15]),
                    topic("karl", "Karl Brenner bebia aqui?", "Uma cerveja por noite. Na última, disse que ia 'dar nome aos bois' e ficar rico. Coitado. Em Berlim, quem dá nome aos bois vira boi."),
                ]),
            person("Günter", "Pohl", false, 29, Job::FactoryWorker, Role::Witness, B(BKind::House, 3), "Colega de turno de Karl no túnel. Jovem, nervoso, fuma o cigarro até queimar os dedos.")
                .temper(80, 30, 60, 40)
                .topics(vec![
                    topic("noite", "Onde você estava quando Karl morreu?", "...Eu estava no turno. Às duas e quarenta o inglês desceu, disse que ia 'verificar o equipamento' e me mandou buscar café na superfície. Demorei vinte minutos. Quando voltei, às três e dezessete, as luzes piscaram e Karl estava caído.")
                        .lie("De folga. Em casa. Não estava lá.", 5)
                        .reveals(&[10]),
                ]),
        ],
        clues: vec![
            clue("O corpo de fones", ClueKind::Physical, Some(B(BKind::Warehouse, 0)), Look3d::Object, "Karl Brenner caído sobre a mesa do posto de escuta, no túnel sob o depósito de Rudow. Ainda de fones. Um cheiro de borracha queimada no ar.", &[])
                .forensic("Queimaduras simétricas nas duas orelhas. Não foi o coração: foi uma descarga elétrica pelos fones."),
            clue("Fones adulterados", ClueKind::Physical, Some(B(BKind::Warehouse, 0)), Look3d::Weapon, "Os fones de Karl têm um fio a mais, fino, ligando o cabo à fiação de 220 volts do posto.", &[2])
                .hidden(1)
                .forensic("A solda é limpa, profissional, feita com liga de chumbo inglesa — do tipo usado pelo exército britânico. Nenhum técnico alemão do túnel usa aquela solda."),
            clue("Relógio do posto de escuta", ClueKind::Physical, Some(B(BKind::Warehouse, 0)), Look3d::Glow, "O relógio da parede do túnel parou às 3:17. O vidro está quente ao toque, como se tivesse passado uma luz forte por dentro.", &[])
                .hidden(3),
            clue("A última gravação", ClueKind::Document, Some(B(BKind::Warehouse, 0)), Look3d::Paper, "O carretel que Karl gravava quando morreu. Transcrição: uma voz em russo, com sotaque inglês, diz a Karlshorst: 'O túnel é conhecido desde o primeiro dia. Continuem alimentando-o.' Assinatura em código: DIAMANT.", &[2])
                .forensic("Depois da voz de DIAMANT, abafada, uma terceira voz em português: 'Laboratório Delta... mil novecentos e setenta e um.' É a sua voz."),
            clue("Envelope com 2.000 DM", ClueKind::Physical, Some(HomeOf(0)), Look3d::Object, "Um envelope pardo escondido na lata de biscoitos da cozinha dos Brenner: dois mil marcos alemães em notas novas.", &[3])
                .forensic("Notas sequenciais, recém-saídas do Banco Central — do tipo usado na folha de pagamento do Exército americano."),
            clue("Escala de plantão", ClueKind::Document, Some(B(BKind::Office, 0)), Look3d::Paper, "A escala do túnel na madrugada do crime: Brenner e Pohl de serviço. Uma linha a caneta: 'Visita de inspeção — J. Ashcombe — 02h40'.", &[2, 6])
                .network(Job::Clerk),
            clue("Bilhete em russo", ClueKind::Document, Some(WorkOf(5)), Look3d::Paper, "Um bilhete em cirílico atrás do balcão do bar de Riegel: 'A encomenda do túnel foi entregue.'", &[5])
                .forensic("O 'túnel' é o de uma cervejaria abandonada de Pankow, usado para contrabando. A 'encomenda' são latas de caviar.")
                .herring(),
            clue("Ferro de solda inglês", ClueKind::Physical, Some(HomeOf(2)), Look3d::Weapon, "Um ferro de solda de campanha, com plugue britânico, na mala do quarto de hotel de Julian Ashcombe.", &[2])
                .hidden(2)
                .forensic("Na ponta, resíduo da mesma liga de chumbo dos fones de Karl. E fiapos do isolamento verde do cabo do posto de escuta."),
            testimony("A voz conhecida", "Liesel: Karl ouviu na linha 'uma voz que não devia estar do outro lado'.", &[2]),
            testimony("O relatório especial", "Um americano pagou dois mil marcos adiantados a Karl pelo nome do dono da voz.", &[3]),
            testimony("O inglês às 2:40", "Günter: Ashcombe desceu às 2h40, mandou-o buscar café e ficou sozinho com Karl. Às 3:17 as luzes piscaram.", &[2]),
            testimony("Há um vazamento", "Kowalski: os russos sabem do túnel. Karl ia revelar de quem era a voz.", &[2]),
            testimony("O Stasi do bar", "Kowalski suspeita de Otto Riegel, informante da Stasi e dono do bar onde Karl bebia.", &[5]),
            testimony("A canção do Exército Vermelho", "Hanne: Ashcombe cantarolou uma canção soviética no intervalo e perguntou pelo técnico de ouvido absoluto.", &[2]),
            clue("Eco: o túnel às 3:17", ClueKind::Temporal, Some(B(BKind::Warehouse, 0)), Look3d::None, "No eco, um homem de terno inglês se debruça sobre a mesa do posto com um ferro de solda. Karl entra, põe os fones. As luzes piscam. 3:17.", &[2]),
            testimony("Caviar, não segredos", "Riegel admite o bilhete: é contrabando de caviar num túnel de cervejaria, nada a ver com o túnel americano.", &[5]),
        ],
        echoes: vec![
            echo(B(BKind::Warehouse, 0), Ghost::Struggle, &["Um corredor de concreto, cabos no teto.", "Um homem de terno solda um fio em silêncio.", "'Vá buscar café, Pohl. Sem pressa.'", "Fones no ouvido. Um estalo azul.", "O relógio: 3:17."], 14),
        ],
        culprit: 2,
        methods: &[
            "Estrangulado por um agente da Stasi que entrou pelo lado oriental",
            "Os fones foram ligados à corrente de 220 volts com um fio soldado às escondidas",
            "Ataque cardíaco natural durante o turno",
            "Cianeto no café trazido da superfície",
        ],
        method: 1,
        motives: &[
            "Dinheiro do contrabando de caviar",
            "Karl reconheceu na linha a voz do agente duplo que entregava o túnel aos soviéticos",
            "Ciúme de Liesel",
            "Vingança da Stasi contra um desertor",
        ],
        motive: 1,
        anomalies: &[
            "Nenhuma — é apenas um crime",
            "A gravação guarda, depois da voz de DIAMANT, a sua própria voz dizendo 'Laboratório Delta, 1971' — e o relógio do posto parou às 3:17",
            "O túnel foi cavado por alguém do século XXI",
            "Karl Brenner está vivo do outro lado do muro",
        ],
        anomaly: 1,
        threads: &[(2, 0), (0, 3), (1, 0), (3, 5), (4, 2), (6, 0)],
        story: "Julian Ashcombe, oficial britânico de ligação, era DIAMANT: um agente soviético que entregou o túnel a Moscou desde o primeiro dia. Karl Brenner, com seu ouvido absoluto, reconheceu a voz dele numa linha de Karlshorst e aceitou dois mil marcos dos americanos para dar o nome. Ashcombe desceu às 2h40, mandou Pohl buscar café e soldou os fones de Karl na corrente de 220 volts.",
        sphere: "Você não estava naquela linha telefônica — ainda. A sua voz foi gravada às 3:17 porque a esfera usa os cabos como usa os relógios: tudo que carrega um sinal carrega também o eco de outras linhas do tempo. Em 1971, alguém vai ouvir essa fita no Laboratório Delta.",
        on_true: outcome(
            "Kowalski prendeu Ashcombe no aeroporto de Tempelhof, com um passaporte falso e uma passagem para Viena. O serviço britânico negou tudo e o trocou por um piloto americano em 1959. Liesel ficou com os dois mil marcos e uma medalha que não pode mostrar a ninguém.",
            "DIPLOMATA BRITÂNICO DETIDO EM TEMPELHOF",
            "...fontes aliadas negam a existência de qualquer túnel sob o setor soviético...",
            vec![(2, "jailed"), (1, "grateful"), (3, "police"), (4, "journalist")],
        )
        .flags(vec!["tunnel_mole_caught"]),
        on_false: outcome(
            "Otto Riegel foi levado por homens de sobretudo e nunca mais abriu o bar. O túnel continuou funcionando mais um ano, gravando exatamente o que Moscou queria que fosse gravado.",
            "INFORMANTE DA STASI PRESO EM NEUKÖLLN",
            "...a RIAS interrompe a programação para um comunicado do setor americano...",
            vec![(5, "jailed"), (2, "criminal")],
        )
        .flags(vec!["tunnel_mole_free"]),
        digit: "1",
        reward: 90,
    }
}

pub fn case14() -> CaseDef {
    CaseDef {
        id: 14,
        title: "A Noite do Arame",
        city: CityId::Berlin,
        year: 1961,
        intro: "Berlim, 13 de agosto de 1961. À meia-noite, caminhões do exército da Alemanha Oriental começaram a desenrolar arame farpado no meio das ruas. Ao amanhecer, a cidade estava cortada ao meio. Às 3:17, no muro do cemitério de Sophien, Dieter Hahn tentou atravessar para o lado ocidental, onde a mulher e a filha dormiam. Um tiro. A versão oficial culpa um guarda de dezenove anos.",
        brief: "Dieter Hahn foi morto no arame farpado do cemitério na noite em que o muro subiu. O guarda Rolf Kessler é apontado como atirador. Mas Rolf diz que hesitou.",
        start: Cemetery,
        cast: vec![
            person("Dieter", "Hahn", false, 33, Job::FactoryWorker, Role::Victim, B(BKind::Apartment, 3), "Torneiro de uma fábrica em Berlim Oriental. A mulher e a filha estavam visitando a sogra no lado ocidental quando o arame subiu.").dead(),
            person("Ilse", "Hahn", true, 30, Job::Tailor, Role::Witness, B(BKind::Apartment, 3), "A viúva. Costureira. Ficou do lado ocidental com a filha de cinco anos e viu o marido morrer do outro lado do arame.")
                .temper(65, 55, 75, 20)
                .topics(vec![
                    topic("ligacao", "Quando a senhora falou com Dieter pela última vez?", "À meia-noite e meia, pelo telefone do bar. As linhas iam ser cortadas. Ele disse: 'Vou atravessar pelo cemitério. E vou levar os papéis do Werner. O mundo tem que saber quem ele é.' Depois a linha caiu.").reveals(&[6]),
                    topic("rota", "Quem mais sabia que ele iria pelo cemitério?", "...Ninguém. Só o meu irmão, Werner. Foi ele quem ensinou o caminho, anos atrás, quando éramos crianças e roubávamos maçãs do outro lado. Meu Deus. Só o Werner sabia.").req(Req::Topic("ligacao")).reveals(&[7]),
                    topic("werner", "Como é o seu irmão?", "Werner acredita no Partido como a nossa mãe acreditava em Deus. Depois da guerra, voltou diferente. Ele diz que protege a família. Eu nunca soube de quem."),
                ]),
            person("Werner", "Lutz", false, 38, Job::Police, Role::Suspect, B(BKind::House, 7), "Irmão de Ilse. Oficial da Volkspolizei, e, segundo os vizinhos, algo mais. Anda com uma pistola sob a capa.")
                .temper(20, 75, 20, 45)
                .flees()
                .topics(vec![
                    topic("noite", "Onde o senhor estava às três da manhã?", "...No cemitério. Fui impedir uma tolice. Dieter ia se matar naquele arame. Eu quis trazê-lo de volta. Quando cheguei, o guarda já tinha atirado. Foi o guarda!")
                        .lie("No quartel, coordenando a Operação Rosa. Duzentos homens me viram.", 2),
                    topic("lista", "Esta lista tem o seu codinome: 'Nachtigall'.", "Rouxinol. Bonito, não é? Eu protegi esta família dezesseis anos. Cada nome que entreguei era alguém que ia arrastar Ilse para a cadeia com ele. Dieter não entendia. Ele queria levar isso ao Ocidente e me enforcar num jornal.").req(Req::Clue(4)),
                    topic("pistola", "A sua Makarov tem uma bala a menos.", "Treino de tiro. Todo oficial treina.").req(Req::Clue(15)),
                ]),
            person("Rolf", "Kessler", false, 19, Job::Police, Role::Witness, B(BKind::Apartment, 6), "Guarda de fronteira recruta, da Saxônia. Três semanas de farda. Chora quando pensa que ninguém vê.")
                .temper(85, 25, 75, 15)
                .topics(vec![
                    topic("tiro", "Você atirou em Dieter Hahn?", "...Não. Eu atirei para cima. Duas vezes, para cima, como quem espanta pássaro. Eu vi o homem no arame e não consegui. Depois veio o outro tiro, baixo, seco, de pistola, lá de trás da capela. Não foi o meu fuzil.")
                        .lie("Eu atirei. Cumpri a ordem. É o que o regulamento manda.", 3)
                        .reveals(&[8]),
                    topic("medo", "Por que você assumiu a culpa?", "Porque um oficial de capa me disse que era melhor ser o herói da fronteira do que o covarde que deixou um traidor fugir. Minha mãe vive em Dresden, senhor. Ele sabia o endereço.").req(Req::Topic("tiro")),
                ]),
            person("Gisela", "Brandt", true, 63, Job::Gravedigger, Role::Witness, B(BKind::House, 2), "Coveira do cemitério de Sophien há trinta anos. Mora na casinha do portão. Viu duas guerras e agora um muro.")
                .temper(30, 65, 70, 15)
                .topics(vec![
                    topic("noite", "A senhora viu alguma coisa na noite do arame?", "Vi um homem de capa escura atrás da capela, parado no meio dos túmulos, esperando. Quando o rapaz subiu no arame, os tiros do guarda foram para o céu — eu vi as faíscas no alto. Depois o homem de capa levantou o braço. Um tiro só.").reveals(&[9]),
                    topic("relogio", "O relógio da capela parou?", "Às três e dezessete. E quando fui dar corda, de manhã, havia um número riscado no vidro: 1989. Não sei quem escreveu. Mortos não escrevem, moço, eu garanto.").req(Req::Topic("noite")),
                ]),
            person("Johannes", "Wendt", false, 58, Job::Priest, Role::Contact, B(BKind::House, 9), "Pastor da igreja ao lado do cemitério. Esconde gente na sacristia e papéis no órgão.")
                .temper(35, 75, 90, 10)
                .topics(vec![
                    topic("papeis", "Dieter lhe deixou alguma coisa?", "Uma cópia. Dieter sabia que podia não chegar ao outro lado. Deixou comigo, dentro do órgão, uma lista com os relatórios de um informante chamado 'Nachtigall'. O nome dele próprio está lá, e o da vizinhança inteira.").reveals(&[10]),
                    topic("sommer", "Dieter pagou alguém para atravessá-lo?", "Kurt Sommer, um atravessador. Cobrou quinhentos marcos ocidentais e sumiu antes da meia-noite. Muita gente pagou ao Sommer naquela semana. Ninguém atravessou.").reveals(&[11]),
                ]),
            person("Kurt", "Sommer", false, 42, Job::Smuggler, Role::Suspect, B(BKind::Apartment, 5), "Atravessador de fronteira. Contrabandeava cigarros e gente. Na noite do arame, sumiu com o dinheiro de todos.")
                .temper(70, 35, 25, 90)
                .flees()
                .topics(vec![
                    topic("dinheiro", "O senhor recebeu quinhentos marcos de Dieter Hahn?", "Recebi, e daí? Quando vi os caminhões, eu corri. Não sou herói. Mas eu vi uma coisa: um carro preto da Stasi parado na rua do cemitério à meia-noite. Um oficial de capa desceu e entrou pelo portão dos fundos. Achei que era comigo.")
                        .lie("Nunca vi esse Hahn. Eu vendo cigarros, só isso.", 5)
                        .reveals(&[12]),
                ]),
        ],
        clues: vec![
            clue("O corpo no arame", ClueKind::Physical, Some(Cemetery), Look3d::Blood, "Dieter Hahn preso ao arame farpado no alto do muro do cemitério. O casaco rasgado no forro, como se alguém tivesse arrancado algo dali.", &[])
                .forensic("Um único ferimento, nas costas, de baixo para cima, disparado a curta distância. Um fuzil da torre, no alto, faria o caminho contrário."),
            clue("Projétil de 9 mm", ClueKind::Physical, Some(Cemetery), Look3d::Weapon, "Um projétil achatado cravado numa lápide, atrás da capela.", &[2])
                .hidden(1)
                .forensic("Calibre 9x18 Makarov. Os guardas de fronteira usam fuzis 7,62. Pistolas Makarov são de oficiais."),
            clue("Registro do arsenal", ClueKind::Document, Some(B(BKind::Police, 0)), Look3d::Paper, "O livro de armas da noite de 13 de agosto. Rolf Kessler devolveu o fuzil com dois cartuchos a menos. W. Lutz retirou uma Makarov às 23h50 e deixou o quartel às 0h15.", &[3, 2])
                .network(Job::Police),
            clue("Cápsulas no cemitério", ClueKind::Physical, Some(Cemetery), Look3d::Object, "Duas cápsulas de fuzil ao pé da torre de vigia. Uma cápsula de pistola atrás da capela, entre os túmulos.", &[3, 2])
                .hidden(2)
                .forensic("As marcas de ranhura no muro da torre mostram que os dois tiros de fuzil foram para o alto, quase na vertical."),
            clue("A lista de 'Nachtigall'", ClueKind::Document, Some(B(BKind::Church, 0)), Look3d::Paper, "Escondida no órgão da igreja: cópias de relatórios de um informante de codinome 'Nachtigall' sobre a própria família e a vizinhança. Assinados com a caligrafia de Werner Lutz.", &[2]),
            clue("Recibo de Kurt Sommer", ClueKind::Document, Some(HomeOf(6)), Look3d::Paper, "Um caderno de Kurt Sommer: 'Hahn — 500 DM ocidentais — cemitério — 3h'. Parece a prova de que o atravessador o entregou.", &[6])
                .forensic("A lista tem vinte nomes, todos com a mesma hora. Sommer vendeu a mesma travessia a vinte pessoas. Um golpista, não um assassino.")
                .herring(),
            testimony("A última ligação", "Ilse: Dieter ia atravessar pelo cemitério levando 'os papéis do Werner'.", &[2]),
            testimony("Só Werner sabia a rota", "Ilse: o caminho pelo cemitério só Werner conhecia — foi ele quem o ensinou.", &[2]),
            testimony("Tiros para o alto", "Rolf atirou duas vezes para cima. O tiro que matou veio de uma pistola, atrás da capela.", &[3, 2]),
            testimony("O homem de capa atrás da capela", "Gisela viu um homem de capa escura atrás da capela. Os tiros do guarda foram para o céu; o de capa disparou uma vez.", &[2]),
            testimony("Os papéis no órgão", "O pastor guarda a cópia da lista de 'Nachtigall' que Dieter lhe deixou.", &[2]),
            testimony("Sommer fugiu com o dinheiro", "O pastor: o atravessador Kurt Sommer cobrou 500 marcos de Dieter e sumiu.", &[6]),
            testimony("O carro preto", "Sommer viu um carro da Stasi na rua do cemitério à meia-noite; um oficial de capa entrou pelo portão dos fundos.", &[2]),
            clue("Eco: o cemitério às 3:17", ClueKind::Temporal, Some(Cemetery), Look3d::None, "No eco, um homem sobe no arame. Da torre, dois clarões para o céu. Atrás da capela, um homem de capa levanta o braço. 3:17.", &[2]),
            clue("Relógio da capela", ClueKind::Physical, Some(B(BKind::Church, 0)), Look3d::Glow, "O relógio da capela do cemitério parou às 3:17. No vidro, riscado com a ponta de um prego: '1989'. Abaixo, menor: 'E.V. esteve aqui'.", &[])
                .hidden(3),
            clue("Pistola Makarov", ClueKind::Physical, Some(HomeOf(2)), Look3d::Weapon, "Uma pistola Makarov no fundo falso de uma gaveta na casa de Werner Lutz. Um cartucho a menos no carregador.", &[2])
                .hidden(2)
                .forensic("Disparada recentemente. As ranhuras do cano batem com o projétil cravado na lápide."),
        ],
        echoes: vec![
            echo(Cemetery, Ghost::Flee, &["Arame farpado brilhando ao luar.", "Um homem sobe, o casaco se rasga.", "Dois clarões para o céu: o guarda hesitou.", "Atrás da capela, um braço se levanta.", "O relógio da capela: 3:17."], 13),
        ],
        culprit: 2,
        methods: &[
            "Rajada do fuzil do guarda Rolf Kessler, do alto da torre",
            "Um tiro de pistola Makarov pelas costas, disparado de trás da capela",
            "Caiu do muro e se feriu no arame",
            "O atravessador Sommer o entregou e o matou na travessia",
        ],
        method: 1,
        motives: &[
            "Cumprimento da ordem de atirar em quem cruzasse a fronteira",
            "Impedir que Dieter levasse ao Ocidente as provas de que Werner era o informante 'Nachtigall'",
            "Roubar os quinhentos marcos da travessia",
            "Ciúme de uma mulher do lado ocidental",
        ],
        motive: 1,
        anomalies: &[
            "Nenhuma — é apenas um crime",
            "O relógio da capela parou às 3:17 com '1989' riscado no vidro — o ano em que o muro vai cair — e as suas iniciais embaixo",
            "O arame farpado estava enferrujado como se tivesse vinte anos",
            "Dieter Hahn atravessou e está vivo em Berlim Ocidental",
        ],
        anomaly: 1,
        threads: &[(2, 0), (1, 0), (1, 2), (3, 2), (6, 0), (5, 0)],
        story: "Werner Lutz, irmão de Ilse, era o informante 'Nachtigall' e denunciava a própria família havia anos. Na noite do arame, Dieter ia atravessar pelo cemitério levando os relatórios de Werner para o Ocidente. Só Werner conhecia a rota. Ele retirou uma Makarov do quartel, esperou atrás da capela e, quando o guarda Rolf hesitou e atirou para o alto, disparou uma vez pelas costas. Depois obrigou o recruta a assumir a morte como 'herói da fronteira'.",
        sphere: "O muro vai durar vinte e oito anos. Alguém — você — já sabia disso e riscou o ano no vidro do relógio. A esfera marca as noites em que uma cidade se parte: nessas noites, as linhas do tempo também se partem, e às 3:17 as duas metades se olham por cima do arame.",
        on_true: outcome(
            "O pastor Wendt levou a lista de 'Nachtigall' a um jornal ocidental. Werner Lutz foi preso — pelos próprios camaradas, que não perdoam um informante exposto. Rolf Kessler desertou pelo mesmo cemitério três meses depois. Ilse criou a filha em Wedding, olhando o muro da janela.",
            "OFICIAL DA VOPO MATOU O CUNHADO NO MURO, REVELA LISTA SECRETA",
            "...a RIAS lê esta noite os nomes da lista de 'Nachtigall'...",
            vec![(2, "jailed"), (3, "left_city"), (1, "grateful"), (5, "journalist")],
        )
        .flags(vec!["wire_truth"]),
        on_false: outcome(
            "Rolf Kessler foi condecorado como herói da fronteira e nunca mais dormiu uma noite inteira. Werner Lutz foi promovido. A lista de 'Nachtigall' apodreceu dentro do órgão da igreja.",
            "GUARDA DE FRONTEIRA IMPEDE 'FUGA CRIMINOSA' NO CEMITÉRIO",
            "...o Neues Deutschland elogia a firmeza dos guardas da fronteira...",
            vec![(3, "hates_elias"), (2, "criminal"), (1, "left_city")],
        )
        .flags(vec!["wire_lie"]),
        digit: "7",
        reward: 95,
    }
}

pub fn case15() -> CaseDef {
    CaseDef {
        id: 15,
        title: "O Farol de Nordnes",
        city: CityId::Bergen,
        year: 1964,
        intro: "Bergen, 1964. Chove há quarenta dias. Anders Holm, guardião do farol da ponta de Nordnes, foi encontrado morto no próprio quarto, sentado na poltrona. O aquecedor a querosene estava no máximo; o termômetro da parede marcava trinta graus. E o corpo de Anders estava congelado: gelo nos cílios, a pele azul, os dedos duros como bacalhau seco.",
        brief: "O faroleiro Anders Holm morreu congelado num quarto aquecido a 30 °C. No porto, contrabandistas de café e aguardente fazem sinais no escuro.",
        start: HomeOf(0),
        cast: vec![
            person("Anders", "Holm", false, 58, Job::Retired, Role::Victim, B(BKind::House, 3), "Guardião do farol de Nordnes havia vinte e dois anos. Viúvo, metódico, anotava cada noite no diário do farol.").dead(),
            person("Ragnhild", "Berg", true, 44, Job::Vendor, Role::Suspect, B(BKind::House, 7), "Viúva de pescador. Vende peixe no mercado do cais. O marido morreu em 1962, numa noite em que o farol de Nordnes ficou apagado.")
                .temper(40, 70, 70, 25)
                .topics(vec![
                    topic("kari", "Como o seu marido morreu?", "Kari saiu com o barco numa noite de tempestade, em novembro de 1962. O farol de Nordnes estava apagado. O barco bateu nas pedras do Puddefjord. Eu cuspi na porta do Anders no enterro. Toda Bergen viu.").reveals(&[8]),
                    topic("anders", "A senhora falou com Anders recentemente?", "...Ele me escreveu. Disse que não foi ele quem apagou a luz naquela noite, que sabia quem foi, e que ia à polícia na segunda-feira. Eu não respondi. Morreu no domingo.")
                        .lie("Não falo com aquele homem há dois anos. Nem quero.", 4)
                        .reveals(&[9]),
                    topic("faca", "Havia uma faca suja de sangue na sua cozinha.", "Sangue de bacalhau, detetive. Eu limpo quarenta peixes por dia. Quer que eu limpe um na sua frente?").req(Req::Clue(7)),
                ]),
            person("Sverre", "Dahl", false, 46, Job::Merchant, Role::Suspect, B(BKind::Mansion, 1), "Dono do maior armazém frigorífico de Nordnes. Exporta arenque e bacalhau. Todo mundo sabe que importa outras coisas.")
                .temper(15, 75, 15, 90)
                .flees()
                .topics(vec![
                    topic("noite", "Onde o senhor estava na madrugada de domingo?", "...No armazém. Tinha carga chegando. Anders apareceu gritando, ameaçando. Eu mandei ele embora. Se ele se trancou no frigorífico depois disso, foi escolha dele.")
                        .lie("Em casa, com a minha esposa, ouvindo a rádio. Durmo cedo.", 10),
                    topic("armazem", "O que fazem quarenta sacos de café brasileiro sem selo no seu armazém?", "Café de Santos, o melhor do mundo. O governo cobra mais imposto do que o grão vale. Eu só faço os noruegueses beberem um café decente. Isso não é crime, é caridade.").req(Req::Clue(5)),
                    topic("farol", "O diário do farol fala de noites 'em manutenção'.", "Um farol velho quebra. O Anders era velho também.").req(Req::Clue(3)),
                ]),
            person("Leif", "Nygaard", false, 27, Job::Dockworker, Role::Witness, B(BKind::Apartment, 4), "Estivador do armazém de Dahl. Grande, calado, com medo do patrão e mais medo ainda de perder o emprego.")
                .temper(80, 35, 60, 45)
                .topics(vec![
                    topic("noite", "O que aconteceu no armazém naquela noite?", "O patrão mandou todo mundo embora às dez. Eu voltei à uma, tinha esquecido o casaco. Ouvi batidas dentro do frigorífico. Batidas de mão. Às três e dezessete o caminhão do patrão saiu com um tapete enrolado na caçamba. Eu não perguntei. Tenho dois filhos.")
                        .lie("Ninguém entra no frigorífico de noite. Eu saí às dez e fui dormir.", 6)
                        .reveals(&[10]),
                ]),
            person("Ingrid", "Solheim", true, 32, Job::Journalist, Role::Contact, B(BKind::Apartment, 2), "Repórter do jornal de Bergen. Investiga o contrabando no porto há um ano; o editor corta todas as matérias dela.")
                .temper(20, 85, 80, 25)
                .soul(1)
                .topics(vec![
                    topic("contrabando", "O que você sabe do contrabando em Nordnes?", "Café e aquavit sueco entram pelos barcos de pesca, nas noites em que o farol 'quebra'. Dahl paga a polícia em coroas e em garrafas. Meu editor bebe o aquavit dele. Por isso nada sai no jornal.").reveals(&[12]),
                    topic("voce", "Por que você me ajuda, Ingrid?", "Porque você tem cara de quem já me conheceu. Eu sonho com uma cidade quente onde tocam trompete a noite toda, e com uma praia onde um homem morre com um livro. Acordo às três e dezessete com gosto de sal. Isso faz sentido para você?"),
                ]),
            person("Halvor", "Rasmussen", false, 61, Job::Doctor, Role::Contact, B(BKind::House, 6), "Médico-legista do hospital de Haukeland. Fuma cachimbo e fala com os mortos em voz baixa.")
                .temper(30, 60, 80, 20)
                .topics(vec![
                    topic("autopsia", "Como um homem congela num quarto a trinta graus?", "Não congela. Anders morreu a vinte graus negativos, durante horas, e foi colocado depois na poltrona. O aquecedor no máximo era para confundir a hora da morte. Há cristais de gelo nos tecidos. Nenhuma noite de Bergen chega a vinte negativos, detetive. Nenhuma noite — só uma câmara frigorífica.").reveals(&[11]),
                ]),
            person("Olav", "Brekke", false, 50, Job::Police, Role::Suspect, B(BKind::House, 9), "Guarda da polícia do porto. Faz a ronda de Nordnes. Tem o nariz vermelho de quem não bebe só água.")
                .temper(60, 35, 30, 70)
                .topics(vec![
                    topic("ronda", "O senhor fez a ronda de Nordnes naquela madrugada?", "...Não fiz. Dahl me deu quinhentas coroas para eu não passar pela ponta naquela noite. Todo mês é assim, nas noites de carga. Eu não sabia que iam matar o velho. Juro pela minha mãe.")
                        .lie("Fiz a ronda normal, de hora em hora. Não vi nada de estranho.", 12)
                        .reveals(&[13]),
                ]),
        ],
        clues: vec![
            clue("O corpo congelado", ClueKind::Physical, Some(HomeOf(0)), Look3d::Object, "Anders Holm sentado na poltrona, de casaco de lã. Gelo nos cílios e na barba. Os dedos duros. O quarto é um forno.", &[])
                .forensic("A roupa está úmida por fora e seca por dentro: o corpo descongelou devagar depois de colocado no quarto. As costas têm marcas de um tapete de sisal."),
            clue("Aquecedor no máximo", ClueKind::Physical, Some(HomeOf(0)), Look3d::Object, "Um aquecedor a querosene aberto no máximo. O tanque está quase vazio.", &[])
                .hidden(1)
                .forensic("Pelo consumo, foi aceso por volta das 3:17 — muito depois da hora da morte que o calor queria sugerir."),
            clue("Escamas de arenque", ClueKind::Physical, Some(HomeOf(0)), Look3d::Object, "Escamas prateadas de arenque grudadas na gola e nas solas de Anders. Ele não pescava havia anos.", &[2])
                .hidden(1)
                .forensic("Escamas cobertas de uma geada fina, típica de câmara a vinte graus negativos. Só um armazém em Nordnes chega a essa temperatura: o de Sverre Dahl."),
            clue("Diário do farol", ClueKind::Document, Some(Docks), Look3d::Paper, "O diário do farol de Nordnes. Noites marcadas 'em manutenção', uma por mês, desde 1960. A última anotação, na letra de Anders: 'D. apagou a luz de novo. Como na noite do Kari. Vou à polícia na segunda.'", &[2]),
            clue("Carta a Ragnhild", ClueKind::Document, Some(HomeOf(1)), Look3d::Paper, "Uma carta de Anders para Ragnhild Berg, amassada e alisada de novo: 'Não fui eu que apaguei a luz na noite do Kari. Sei quem foi. Segunda-feira todos vão saber.'", &[2, 1]),
            clue("Café e aquavit sem selo", ClueKind::Physical, Some(B(BKind::Warehouse, 0)), Look3d::Object, "Atrás das caixas de arenque do armazém de Dahl: quarenta sacos de café de Santos e caixas de aquavit sueco, sem nenhum selo da alfândega.", &[2])
                .network(Job::Smuggler),
            clue("A câmara frigorífica", ClueKind::Physical, Some(B(BKind::Warehouse, 0)), Look3d::Prints, "Na face interna da porta da câmara frigorífica, arranhões de unhas. No chão congelado, um botão de chifre de um casaco de lã.", &[2])
                .hidden(2)
                .forensic("O botão é do casaco de Anders — falta um, o terceiro. As marcas de unha estão na altura de um homem sentado, já sem forças."),
            clue("Faca de pesca ensanguentada", ClueKind::Physical, Some(HomeOf(1)), Look3d::Weapon, "Uma faca de limpar peixe com sangue seco, na cozinha de Ragnhild Berg — a mulher que cuspiu na porta do faroleiro.", &[1])
                .forensic("Sangue de peixe. E Anders não tem nenhum ferimento de faca.")
                .herring(),
            testimony("A noite em que Kari morreu", "Ragnhild: o marido morreu em 1962 numa noite em que o farol de Nordnes estava apagado.", &[]),
            testimony("Anders ia à polícia", "Anders escreveu a Ragnhild: sabia quem apagava a luz e ia à polícia na segunda-feira. Morreu no domingo.", &[2]),
            testimony("O caminhão das 3:17", "Leif ouviu batidas de mão dentro do frigorífico; às 3:17 o caminhão de Dahl saiu com um tapete enrolado.", &[2]),
            testimony("Vinte graus negativos", "O legista: Anders morreu a vinte graus negativos, durante horas, e foi posto no quarto depois. Só uma câmara frigorífica explica.", &[]),
            testimony("Coroas e aquavit", "Ingrid: Dahl paga a polícia em coroas e garrafas nas noites em que o farol 'quebra'.", &[2, 6]),
            testimony("Quinhentas coroas", "Brekke recebeu 500 coroas de Dahl para não fazer a ronda de Nordnes naquela noite.", &[2]),
            clue("Eco: a câmara fria", ClueKind::Temporal, Some(B(BKind::Warehouse, 0)), Look3d::None, "No eco, dois homens discutem entre caixas de arenque. Um empurra o outro para dentro da câmara e fecha a porta pesada. Batidas. Depois, silêncio.", &[2]),
            clue("A lente do farol", ClueKind::Physical, Some(Docks), Look3d::Glow, "Na grande lente de Fresnel do farol, gravado de dentro para fora no vidro: um círculo com pontos vermelhos e os números 3:17. Refletida na lente, por um instante, você vê uma sala branca cheia de aparelhos.", &[])
                .hidden(3),
        ],
        echoes: vec![
            echo(B(BKind::Warehouse, 0), Ghost::Struggle, &["Cheiro de sal e arenque.", "'Segunda-feira, Dahl. Toda Bergen vai saber.'", "Uma porta de aço se fecha.", "Mãos batendo no escuro gelado.", "Um relógio de parede: 3:17."], 14),
        ],
        culprit: 2,
        methods: &[
            "Envenenado com aquavit adulterado",
            "Trancado na câmara frigorífica do armazém e levado de volta ao quarto aquecido para confundir a hora da morte",
            "Morreu de frio no mar e se arrastou sozinho até em casa",
            "Esfaqueado pela viúva do pescador",
        ],
        method: 1,
        motives: &[
            "Vingança pela morte do marido pescador",
            "Anders ia denunciar o contrabando e as noites em que o farol era apagado — inclusive a que matou Kari Berg",
            "Herança da casa do farol",
            "Dívida de jogo no porto",
        ],
        motive: 1,
        anomalies: &[
            "Nenhuma — é apenas um crime",
            "A lente do farol, fabricada em 1880, traz gravados a esfera e 3:17 — e reflete uma sala de laboratório que ainda não existe",
            "O gelo no corpo não derrete nunca",
            "Anders Holm morreu duas vezes, em anos diferentes",
        ],
        anomaly: 1,
        threads: &[(2, 0), (1, 0), (2, 6), (3, 2), (4, 2), (5, 0)],
        story: "Sverre Dahl usava as noites em que o farol 'quebrava' para desembarcar café e aquavit sem imposto. Numa dessas noites, em 1962, o barco de Kari Berg bateu nas pedras. Anders descobriu que era Dahl quem apagava a luz e prometeu ir à polícia na segunda. No domingo, foi tirar satisfação no armazém; Dahl o trancou na câmara frigorífica, esperou que morresse e, às 3:17, levou o corpo enrolado num tapete de volta ao quarto, com o aquecedor no máximo, para parecer uma morte natural de velho sozinho.",
        sphere: "Todo farol é um relógio de luz: acende e apaga no mesmo ritmo, noite após noite. A esfera gosta de relógios. Quando alguém apaga um farol, a linha do tempo fica no escuro por um instante — e às 3:17 a lente de Nordnes mostrou a você o lugar onde ela vai nascer: uma sala branca, 1971, Laboratório Delta.",
        on_true: outcome(
            "Sverre Dahl foi preso com o caminhão ainda cheirando a querosene. O guarda Brekke perdeu a farda. Ingrid publicou a matéria que o editor cortou por um ano. Ragnhild Berg acendeu uma vela no farol de Nordnes — que nunca mais apagou.",
            "EXPORTADOR DE NORDNES PRESO PELA MORTE DO FAROLEIRO",
            "...a alfândega de Bergen apreende quarenta sacos de café brasileiro...",
            vec![(2, "jailed"), (6, "jailed"), (4, "journalist"), (1, "grateful")],
        )
        .close(vec![BKind::Warehouse])
        .flags(vec!["lighthouse_true"]),
        on_false: outcome(
            "Ragnhild Berg foi acusada — todos a viram cuspir na porta do faroleiro. O armazém de Dahl continuou recebendo barcos nas noites de 'manutenção'. No inverno seguinte, outro barco bateu nas pedras.",
            "VIÚVA DE PESCADOR ACUSADA DE MATAR O FAROLEIRO",
            "...o farol de Nordnes ficará apagado por reparos esta noite...",
            vec![(1, "jailed"), (2, "criminal"), (4, "hates_elias")],
        )
        .flags(vec!["lighthouse_false"]),
        digit: "2",
        reward: 95,
    }
}
