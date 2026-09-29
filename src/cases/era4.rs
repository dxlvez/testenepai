//! ERA IV — 1969–1979 — Bergen, São Francisco, Portland.

use super::defs::*;
use crate::city::gen::CityId;
use crate::city::map::BKind;
use crate::sim::people::Job;
use Place::*;

pub fn cases() -> Vec<CaseDef> {
    vec![case16(), case17(), case18(), case19(), case20()]
}

// ------------------------------------------------------------------ 16 — Bergen, 1970

pub fn case16() -> CaseDef {
    CaseDef {
        id: 16,
        title: "A Mulher do Vale do Gelo",
        city: CityId::Bergen,
        year: 1970,
        intro: "Bergen, 1970. Um professor que caminhava com as filhas encontrou, no fundo do Vale do Gelo, o corpo de uma mulher queimado até ficar irreconhecível. Todas as etiquetas das roupas tinham sido arrancadas. Na estação, duas malas esperavam por ela no guarda-volumes: perucas, óculos sem grau, dinheiro de seis países e um caderno escrito em código. Ninguém em Bergen sabe o nome dela. Ela usou oito.",
        brief: "Uma mulher sem nome foi encontrada queimada no Vale do Gelo. Etiquetas arrancadas, malas na estação, perucas, passaportes e um caderno cifrado. A polícia fala em suicídio.",
        start: Woods,
        cast: vec![
            person("Vera", "Moreau", true, 38, Job::None, Role::Victim, B(BKind::Hotel, 1), "A mulher do vale. 'Vera Moreau' é só um dos oito nomes que ela usou em onze meses. Falava quatro línguas e nunca dormia duas noites no mesmo quarto.").dead(),
            person("Halvard", "Brekke", false, 47, Job::Scientist, Role::Suspect, B(BKind::House, 4), "Engenheiro de acústica da estação naval de testes. Educado, pontual, sempre de luvas. Dirige o carro de serviço da Marinha.")
                .temper(40, 55, 25, 75)
                .flees()
                .topics(vec![
                    topic("mulher", "O senhor conhecia a mulher encontrada no vale?", "...Conhecia. Ela me procurou em março, dizendo que era fotógrafa de uma revista belga. Queria saber dos testes de foguetes na baía. Eu vendi o que sabia — horários, posições, umas fotos. Ninguém morre por causa de umas fotos de mar.")
                        .lie("Nunca a vi. Eu trabalho com hidrofones, não com turistas.", 6),
                    topic("noite", "Uma testemunha viu um homem subir o vale com ela na véspera. Era o senhor?", "Eu a levei até a trilha, sim. Ela queria sair do país pelas montanhas, longe dos aeroportos. Deixei-a viva, com uma garrafa térmica de chá e o dinheiro. O que aconteceu depois não é comigo.")
                        .req(Req::Clue(10)),
                    topic("remedios", "O Fenemal da receita da sua esposa. Quem o retirou na farmácia?", "Eu retiro toda semana. Randi não dorme desde o inverno passado. Isso não prova nada, senhor Vale. Metade de Bergen toma Fenemal para aguentar a escuridão.")
                        .req(Req::Clue(8)),
                    topic("codigo", "O caderno dela tem as suas iniciais e a data da morte.", "Ela anotava tudo. Era o trabalho dela. E o meu erro. Se ela tivesse chegado a Bruxelas com aquele caderno, eu seria enforcado como traidor pelos dois lados.")
                        .req(Req::Clue(3)),
                ]),
            person("Solveig", "Haugland", true, 34, Job::HotelClerk, Role::Contact, B(BKind::Apartment, 2), "Recepcionista do Hotel Bryggen. Guarda na memória cada hóspede que passou pelo balcão. Olha para Elias como quem tenta lembrar uma música.")
                .temper(35, 65, 80, 20)
                .soul(1)
                .topics(vec![
                    topic("hospede", "O que a senhora lembra da hóspede do quarto 407?", "Ela trocou de quarto três vezes em seis dias. Sempre pedia janela para o porto. Recebeu um homem de luvas, que usava um anel de formatura da escola técnica. No dia 23 ela pagou tudo em dinheiro, pediu a conta e disse: 'Se alguém perguntar, eu nunca estive aqui.'")
                        .reveals(&[9]),
                    topic("nomes", "Ela se registrava com nomes diferentes?", "Genevieve Lancier, de Bruxelas. Claudia Nielsen, de Paris. Vera Moreau, de Genebra. A caligrafia era sempre a mesma. E ela cantarolava a mesma música no elevador, em todas as vezes.")
                        .req(Req::Clue(4)),
                    topic("voce", "Por que a senhora me olha desse jeito?", "Porque eu já vi o senhor. Não aqui. Num lugar quente, com um ventilador de teto e uma máquina de escrever... Bobagem. Eu nunca saí da Noruega. Mas eu sonho com incêndios desde menina, e em todos o senhor chega tarde."),
                ]),
            person("Arne", "Tveit", false, 44, Job::Teacher, Role::Witness, B(BKind::House, 7), "Professor de geografia. Encontrou o corpo quando caminhava com as filhas pelo vale. Não dorme desde então.")
                .temper(60, 50, 85, 15)
                .topics(vec![
                    topic("corpo", "Como o senhor encontrou o corpo?", "O cheiro, primeiro. Depois a fumaça que ainda saía das pedras. Ela estava de costas, com os braços erguidos, como um boxeador. Em volta, um relógio derretido, duas garrafas de plástico e uma garrafa térmica. Tirei as meninas dali correndo."),
                    topic("trilha", "O senhor esteve no vale na véspera?", "Estive, com a minha turma. Cruzamos com um casal subindo a trilha. Ela, de casaco claro e peruca escura. Ele, de sobretudo cinza e luvas, carregando uma garrafa térmica. Ele desviou o rosto. Ela sorriu para as crianças.")
                        .reveals(&[10]),
                ]),
            person("Luca", "Ferri", false, 38, Job::Photographer, Role::Suspect, B(BKind::Hotel, 3), "Fotógrafo italiano de passagem por Bergen. Jantou com a mulher duas vezes. Parece mais apaixonado do que culpado.")
                .temper(55, 40, 60, 40)
                .topics(vec![
                    topic("jantar", "O senhor jantou com ela no Hotel Bryggen.", "Jantei. Ela era... impossível. Na segunda noite me disse que ia embora com 'o engenheiro' e que depois disso nunca mais usaria outro nome. Eu tirei fotos dela, sim. Ela não gostou. Pediu o filme e eu não dei.")
                        .lie("Nunca a vi. Sou turista, fotografo fiordes.", 7)
                        .reveals(&[11]),
                    topic("ciume", "O senhor sentiu ciúme do engenheiro?", "Senti. E fui para Stavanger no dia 22, de ônibus. O motorista lembra de mim, eu vomitei no banco.")
                        .req(Req::Topic("jantar")),
                ]),
            person("Olav", "Strand", false, 55, Job::Detective, Role::Contact, B(BKind::House, 9), "Inspetor da polícia de Bergen. Quer arquivar o caso como suicídio antes que o Serviço de Inteligência apareça.")
                .temper(40, 55, 55, 45)
                .topics(vec![
                    topic("caso", "O que a polícia concluiu?", "Suicídio. É a versão oficial. Entre nós: ninguém arranca as etiquetas da própria roupa para se matar. E ninguém se queima vivo depois de engolir cinquenta comprimidos de Fenemal. Mas o que eu acho não interessa a quem manda.")
                        .reveals(&[12]),
                    topic("malas", "E as malas da estação?", "Estão no guarda-volumes, lacradas. O Serviço de Inteligência pediu para ninguém mexer até segunda. Hoje é sábado, senhor Vale. Eu não vi o senhor entrar lá."),
                ]),
            person("Randi", "Brekke", true, 43, Job::Housewife, Role::Witness, B(BKind::House, 4), "Esposa de Halvard. Toma Fenemal para dormir. Nos últimos dias, o frasco esvazia rápido demais.")
                .temper(80, 30, 70, 20)
                .topics(vec![
                    topic("marido", "Onde seu marido estava na noite de 23 de novembro?", "Ele saiu às nove com o carro da Marinha. Voltou às quatro, cheirando a gasolina. Lavou as luvas na pia da cozinha. E o meu frasco de Fenemal, que eu tinha comprado na segunda... estava vazio.")
                        .lie("Em casa, comigo. Ele nunca sai à noite.", 8)
                        .reveals(&[13]),
                ]),
        ],
        clues: vec![
            clue("Corpo queimado no vale", ClueKind::Physical, Some(Woods), Look3d::Blood, "O corpo está em posição de boxeador, entre pedras escurecidas. Perto dela, um relógio de pulso derretido e restos de um guarda-chuva.", &[])
                .forensic("Fuligem nos pulmões: ela ainda respirava quando o fogo começou. No estômago, dezenas de comprimidos de Fenemal, dissolvidos pela metade. Estava dormindo, não morta."),
            clue("Garrafas de gasolina e garrafa térmica", ClueKind::Physical, Some(Woods), Look3d::Object, "Duas garrafas de plástico com cheiro de gasolina e uma garrafa térmica com um resto de chá amargo.", &[1])
                .forensic("O chá contém Fenemal moído. No fundo de uma das garrafas, o carimbo do posto de Fana — onde abastecem os carros de serviço da estação naval."),
            clue("Malas no guarda-volumes", ClueKind::Physical, Some(B(BKind::Station, 0)), Look3d::Object, "Duas malas sem uma única etiqueta. Perucas, óculos sem grau, cremes com os rótulos raspados, 500 marcos alemães, francos belgas e 900 coroas norueguesas.", &[])
                .forensic("O forro de uma mala foi descosturado e costurado de novo. Lá dentro, pó de revelador fotográfico e um pedaço de negativo cortado com tesoura."),
            clue("Caderno cifrado", ClueKind::Document, Some(B(BKind::Station, 0)), Look3d::Paper, "Um caderno com letras e números: 'O22 P28 L21 O29 ... B23 H.B. V'. Parecem datas e lugares.", &[1])
                .forensic("As datas coincidem com os testes de foguetes na baía de Bergen. A última entrada: '23/11 — H.B. — Vale'. H.B.: Halvard Brekke.")
                .hidden(1),
            clue("Registro do Hotel Bryggen", ClueKind::Document, Some(B(BKind::Hotel, 1)), Look3d::Paper, "Três nomes, três quartos, a mesma caligrafia: Genevieve Lancier, Claudia Nielsen, Vera Moreau. Saída no dia 23, conta paga em dinheiro.", &[]),
            clue("Oito passaportes", ClueKind::Document, Some(HomeOf(0)), Look3d::Paper, "Escondidos no fundo falso de uma nécessaire, oito passaportes com fotos diferentes da mesma mulher.", &[])
                .forensic("Falsificações excelentes. Nenhum deles tem o carimbo de entrada na Noruega: ela chegou por um caminho que não passa por fronteiras.")
                .hidden(1)
                .network(Job::Police),
            clue("Negativos na casa de Brekke", ClueKind::Physical, Some(HomeOf(1)), Look3d::Photo, "Na escrivaninha de Brekke, tiras de negativos com fotos da baía e das plataformas de lançamento.", &[1])
                .forensic("As tiras foram cortadas com a mesma tesoura do negativo escondido no forro da mala. Faltam exatamente os quadros que ela levava.")
                .hidden(2),
            clue("Filmes de Luca", ClueKind::Physical, Some(HomeOf(4)), Look3d::Photo, "Rolos de filme com dezenas de retratos da mulher, no restaurante, no porto, no elevador.", &[4])
                .forensic("Fotos de um homem apaixonado. Nenhuma foi tirada depois do dia 21.")
                .herring(),
            clue("Receita de Fenemal", ClueKind::Document, Some(B(BKind::Pharmacy, 0)), Look3d::Paper, "Receita em nome de Randi Brekke. Cem comprimidos retirados na segunda, dia 23. Assinatura de quem retirou: H. Brekke.", &[1])
                .network(Job::Pharmacist),
            testimony("A hóspede dos três quartos", "Solveig: a mulher trocou de quarto três vezes, recebeu um homem de luvas com anel de escola técnica e saiu no dia 23 pedindo que ninguém soubesse dela.", &[1]),
            testimony("O casal na trilha", "Arne cruzou na véspera com a mulher e um homem de sobretudo cinza e luvas, que carregava uma garrafa térmica.", &[1]),
            testimony("O engenheiro", "Luca: ela disse que ia embora com 'o engenheiro' e que depois nunca mais usaria outro nome.", &[1]),
            testimony("Suicídio, diz a polícia", "Strand: a versão oficial é suicídio, mas ninguém arranca as próprias etiquetas nem se queima depois de engolir cinquenta comprimidos.", &[]),
            testimony("Voltou às quatro cheirando a gasolina", "Randi: Halvard saiu às nove com o carro da Marinha, voltou às quatro cheirando a gasolina e o frasco de Fenemal estava vazio.", &[1]),
            clue("Eco: o vale", ClueKind::Temporal, Some(Woods), Look3d::None, "No eco, um homem de luvas serve chá a uma mulher sentada nas pedras. Ela adormece. Ele despeja as garrafas sobre ela. O relógio no pulso dela marca 3:17.", &[1]),
            clue("A última página", ClueKind::Document, Some(B(BKind::Station, 0)), Look3d::Glow, "Colada ao verso da capa do caderno, uma página que não segue o código dela: 'Laboratório Delta — 1971 — a esfera precisa de alguém que a construa. E.V.'", &[])
                .hidden(3),
        ],
        echoes: vec![
            echo(Woods, Ghost::Struggle, &["Neblina entre as pedras.", "Um homem de luvas estende uma garrafa térmica.", "'Beba. A trilha é longa até a Suécia.'", "Ela dorme. Um cheiro de gasolina.", "Um relógio derretendo: 3:17."], 14),
        ],
        culprit: 1,
        methods: &[
            "Luca a empurrou numa fogueira depois de uma briga de amantes",
            "Sedada com Fenemal moído no chá da garrafa térmica e queimada viva com gasolina",
            "Ela engoliu os comprimidos e ateou fogo a si mesma",
            "Estrangulada no hotel e levada ao vale já morta",
        ],
        method: 1,
        motives: &[
            "Ciúme — ela ia partir com outro homem",
            "Ela ia levar a Bruxelas o caderno que provava que ele vendia os segredos da estação naval",
            "Roubar o dinheiro das malas",
            "Uma ordem direta de um serviço secreto estrangeiro",
        ],
        motive: 1,
        anomalies: &[
            "Nenhuma — é apenas um crime",
            "O caderno dela traz, com letra que não é a dela, o Laboratório Delta, a esfera e o ano de 1971 — e o relógio derretido parou às 3:17",
            "O corpo tinha sido queimado dias antes de ela chegar a Bergen",
            "Os oito passaportes são de oito mulheres diferentes que existiram de verdade",
        ],
        anomaly: 1,
        threads: &[(1, 0), (1, 6), (0, 4), (0, 2), (5, 1), (3, 1)],
        story: "Halvard Brekke vendia os horários dos testes de foguetes da baía a uma mulher de oito nomes — e, pelas costas dela, vendia a mesma coisa a quem pagasse mais. Ela descobriu, e seu caderno cifrado provava tudo. Brekke ofereceu-se para tirá-la do país pelas montanhas, subiu o Vale do Gelo com ela, serviu-lhe chá com o Fenemal da esposa e, quando ela adormeceu, queimou-a com a gasolina do carro da Marinha. As etiquetas, ela mesma arrancara: era o trabalho dela não existir.",
        sphere: "A última página do caderno não foi escrita por ela. A letra é a sua, Elias — mais velha, mais trêmula. Alguém já está recrutando nomes para 1971. A esfera não nasce num laboratório: ela é encomendada, peça por peça, através das décadas.",
        on_true: outcome(
            "Halvard Brekke foi preso ao tentar embarcar no ferry para Newcastle. O Serviço de Inteligência levou o caderno e negou que ele existisse. A mulher foi enterrada sem nome, num caixão de zinco, com uma cerimônia católica. Solveig foi a única pessoa presente.",
            "ENGENHEIRO DA ESTAÇÃO NAVAL É PRESO PELA MORTE NO VALE DO GELO",
            "...o Ministério da Defesa não comenta a prisão de um funcionário civil...",
            vec![(1, "jailed"), (6, "left_city"), (2, "grateful"), (5, "police")],
        )
        .flags(vec!["isdal_solved"]),
        on_false: outcome(
            "O caso foi arquivado como suicídio. Brekke foi promovido. Em 1971, Solveig encontrou no balcão do hotel um envelope sem remetente, com uma única palavra no código da mulher: DELTA.",
            "MULHER DO VALE DO GELO: POLÍCIA CONFIRMA SUICÍDIO",
            "...a identidade da mulher continua desconhecida...",
            vec![(1, "criminal")],
        )
        .flags(vec!["isdal_unsolved"]),
        digit: "9",
        reward: 90,
    }
}

// ------------------------------------------------------------------ 17 — São Francisco, 1969

pub fn case17() -> CaseDef {
    CaseDef {
        id: 17,
        title: "As Cartas Cifradas",
        city: CityId::SanFrancisco,
        year: 1969,
        intro: "São Francisco, 1969. Um homem que assina com um círculo cortado por uma cruz escreve aos jornais. Manda cifras, conta mortos, promete outros. Esta semana matou um taxista em Presidio Heights e mandou ao jornal um pedaço da camisa ensanguentada. A nova cifra, de 340 símbolos, ninguém conseguiu ler. Um cartunista do jornal diz que conseguiu uma parte. E que a parte diz: ELIAS VALE.",
        brief: "O taxista Dale Hinkley foi morto com um tiro na nuca. O assassino mandou ao jornal um pedaço da camisa e uma cifra de 340 símbolos que, dizem, contém o seu nome.",
        start: Near(BKind::Mansion, 1),
        cast: vec![
            person("Dale", "Hinkley", false, 29, Job::Driver, Role::Victim, B(BKind::Apartment, 3), "Taxista da Yellow Cab, estudante de letras à noite. Morto com um tiro na nuca dentro do próprio táxi.").dead(),
            person("Raymond", "Voss", false, 38, Job::Clerk, Role::Suspect, B(BKind::Apartment, 6), "Funcionário do turno da madrugada no centro de triagem dos Correios. Ex-criptógrafo da Marinha. Corpulento, de óculos, cabelo à escovinha. Fala baixo e sorri no lugar errado.")
                .temper(15, 70, 5, 40)
                .flees()
                .topics(vec![
                    topic("cartas", "O senhor guarda um manual de criptografia da Marinha em casa.", "Guardei, sim. A Marinha me ensinou a pensar em símbolos. Metade da cidade tenta ler essas cartas no café da manhã. Eu só leio mais rápido. Isso é crime, agora?")
                        .lie("Cifras? Eu separo envelopes, senhor. Não leio o que tem dentro.", 5),
                    topic("noite", "Onde o senhor estava na noite em que o taxista morreu?", "Bati o ponto às duas e saí. Gosto de andar à noite, pela Presidio. O ar é limpo. Ninguém te olha.")
                        .lie("De plantão no centro de triagem. A noite toda. Está no livro de ponto.", 6),
                    topic("nome", "A cifra de 340 símbolos tem o meu nome.", "Tem? Então o senhor é o próximo, senhor Vale. Ou o primeiro. Depende de que lado se lê. Eu não escrevi o seu nome. Mas eu sonhei com ele. Às três e dezessete, toda noite, desde junho.")
                        .req(Req::Clue(2)),
                ]),
            person("Gordon", "Kessler", false, 27, Job::Journalist, Role::Contact, B(BKind::House, 2), "Cartunista do jornal. Largou o sono, a esposa e os desenhos para tentar decifrar as cartas. Tem os olhos de alguém que Elias conhece do espelho.")
                .temper(45, 60, 80, 15)
                .topics(vec![
                    topic("cifra", "Como você decifrou parte da cifra?", "O cara não inventou nada. É uma substituição homofônica com transposição em diagonal, igual ao manual de campo da Marinha de 1955. Quem escreveu aprendeu a cifrar num navio. E usa selos demais: sempre dois, como se tivesse medo de a carta não chegar.")
                        .reveals(&[11]),
                    topic("obsessao", "Por que isso te consome desse jeito?", "Porque ninguém mais fica acordado às 3:17 contando símbolos. Minha mulher levou as crianças. Meu editor me chama de maluco. E você... você tem a mesma cara que eu tenho no espelho às quatro da manhã. Há quanto tempo você não dorme, Elias?"),
                    topic("parede", "O que são essas fotos na sua parede?", "Recortes de crimes de 1920, 1946, 1958. Em todos, na borda da foto, o mesmo homem. Olha bem. É você. Eu ia te perguntar quando você chegasse. Eu sabia que você ia chegar.")
                        .req(Req::Clue(14)),
                ]),
            person("Michael", "Dorsey", false, 20, Job::Student, Role::Witness, B(BKind::House, 5), "Estudante. Sobreviveu ao ataque na estrada do Lago Berman, onde a namorada morreu. Tem quatro cicatrizes de bala e uma voz que treme.")
                .temper(85, 30, 80, 10)
                .topics(vec![
                    topic("noite", "O que aconteceu na estrada do lago?", "Um carro parou atrás do nosso. Uma lanterna na cara. Depois os tiros, sem uma palavra. Quando eu caí, ele se abaixou, calmo, e escreveu alguma coisa na porta do carro. Era gordo, de óculos, cabelo curto. Calça de prega, como um militar de folga. Ele disse o meu nome. Eu nunca tinha visto aquele homem.")
                        .lie("Eu não vi nada. Só a luz. Por favor, não me faça lembrar.", 15)
                        .reveals(&[7]),
                ]),
            person("Leonard", "Pike", false, 41, Job::Teacher, Role::Suspect, B(BKind::House, 8), "Professor primário demitido. Mora com a mãe, caça esquilos e usa um relógio com o símbolo do zodíaco. A polícia inteira acha que é ele.")
                .temper(50, 45, 40, 45)
                .topics(vec![
                    topic("relogio", "Esse relógio tem o mesmo símbolo das cartas.", "Minha mãe me deu no Natal de 1967. É uma marca suíça, vende em qualquer loja de penhores da Mission. Eu sei o que parece. Eu sei que pareço o homem certo. Esse é o problema de parecer.")
                        .lie("Que relógio? Eu não uso relógio.", 4),
                    topic("noite", "Onde o senhor estava quando o taxista foi morto?", "Em casa, vendo o programa do Johnny Carson com a minha mãe. Ela não dorme antes do fim. A faca com sangue no meu carro é de esquilo. A polícia já sabe, só não quer saber.")
                        .reveals(&[10]),
                    topic("simbolos", "O senhor conhece alguém que desenhe esses símbolos?", "Conheço. Um tal de Voss, que joga xadrez no Café Trieste. Ex-Marinha. Uma vez desenhou esse círculo com a cruz num guardanapo e disse: 'É a mira de um rifle, Leonard. É tudo o que o mundo é.' Eu nunca mais joguei com ele.")
                        .req(Req::Topic("relogio"))
                        .reveals(&[12]),
                ]),
            person("Frank", "Arnone", false, 44, Job::Detective, Role::Contact, B(BKind::House, 10), "Inspetor de homicídios da polícia de São Francisco. Gravata frouxa, café frio, três casos abertos na mesa. Tem certeza de que o culpado é Pike.")
                .temper(35, 70, 65, 30)
                .topics(vec![
                    topic("suspeito", "A polícia tem um suspeito?", "Leonard Pike. Relógio com o símbolo, facas com sangue no carro, mora com a mãe, odeia mulheres. Só falta uma digital que bata. As do táxi não batem. Ainda.")
                        .reveals(&[9]),
                    topic("patrulha", "Uma patrulha passou pelo assassino naquela noite?", "Passou. O rádio avisou para procurar um suspeito negro. Os meus homens viram um branco gordo, de óculos, andando para a Presidio, e seguiram em frente. Se isso sair no jornal, a delegacia pega fogo."),
                ]),
            person("Ellen", "Pruitt", true, 15, Job::Student, Role::Witness, B(BKind::Mansion, 1), "Adolescente de Presidio Heights. Viu tudo da janela do quarto, na esquina onde o táxi parou.")
                .temper(55, 60, 85, 10)
                .topics(vec![
                    topic("janela", "O que você viu da janela, Ellen?", "O táxi parou na esquina. O passageiro estava no banco de trás. Depois ele passou para a frente e ficou limpando o painel com um pano, com toda a calma. Era branco, gordo, de óculos. Uma viatura passou do lado dele e não parou. Ele foi andando para a Presidio. O relógio do táxi marcava 3:17 — eu vi a luz verde.")
                        .reveals(&[8]),
                ]),
        ],
        clues: vec![
            clue("Taxímetro parado", ClueKind::Physical, Some(Near(BKind::Mansion, 1)), Look3d::Glow, "O taxímetro do Yellow Cab ainda marca a corrida: $6,25. O relógio do painel parou às 3:17.", &[])
                .forensic("O painel foi limpo com um pano, mas o limpador era destro e deixou a borda esquerda intacta. O táxi foi chamado da Union Square, a duas quadras do centro de triagem dos Correios."),
            clue("Pedaço da camisa ensanguentada", ClueKind::Physical, Some(B(BKind::Newspaper, 0)), Look3d::Blood, "Um retalho da camisa de Dale, mandado ao jornal dentro de um envelope com dois selos.", &[1])
                .forensic("O envelope foi carimbado às 3:17 no centro de triagem da Rua Sete, antes de qualquer caixa de correio da cidade ser esvaziada. Alguém lá dentro o carimbou."),
            clue("Cifra de 340 símbolos", ClueKind::Document, Some(B(BKind::Newspaper, 0)), Look3d::Paper, "Dezessete linhas de vinte símbolos. Triângulos, letras invertidas, o círculo com a cruz.", &[])
                .forensic("Lida em diagonal, a partir do canto inferior, uma sequência se repete três vezes: E-L-I-A-S V-A-L-E 3-1-7. A carta foi escrita em junho. Você chegou ontem."),
            clue("Digitais ensanguentadas no táxi", ClueKind::Physical, Some(Near(BKind::Mansion, 1)), Look3d::Prints, "Digitais de sangue na maçaneta interna e no retrovisor.", &[1])
                .forensic("Não batem com as de Leonard Pike. Batem com a ficha de alistamento de um criptógrafo da Marinha dispensado em 1958: R. Voss.")
                .network(Job::Police),
            clue("Relógio do zodíaco", ClueKind::Physical, Some(HomeOf(4)), Look3d::Object, "Um relógio suíço com o círculo e a cruz no mostrador, na cômoda de Leonard Pike.", &[4])
                .forensic("Modelo de 1967, vendido aos milhares. O símbolo é o logotipo da marca, não uma assinatura.")
                .herring(),
            clue("Manual da Marinha e selos", ClueKind::Document, Some(HomeOf(1)), Look3d::Paper, "Um manual de criptografia de campo da Marinha, cheio de anotações, e uma cartela de selos com fileiras inteiras arrancadas de duas em duas.", &[1])
                .forensic("As anotações à margem usam o mesmo triângulo invertido da cifra de 340. A caneta de feltro azul é a mesma das cartas.")
                .hidden(1),
            clue("Livro de ponto dos Correios", ClueKind::Document, Some(B(BKind::Office, 0)), Look3d::Paper, "O livro de ponto do centro de triagem. Na noite do crime, R. Voss bateu o ponto de saída às 2:04. Nas noites das outras cartas, também.", &[1])
                .network(Job::Journalist),
            testimony("O homem que sabia o nome", "Michael: o atirador era gordo, de óculos, cabelo à escovinha, calça de prega. Escreveu na porta do carro e disse o nome dele.", &[1]),
            testimony("A janela de Ellen", "Ellen viu um branco gordo de óculos limpando o painel do táxi. Uma viatura passou sem parar. O relógio marcava 3:17.", &[1]),
            testimony("Pike, o suspeito perfeito", "Arnone: a polícia tem certeza de que é Leonard Pike, mas as digitais do táxi não batem.", &[4]),
            testimony("A noite de Pike", "Pike estava em casa com a mãe, vendo televisão. O sangue das facas é de esquilo.", &[]),
            testimony("O decifrador", "Gordon: a cifra segue o manual de campo da Marinha de 1955. O autor aprendeu a cifrar num navio e sempre usa dois selos.", &[1]),
            testimony("O homem do guardanapo", "Pike: um ex-marinheiro chamado Voss desenhou o círculo com a cruz num guardanapo e disse que era a mira de um rifle.", &[1]),
            clue("Eco: o táxi", ClueKind::Temporal, Some(Near(BKind::Mansion, 1)), Look3d::None, "No eco, o passageiro do banco de trás encosta a pistola na nuca de Dale. Depois rasga a camisa com cuidado de alfaiate. O taxímetro marca 3:17.", &[1]),
            clue("Parede de recortes de Gordon", ClueKind::Document, Some(HomeOf(2)), Look3d::Glow, "Centenas de recortes presos com tachinhas e barbante vermelho. Em fotos de 1920, 1946 e 1958, na borda do quadro, sempre o mesmo homem de costas.", &[])
                .hidden(2),
            clue("Eco: a estrada do lago", ClueKind::Temporal, Some(Woods), Look3d::None, "No eco, um carro para atrás de outro na estrada escura. Uma lanterna. Os tiros. Um homem de óculos se agacha e escreve datas na porta com caneta de feltro.", &[1]),
        ],
        echoes: vec![
            echo(Near(BKind::Mansion, 1), Ghost::Struggle, &["Uma luz verde no painel: 3:17.", "'Siga até a Cherry Street, por favor.'", "Um estampido abafado.", "Um pano limpando o painel, devagar."], 13),
            echo(Woods, Ghost::Flee, &["Faróis se apagam na estrada do lago.", "Uma lanterna ofusca o para-brisa.", "Um rapaz se arrasta pelo cascalho.", "Uma caneta escreve na lataria: a data, a hora."], 15),
        ],
        culprit: 1,
        methods: &[
            "Pike o esfaqueou com a faca de caça",
            "Entrou no táxi como passageiro e atirou na nuca do motorista; depois rasgou a camisa para 'assinar' a carta",
            "Assalto comum que deu errado",
            "Um cúmplice dirigiu enquanto outro atirava de fora",
        ],
        method: 1,
        motives: &[
            "O dinheiro da corrida",
            "Ser lido — ele mata para escrever, e escreve para ser decifrado",
            "Vingança contra a companhia de táxis",
            "Ciúme de uma namorada do taxista",
        ],
        motive: 1,
        anomalies: &[
            "Nenhuma — é apenas um crime",
            "O autor sabia datilografar",
            "A cifra, escrita meses antes de você chegar, contém ELIAS VALE e 3:17 — e Voss sonha com essa hora desde junho",
            "Os selos são de 1990",
        ],
        anomaly: 2,
        threads: &[(1, 0), (1, 3), (2, 1), (4, 1), (5, 4), (6, 1)],
        story: "Raymond Voss aprendeu a cifrar na Marinha e nunca aprendeu a viver fora dela. No turno da madrugada dos Correios, ele via cada carta passar pelas suas mãos — inclusive as suas. Matava para ter o que escrever e escrevia para ser lido. Na noite do crime, bateu o ponto às duas, chamou um táxi na Union Square, matou Dale em Presidio Heights e carimbou o envelope com a camisa às 3:17, dentro do próprio centro de triagem.",
        sphere: "Voss não inventou o seu nome. A esfera vaza pelas frestas de quem fica acordado às 3:17 — e ele e Gordon estavam acordados todas as noites. Um virou assassino, o outro virou você. É a mesma fenda, Elias. O que muda é o que se faz com ela.",
        on_true: outcome(
            "Raymond Voss foi preso no centro de triagem, com uma carta ainda sem selo no bolso. As cartas pararam. Gordon voltou a desenhar — um homem de costas, sempre de costas, na borda de cada tira.",
            "FUNCIONÁRIO DOS CORREIOS É PRESO COMO O ASSASSINO DAS CARTAS CIFRADAS",
            "...a polícia de São Francisco confirma que as digitais do táxi pertencem ao suspeito...",
            vec![(1, "jailed"), (2, "journalist"), (4, "grateful"), (3, "survived")],
        )
        .flags(vec!["cipher_caught"]),
        on_false: outcome(
            "A cidade inteira decidiu que era Pike. Ele nunca foi julgado, mas nunca mais conseguiu emprego. As cartas continuaram por mais cinco anos. A última trazia um desenho de um homem de sobretudo e a frase: 'Nos vemos em Portland.'",
            "A POLÍCIA CERCA O PROFESSOR. O ASSASSINO ESCREVE DE NOVO",
            "...uma nova cifra chegou hoje à redação...",
            vec![(4, "hates_elias"), (1, "criminal"), (2, "left_city")],
        )
        .flags(vec!["cipher_free"]),
        digit: "9",
        reward: 95,
    }
}

// ------------------------------------------------------------------ 18 — São Francisco, 1974

pub fn case18() -> CaseDef {
    CaseDef {
        id: 18,
        title: "A Comuna da Colina",
        city: CityId::SanFrancisco,
        year: 1974,
        intro: "São Francisco, 1974. No alto de Haight-Ashbury, numa casa vitoriana pintada de roxo, vive a Família da Colina. O líder, Irmão Orion, anunciou num domingo: 'Às 3:17 de quinta, um de nós vai partir.' Na quinta, às 3:17, Tobias Reyes convulsionou até morrer no chão da cozinha. Na mesma semana, Caroline Whitlock, herdeira de um império de jornais, foi arrancada do apartamento. Desde então, uma rádio da madrugada recebe fitas com a voz dela.",
        brief: "Tobias Reyes morreu às 3:17, na hora profetizada pelo líder da comuna. A herdeira Caroline Whitlock está desaparecida e manda mensagens por fitas enviadas a uma rádio.",
        start: HomeOf(0),
        cast: vec![
            person("Tobias", "Reyes", false, 24, Job::Musician, Role::Victim, B(BKind::House, 5), "Violonista da comuna. Morreu em convulsões no chão da cozinha, às 3:17, exatamente como Orion anunciou.").dead(),
            person("Orion", "Kittredge", false, 41, Job::Priest, Role::Suspect, B(BKind::House, 5), "O 'Irmão Orion'. Ex-vendedor de enciclopédias que virou profeta. Sonha com horas exatas e acorda chorando.")
                .temper(55, 40, 55, 35)
                .topics(vec![
                    topic("profecia", "Como o senhor sabia que alguém morreria às 3:17?", "Eu não sei nada. Eu sonho. Desde criança, sonho com um relógio vermelho e um homem de casaco que diz a hora. Às vezes alguém morre na hora do sonho. Eu comecei a anunciar para dar sentido ao que eu vejo. Os garotos chamam de profecia. Eu chamo de maldição.")
                        .reveals(&[7]),
                    topic("cha", "Quem preparou o chá de Tobias naquela noite?", "Dana. Ela sempre prepara o chá da meia-noite. Tobias não queria beber, disse que estava com o estômago ruim. Ela insistiu. Disse: 'É o último, Toby. Prometo.' Eu achei bonito, na hora.")
                        .req(Req::Topic("profecia"))
                        .reveals(&[12]),
                    topic("caroline", "A herdeira Whitlock esteve aqui?", "Todos estiveram aqui um dia. É uma casa aberta. Mas a garota Whitlock... Dana trouxe ela duas vezes, antes do 'sequestro'. Ela ria muito. Não parecia assustada com nada."),
                ]),
            person("Dana", "Kovach", true, 29, Job::Student, Role::Suspect, B(BKind::House, 5), "Ex-estudante de química de Berkeley, braço direito de Orion. Fala de revolução com a calma de quem já fez as contas.")
                .temper(20, 80, 20, 60)
                .flees()
                .topics(vec![
                    topic("sequestro", "Você sabe onde Caroline Whitlock está?", "Ninguém sequestrou ninguém. Caroline quis sair da gaiola do pai e eu dei a porta. As fitas são dela. As exigências são dela. O pai finalmente vai alimentar a cidade que ele envenena todo dia com os jornais dele.")
                        .lie("O Exército Popular a levou. Eu só li sobre isso no jornal, como todo mundo.", 2),
                    topic("cha", "Foi você quem preparou o chá de Tobias?", "Eu preparo o chá de todo mundo. Coloquei um ácido no dele, um só. Tobias estava paranoico, precisava relaxar. Se tinha outra coisa no papel, não fui eu que pus.")
                        .lie("Não lembro quem fez o chá. Todo mundo mexe naquela cozinha.", 4),
                    topic("tobias", "Tobias ameaçou contar alguma coisa à polícia?", "Tobias era um menino de Fresno com um violão. Achava que revolução era cantar. Ele não entendia que algumas coisas custam caro.")
                        .req(Req::Clue(8)),
                ]),
            person("Caroline", "Whitlock", true, 19, Job::Aristocrat, Role::Suspect, B(BKind::Apartment, 8), "A herdeira desaparecida. Escondida num apartamento da comuna, com uma carabina e um nome novo: 'Tânia'.")
                .temper(45, 65, 45, 30)
                .topics(vec![
                    topic("sequestro", "Você foi levada à força, Caroline?", "...Não. Eu abri a porta. Dana escreveu o roteiro e eu li as fitas. Quatro milhões em comida para os pobres de Oakland — foi o único jeito de fazer meu pai dar alguma coisa a alguém.")
                        .lie("Eles me arrancaram da cama. Eu sou prisioneira de guerra. Não posso dizer mais nada.", 6),
                    topic("tobias", "O que aconteceu com Tobias?", "Tobias descobriu que eu comprei a carabina duas semanas antes do 'sequestro'. Ficou apavorado. Disse que ia contar tudo à polícia antes que alguém se machucasse. Dana disse que ia 'conversar com ele'. Na quinta, ele estava morto.")
                        .req(Req::Topic("sequestro"))
                        .reveals(&[10]),
                ]),
            person("Harlan", "Whitlock", false, 58, Job::Aristocrat, Role::Contact, B(BKind::Mansion, 0), "Pai de Caroline, dono do San Francisco Courier. Já distribuiu dois milhões em comida e não sabe mais se paga um resgate ou uma vingança.")
                .temper(60, 45, 40, 70)
                .topics(vec![
                    topic("filha", "Como era sua relação com Caroline?", "Ela me odiava. Dizia que meus jornais eram fábricas de medo. Na última briga, disse que um dia eu ia pagar por cada manchete. Eu achei que era drama de uma menina de dezenove anos. Agora eu pago toda semana.")
                        .reveals(&[11]),
                    topic("fitas", "O senhor acredita nas fitas?", "É a voz dela. Mas ela lê como quem lê a lista do mercado. A minha filha grita quando tem medo. Naquelas fitas ela não tem medo nenhum."),
                ]),
            person("Joan", "Ashby", true, 32, Job::RadioHost, Role::Contact, B(BKind::Apartment, 4), "Locutora da madrugada da KVLT. As fitas do sequestro chegam sempre ao estúdio dela. Às vezes chama Elias por um nome que não é o dele.")
                .temper(30, 75, 80, 20)
                .soul(1)
                .topics(vec![
                    topic("fitas", "Como as fitas chegam à rádio?", "Sempre às 3:17, na caixa de correio do estúdio. Eu toco no ar sem cortar nada. Mas escuta a terceira com fone: no fundo, alguém sopra as frases antes da Caroline. É uma mulher. Eu conheço essa voz. É a Dana, da comuna — ela ligava aqui pedindo Joplin.")
                        .reveals(&[9]),
                    topic("voce", "Você me chamou de outro nome agora.", "Chamei? Eu disse... Jonas? Não, Elias. Eu sei que é Elias. É que às vezes eu sonho com uma rádio em Portland, com neve na janela, e alguém que liga para dizer o futuro. Você está lá. Mais velho."),
                    topic("rolo", "O que é este rolo de fita sem etiqueta?", "Apareceu no arquivo na semana passada. Ninguém gravou. Toca uma voz de homem dizendo a hora, e depois o meu nome. Não o meu nome de agora. Um nome que eu ainda não tive.")
                        .req(Req::Clue(14)),
                ]),
            person("Samuel", "Okafor", false, 45, Job::Doctor, Role::Witness, B(BKind::Apartment, 6), "Médico da clínica gratuita da Haight. Atendeu Tobias na véspera e assinou o atestado de óbito. Não gostou do que viu.")
                .temper(40, 65, 85, 15)
                .topics(vec![
                    topic("tobias", "Tobias esteve na clínica antes de morrer?", "Na véspera. Não estava doente — estava com medo. Disse: 'Doutor, o sequestro é teatro. A garota está com a gente. Eu vou contar à polícia amanhã, antes que alguém leve um tiro.' Pedi que ele fosse naquela hora mesmo. Ele disse que antes precisava se despedir de alguém.")
                        .reveals(&[8]),
                    topic("morte", "Do que Tobias morreu?", "Estricnina. Não foi overdose, não foi ácido ruim. Arco das costas, maxilar travado, consciente até o fim. É veneno de rato. Alguém dissolveu no chá dele, ou no papel que ele engoliu."),
                ]),
        ],
        clues: vec![
            clue("Corpo de Tobias", ClueKind::Physical, Some(HomeOf(0)), Look3d::Blood, "Tobias está no chão da cozinha, as costas arqueadas, os punhos fechados. Ao lado, uma xícara de chá de hibisco.", &[])
                .forensic("Rigidez imediata e maxilar travado: estricnina. Na xícara, resíduo de um papel dissolvido."),
            clue("Mata-borrão de ácido", ClueKind::Physical, Some(HomeOf(0)), Look3d::Object, "Uma cartela de papel mata-borrão estampada com uma esfera roxa. Falta um quadrado.", &[2])
                .forensic("Os quadrados vizinhos ao que falta estão impregnados de estricnina. A cartela foi cortada com a guilhotina do laboratório de química de Berkeley — o mesmo corte de outras cartelas que Dana distribui."),
            clue("Fita cassete do sequestro", ClueKind::Document, Some(B(BKind::Radio, 0)), Look3d::Paper, "A terceira fita: 'Pai, eu estou bem. Eu escolhi um nome novo.' A voz de Caroline é firme.", &[3, 2])
                .forensic("No fundo, abafada, uma voz feminina sopra cada frase meio segundo antes. E o sino de um bonde da linha Powell: a gravação foi feita perto da Rua Hyde."),
            clue("Caderno de profecias de Orion", ClueKind::Document, Some(HomeOf(1)), Look3d::Paper, "Um caderno de capa roxa com datas, horas e nomes. Na página de quinta: '3:17 — um de nós parte.'", &[1])
                .forensic("As profecias são vagas. Nenhuma cita Tobias. Orion anotava o sonho, não um plano.")
                .herring(),
            clue("Laboratório improvisado", ClueKind::Physical, Some(B(BKind::Abandoned, 0)), Look3d::Object, "Num galpão abandonado, uma balança de precisão, frascos de laboratório e uma lata de veneno de rato aberta.", &[2])
                .forensic("Digitais de Dana na balança. A lata de veneno foi comprada com o nome dela, numa loja de ferragens da Rua Haight.")
                .hidden(1),
            clue("Carta de exigências", ClueKind::Document, Some(B(BKind::Newspaper, 0)), Look3d::Paper, "Datilografada: 'Quatro milhões de dólares em comida para o povo. Ou a herdeira responde pelos crimes do pai.'", &[2])
                .forensic("A máquina é uma Olivetti com a letra 'r' torta. Igual à dos panfletos que Dana distribui na porta da comuna.")
                .network(Job::Journalist),
            clue("Recibo da carabina", ClueKind::Document, Some(B(BKind::GunShop, 0)), Look3d::Paper, "Uma carabina M1 vendida duas semanas antes do sequestro. Compradora: C. Whitlock. Pago em dinheiro, $120.", &[3])
                .hidden(1)
                .network(Job::Police),
            testimony("A profecia de Orion", "Orion sonha desde criança com um relógio vermelho e um homem de casaco que lhe diz a hora. Às vezes alguém morre na hora do sonho.", &[1]),
            testimony("Tobias queria falar", "Dr. Okafor: na véspera, Tobias disse que o sequestro era teatro e que ia contar à polícia.", &[2, 3]),
            testimony("A voz que sopra", "Joan: na terceira fita, alguém sopra as frases antes de Caroline. É a voz de Dana.", &[2]),
            testimony("A confissão de Caroline", "Caroline: Tobias descobriu a carabina e ia à polícia. Dana disse que ia 'conversar com ele'.", &[2]),
            testimony("A filha que odiava o pai", "Harlan: Caroline dizia que um dia ele ia pagar por cada manchete.", &[3]),
            testimony("O último chá", "Orion: Dana preparou o chá de Tobias à meia-noite e insistiu que ele bebesse: 'É o último, Toby.'", &[2]),
            clue("Eco: a cozinha da comuna", ClueKind::Temporal, Some(HomeOf(0)), Look3d::None, "No eco, Dana corta um quadrado de papel com uma tesoura e o dissolve numa xícara de hibisco. O relógio de parede marca meia-noite. Depois, 3:17.", &[2]),
            clue("Rolo sem etiqueta", ClueKind::Document, Some(B(BKind::Radio, 0)), Look3d::Glow, "Um rolo de fita que ninguém gravou. Uma voz masculina, cansada: 'São 3:17 em Portland, 1978. Nadia, não atenda a próxima ligação.'", &[])
                .hidden(3),
            clue("Eco: a discussão no parque", ClueKind::Temporal, Some(Park), Look3d::None, "No eco, no alto da colina do parque, Tobias grita com Dana: 'Eu vou contar tudo amanhã!' Ela sorri e lhe oferece um cigarro.", &[2]),
        ],
        echoes: vec![
            echo(HomeOf(0), Ghost::Write, &["Uma cozinha cheia de fumaça de incenso.", "Uma tesoura corta um quadrado de papel.", "O papel afunda no chá vermelho.", "'É o último, Toby. Prometo.'"], 13),
            echo(Park, Ghost::Argue, &["Tambores ao longe, na colina.", "'O sequestro é mentira, Dana!'", "'Tudo é mentira, Toby. Escolhe a tua.'", "Um cigarro aceso. Um abraço que dura demais."], 15),
        ],
        culprit: 2,
        methods: &[
            "Overdose acidental de LSD",
            "Estricnina num quadrado de mata-borrão dissolvido no chá de Tobias, servido horas antes das 3:17",
            "Orion o sufocou durante um ritual",
            "Caroline atirou nele com a carabina",
        ],
        method: 1,
        motives: &[
            "Disputa pela liderança da comuna",
            "Ciúme — Tobias estava apaixonado por Caroline",
            "Calar Tobias, que ia contar à polícia que o sequestro era uma farsa de Dana e Caroline",
            "Ficar com o dinheiro do resgate sozinha",
        ],
        motive: 2,
        anomalies: &[
            "Nenhuma — é apenas um crime",
            "O ácido da comuna provoca visões do futuro",
            "Harlan Whitlock já sabia de tudo antes do sequestro",
            "Orion sonha com a hora das mortes porque a esfera toca a colina às 3:17 — e a rádio recebeu uma fita gravada em Portland, em 1978",
        ],
        anomaly: 3,
        threads: &[(2, 0), (2, 3), (3, 4), (6, 0), (5, 2), (1, 2)],
        story: "Dana Kovach e Caroline Whitlock inventaram juntas o sequestro: Caroline queria punir o pai, Dana queria a revolução e o dinheiro. Tobias descobriu a carabina comprada antes do 'sequestro' e decidiu ir à polícia. Dana conhecia a profecia de Orion e a usou como disfarce: à meia-noite, serviu a Tobias um chá com um quadrado de mata-borrão envenenado com estricnina. Ele morreu às 3:17, e a comuna inteira acreditou que o profeta estava certo.",
        sphere: "Orion não mente: ele sonha com a hora porque a casa roxa fica sobre uma das costuras onde a esfera toca São Francisco. E o rolo sem etiqueta é uma mensagem de 1978, gravada por alguém com a sua voz. Portland está chamando.",
        on_true: outcome(
            "Dana Kovach foi presa no galpão, tentando queimar a balança. Caroline voltou para casa e testemunhou contra ela. Orion desfez a comuna e parou de anunciar horas. Joan tocou Joplin a madrugada inteira.",
            "MORTE NA COMUNA DA COLINA: EX-ESTUDANTE DE BERKELEY É PRESA; HERDEIRA WHITLOCK VOLTA PARA CASA",
            "...a família Whitlock pede privacidade...",
            vec![(2, "jailed"), (3, "survived"), (4, "grateful"), (1, "left_city"), (5, "grateful")],
        )
        .flags(vec!["commune_solved"]),
        on_false: outcome(
            "Orion foi preso como o 'profeta assassino'. Dana e Caroline assaltaram um banco em Sunset duas semanas depois, com a carabina. As fitas pararam. A última terminava com a frase: 'Às 3:17, alguém mais vai partir.'",
            "PROFETA DA COMUNA É PRESO. HERDEIRA É VISTA ARMADA EM ASSALTO",
            "...o FBI considera Caroline Whitlock uma fugitiva...",
            vec![(1, "jailed"), (2, "criminal"), (3, "criminal"), (4, "hates_elias")],
        )
        .flags(vec!["commune_false"]),
        digit: "5",
        reward: 100,
    }
}

// ------------------------------------------------------------------ 19 — Portland, 1971

pub fn case19() -> CaseDef {
    CaseDef {
        id: 19,
        title: "O Passageiro da Poltrona 18C",
        city: CityId::Portland,
        year: 1971,
        intro: "Portland, 1971. Na véspera do Dia de Ação de Graças, um homem de terno escuro comprou uma passagem para Seattle em nome de 'Dan Colter', sentou-se na poltrona 18C, pediu um bourbon e entregou à comissária um bilhete: tenho uma bomba. Recebeu duzentos mil dólares e quatro paraquedas, e saltou pela escada traseira no meio da tempestade. Ninguém sabe quem ele era. Três dias depois, um fazendeiro das margens do rio Lewis apareceu morto no celeiro, com notas do resgate escondidas na bota.",
        brief: "Um sequestrador saltou de um avião com $200.000. Três dias depois, o fazendeiro Earl Tuttle foi morto no celeiro, com notas do resgate na bota.",
        start: HomeOf(0),
        cast: vec![
            person("Earl", "Tuttle", false, 52, Job::Farmer, Role::Victim, B(BKind::Farmhouse, 1), "Fazendeiro das margens do rio Lewis. Devia ao banco e, de repente, deixou de dever.").dead(),
            person("Russell", "Hask", false, 44, Job::Scientist, Role::Suspect, B(BKind::House, 3), "Físico demitido da Boeing. Alugou um galpão na zona industrial e compra metais raros em dinheiro vivo. Fala pouco e olha muito para o relógio.")
                .temper(35, 70, 35, 55)
                .flees()
                .topics(vec![
                    topic("voo", "Onde o senhor estava na véspera de Ação de Graças?", "...Num avião. Poltrona 18C. Eu precisava de duzentos mil dólares, e nenhum banco empresta para um homem que quer construir o que eu quero construir. Ninguém se feriu. Eu fui gentil com a moça. Paguei até o bourbon.")
                        .lie("Em casa, trabalhando. Eu nunca entrei num avião, tenho pavor de altura.", 4),
                    topic("earl", "O senhor conhecia Earl Tuttle?", "Ele me achou na mata, encharcado, com o paraquedas enrolado no braço. Me deu café e um lugar no celeiro. Dei a ele cinco mil pelo silêncio. Três dias depois ele quis cinquenta. Disse que ia ligar para o FBI. Eu só fui conversar.")
                        .lie("Nunca ouvi esse nome. Não conheço fazendeiro nenhum.", 8),
                    topic("esfera", "Para que serve esse projeto de esfera?", "Não é meu. Um homem me procurou em outubro. Sobretudo, cabelo grisalho, voz cansada. Me deu os desenhos e disse: 'Em 1971 alguém precisa começar. Você é só o primeiro tijolo do Laboratório Delta.' Ele sabia coisas sobre mim que eu nunca contei a ninguém.")
                        .req(Req::Clue(6)),
                ]),
            person("Tina", "Mulready", true, 22, Job::Waiter, Role::Witness, B(BKind::Apartment, 2), "Comissária de bordo do voo 305. Sentou ao lado do sequestrador por quase duas horas.")
                .temper(55, 60, 80, 20)
                .topics(vec![
                    topic("voo", "Como era o homem da poltrona 18C?", "Calmo. Educado. Óculos escuros, terno preto, gravata com prendedor de madrepérola. Conhecia o avião melhor que o piloto — sabia que a escada traseira abria em voo. E o relógio dele estava parado. Eu perguntei a hora e ele disse: 'Para mim, é sempre 3:17.'")
                        .reveals(&[9]),
                    topic("bilhete", "O que dizia o bilhete?", "'Tenho uma bomba. Quero duzentos mil em notas de vinte e quatro paraquedas.' Ele pediu o bilhete de volta. Eu devolvi. Não sei por quê. Ele sorriu como se já soubesse que eu ia devolver."),
                ]),
            person("Ralph", "Himmel", false, 48, Job::Detective, Role::Contact, B(BKind::Hotel, 0), "Agente do FBI vindo de Seattle. Tem uma lista de números de série e nenhuma paciência.")
                .temper(30, 70, 60, 30)
                .topics(vec![
                    topic("suspeito", "O FBI já tem um suspeito?", "Duane Rourke. Ex-paraquedista das Forças Especiais, voltou do Vietnã com dívidas e raiva. Tem um paraquedas militar na garagem. Se não for ele, é alguém que saltou igual a ele.")
                        .reveals(&[11]),
                    topic("notas", "As notas na bota de Earl são do resgate?", "Cada uma das vinte mil notas foi microfilmada antes da entrega. As três da bota estão na lista. Quem matou o fazendeiro sabe exatamente onde está o resto.")
                        .req(Req::Clue(1)),
                ]),
            person("Duane", "Rourke", false, 29, Job::Driver, Role::Suspect, B(BKind::House, 6), "Ex-paraquedista, caminhoneiro. Tem um paraquedas na garagem e um temperamento que o FBI adora.")
                .temper(30, 80, 50, 50)
                .topics(vec![
                    topic("paraquedas", "O senhor tem um paraquedas militar na garagem.", "Tenho. Trouxe do Vietnã. Saltei quarenta e duas vezes lá, e nunca mais depois. Vai lá ver: a poeira na capa está intacta.")
                        .lie("Nunca saltei na vida. Esse paraquedas é de um amigo.", 3),
                    topic("noite", "Onde o senhor estava na noite do sequestro?", "No hospital dos veteranos, em Salt Lake City, tirando estilhaço da perna. Tem prontuário, tem enfermeira, tem até foto. O FBI tem cópia de tudo e mesmo assim bate na minha porta.")
                        .reveals(&[12]),
                ]),
            person("Morty", "Feldman", false, 60, Job::Merchant, Role::Witness, B(BKind::Apartment, 5), "Dono de um depósito de metais e sucata industrial na beira do rio. Vende o que for, a quem pagar em dinheiro.")
                .temper(50, 40, 45, 70)
                .topics(vec![
                    topic("compra", "Quem anda comprando metais raros com o senhor?", "Um físico. Hask. Nióbio, bobinas de cobre, uma cúpula de vidro de um metro e setenta. Pagou tudo em notas de vinte, novinhas, na semana depois do sequestro. Eu não pergunto. Mas ele me disse uma coisa esquisita: 'É para um homem que ainda não chegou.'")
                        .reveals(&[13]),
                    topic("cupula", "Uma cúpula de vidro para quê?", "Ele chamou de 'a esfera'. Disse que precisava ficar pronta em Nevada até dezembro, num lugar chamado Delta. Eu vendo metal, senhor. Não vendo perguntas."),
                ]),
            person("June", "Tuttle", true, 47, Job::Housewife, Role::Witness, B(BKind::Farmhouse, 1), "Viúva de Earl. Mulher de poucas palavras e muito medo. Sabe mais do que diz.")
                .temper(80, 35, 70, 30)
                .topics(vec![
                    topic("estranho", "Earl trouxe alguém para casa na noite do sequestro?", "Trouxe. Um homem encharcado, de terno, com um paraquedas enrolado no braço. O relógio da cozinha marcava 3:17. Earl disse que era um piloto perdido. O homem dormiu no celeiro e deixou um envelope. Depois Earl ficou ganancioso. Na segunda, disse que ia a Portland 'cobrar o resto'.")
                        .lie("Ninguém. Earl chegou sozinho, como sempre. Não sei de dinheiro nenhum.", 1)
                        .reveals(&[10]),
                ]),
        ],
        clues: vec![
            clue("Corpo de Earl no celeiro", ClueKind::Physical, Some(HomeOf(0)), Look3d::Blood, "Earl está caído entre os fardos de feno, com um golpe na têmpora.", &[])
                .forensic("O ferimento é perfeitamente circular, com cinco centímetros de diâmetro. Não foi uma chave de roda nem uma pedra: foi algo esférico, pesado e liso."),
            clue("Notas na bota", ClueKind::Physical, Some(HomeOf(0)), Look3d::Object, "Três notas de vinte dólares dobradas no forro da bota de Earl.", &[])
                .forensic("Os números de série começam com L e terminam com a mesma sequência microfilmada pelo FBI."),
            clue("Lista de séries do FBI", ClueKind::Document, Some(B(BKind::Police, 0)), Look3d::Paper, "A lista dos números de série das dez mil notas do resgate, datilografada pelo FBI.", &[])
                .network(Job::Police),
            clue("Paraquedas na garagem", ClueKind::Physical, Some(HomeOf(4)), Look3d::Object, "Um paraquedas militar verde-oliva, dobrado na garagem de Duane Rourke.", &[4])
                .forensic("A poeira sobre a capa está intacta há meses. Esse paraquedas não saiu da garagem.")
                .herring(),
            clue("Gravata da poltrona 18C", ClueKind::Physical, Some(B(BKind::Station, 0)), Look3d::Object, "Uma gravata preta com prendedor de madrepérola, esquecida na poltrona 18C e guardada no aeroporto.", &[1])
                .forensic("Partículas microscópicas de titânio e nióbio no tecido. Metais que só se usam em laboratórios de física de alta energia."),
            clue("Encomenda de nióbio", ClueKind::Document, Some(B(BKind::Warehouse, 0)), Look3d::Paper, "Nota de venda: 20 kg de nióbio, bobinas de cobre supercondutor, uma cúpula de vidro de 1,70 m. Entrega: 'Laboratório D., Nevada'. Pagamento: $14.000 em notas de vinte.", &[1])
                .hidden(1)
                .network(Job::Merchant),
            clue("Projeto da esfera", ClueKind::Document, Some(HomeOf(1)), Look3d::Glow, "Plantas de uma esfera oca de 1,70 m com um filamento vermelho no centro. No canto: 'Ativação: 3:17. — E.V.'", &[1])
                .forensic("A caligrafia das anotações não é a de Hask. É a sua, Elias.")
                .hidden(2),
            clue("Mapa do rio Lewis", ClueKind::Document, Some(HomeOf(1)), Look3d::Paper, "Um mapa aeronáutico com a rota Portland–Seattle. Um X a lápis sobre a margem do rio Lewis, a um quilômetro da fazenda dos Tuttle.", &[1, 0])
                .forensic("Anotações de velocidade, altitude e tempo de queda. O X foi calculado, não adivinhado."),
            clue("Esfera de aço", ClueKind::Physical, Some(Docks), Look3d::Weapon, "Uma esfera de aço maciço, do tamanho de uma laranja, presa no lodo da margem do rio.", &[1])
                .forensic("Sangue e cabelo de Earl. A esfera foi usinada com precisão de laboratório: é um peso de calibração, com o número de série de um galpão alugado por R. Hask.")
                .hidden(1),
            testimony("O homem de óculos escuros", "Tina: o sequestrador era calmo, usava gravata com prendedor de madrepérola, conhecia o avião e disse que para ele era sempre 3:17.", &[1]),
            testimony("O estranho no celeiro", "June: Earl trouxe para casa um homem encharcado com um paraquedas, às 3:17. Depois quis 'cobrar o resto' em Portland.", &[1]),
            testimony("Rourke, o paraquedista", "Himmel: o FBI suspeita de Duane Rourke, ex-paraquedista com um paraquedas na garagem.", &[4]),
            testimony("Rourke em Salt Lake", "Rourke estava num hospital de veteranos em Salt Lake City na noite do sequestro.", &[]),
            testimony("Pago em notas de vinte", "Feldman: Hask comprou nióbio e uma cúpula de vidro em notas de vinte novas, 'para um homem que ainda não chegou'.", &[1]),
            clue("Eco: o celeiro", ClueKind::Temporal, Some(HomeOf(0)), Look3d::None, "No eco, Earl exige cinquenta mil e ameaça ligar para o FBI. O outro homem tira do bolso uma esfera de aço. O relógio de pulso dele marca 3:17 e não anda.", &[1]),
            clue("Notas apodrecidas na margem", ClueKind::Physical, Some(Docks), Look3d::Glow, "Enterrado na areia, um maço de notas de vinte do resgate, podres e desbotadas, como se tivessem passado nove anos no rio. Um cartão de plástico preso ao elástico: 'fevereiro de 1980'.", &[])
                .hidden(3),
        ],
        echoes: vec![
            echo(HomeOf(0), Ghost::Struggle, &["Chuva batendo no telhado do celeiro.", "'Cinquenta mil, ou eu ligo pro FBI.'", "Uma esfera de aço brilha na lanterna.", "Um relógio parado: 3:17."], 14),
        ],
        culprit: 1,
        methods: &[
            "Rourke o matou com uma chave de roda",
            "Golpeado na têmpora com uma esfera de aço do laboratório de Hask, durante uma briga por dinheiro",
            "Acidente com o trator",
            "Afogado no rio e trazido de volta ao celeiro",
        ],
        method: 1,
        motives: &[
            "Vingança de guerra",
            "Ciúme de June",
            "Earl exigiu uma parte maior do resgate e ameaçou entregá-lo ao FBI",
            "Queima de arquivo a mando da máfia de Seattle",
        ],
        motive: 2,
        anomalies: &[
            "Nenhuma — é apenas um crime",
            "O sequestrador nunca existiu",
            "As notas na margem apodreceram nove anos em três dias — e o projeto da esfera traz a sua caligrafia, as iniciais E.V. e a hora 3:17",
            "O avião pousou em outro ano",
        ],
        anomaly: 2,
        threads: &[(1, 0), (1, 2), (1, 5), (0, 6), (3, 4)],
        story: "Russell Hask, físico demitido, recebeu de um homem de sobretudo os desenhos de uma esfera e uma promessa. Para comprar os materiais, sequestrou o voo 305 como 'Dan Colter' e saltou com duzentos mil dólares sobre a margem do rio Lewis. Earl Tuttle o encontrou, o abrigou e aceitou cinco mil pelo silêncio. Três dias depois, Earl quis cinquenta e ameaçou chamar o FBI. Hask o matou no celeiro com um peso de calibração de aço que carregava no bolso e jogou a esfera no rio.",
        sphere: "O homem de sobretudo que encomendou a esfera tinha a sua voz e a sua letra. O Laboratório Delta não foi descoberto em 2025, Elias: ele foi começado em 1971, com dinheiro de resgate, por alguém que você ainda vai ser. As notas na margem só serão encontradas em 1980. Você acaba de vê-las antes do tempo.",
        on_true: outcome(
            "Russell Hask foi preso no galpão, entre bobinas de cobre e uma cúpula de vidro inacabada. O FBI nunca admitiu que ele era 'Dan Colter': o caso do sequestro continuou oficialmente aberto. A cúpula desapareceu do depósito de provas uma semana depois.",
            "FÍSICO É PRESO PELA MORTE DE FAZENDEIRO NO RIO LEWIS",
            "...o FBI não comenta ligação com o sequestro do voo 305...",
            vec![(1, "jailed"), (6, "survived"), (4, "grateful"), (3, "police")],
        )
        .flags(vec!["colter_caught", "delta_seed_seen"]),
        on_false: outcome(
            "Duane Rourke foi preso. O júri o absolveu em dois dias, mas ele perdeu o emprego e a casa. Hask desapareceu em dezembro, rumo a Nevada, com um caminhão fechado. Em 1980, um menino encontrou notas podres na margem do rio.",
            "EX-PARAQUEDISTA É ACUSADO PELA MORTE NO RIO LEWIS",
            "...o paradeiro do sequestrador do voo 305 continua desconhecido...",
            vec![(4, "hates_elias"), (1, "left_city")],
        )
        .flags(vec!["colter_free", "delta_seed_seen"]),
        digit: "1",
        reward: 105,
    }
}

// ------------------------------------------------------------------ 20 — Portland, 1978

pub fn case20() -> CaseDef {
    CaseDef {
        id: 20,
        title: "A Rádio que Transmite do Futuro",
        city: CityId::Portland,
        year: 1978,
        intro: "Portland, 1978. Toda madrugada, no programa 'Linha Aberta' da KPDX, Walt Ferrigan atende ouvintes insones. Há três semanas, às 3:17 em ponto, alguém começou a ligar para descrever crimes. Todos aconteceram na noite seguinte, exatamente como descritos. Ontem, a voz anunciou a morte de um locutor. Hoje de manhã, Walt foi encontrado no estúdio, estrangulado com o cabo dos próprios fones de ouvido. A fita da última ligação sumiu.",
        brief: "O locutor Walt Ferrigan foi estrangulado no estúdio da KPDX. Um ouvinte misterioso prevê crimes às 3:17. A fita da última ligação desapareceu.",
        start: B(BKind::Radio, 0),
        cast: vec![
            person("Walt", "Ferrigan", false, 49, Job::RadioHost, Role::Victim, B(BKind::Apartment, 3), "Voz grave da madrugada de Portland. Vinte anos de rádio. Morto no ar, com o microfone ainda aberto.").dead(),
            person("Nadia", "Sorensen", true, 31, Job::RadioHost, Role::Contact, B(BKind::Apartment, 7), "Produtora do 'Linha Aberta'. Atende as ligações antes de passá-las a Walt. Reconhece Elias antes de ele dizer o nome.")
                .temper(45, 65, 80, 20)
                .soul(1)
                .topics(vec![
                    topic("voz", "Como é a voz que liga às 3:17?", "Cansada. Paciente. Como alguém que já contou a mesma coisa muitas vezes. Na primeira ligação ele disse: 'Diga ao Walt que um policial vai matar um homem no armazém da Burnside, por uma sacola.' Aconteceu. Na última, ele disse o seu nome, Elias. Disse que você chegaria tarde de novo.")
                        .reveals(&[9]),
                    topic("fita", "O que havia na fita 317?", "Todas as ligações do ouvinte. Walt numerou a fita com a hora. Ele ia entregar a um repórter do Oregonian hoje de manhã. Disse que a fita provava quem matou o homem do armazém. E que a voz no telefone era 'a coisa mais estranha que ele já tinha ouvido'."),
                    topic("voce", "Nós já nos conhecemos, Nadia?", "Numa rádio de São Francisco, com fitas chegando às 3:17. Num hotel de Bergen, com neve no porto. Eu sei que não faz sentido. Eu tinha outros nomes. E em todas as vezes você me olhava assim, como quem pede desculpa por algo que ainda vai fazer."),
                ]),
            person("Gary", "Lindqvist", false, 36, Job::Police, Role::Suspect, B(BKind::House, 4), "Policial de patrulha da zona norte. Condecorado, respeitado, com uma casa nova demais para o salário.")
                .temper(20, 85, 15, 80)
                .flees()
                .topics(vec![
                    topic("radio", "O senhor esteve na KPDX ontem à noite?", "Passei lá, sim. Rotina. O locutor andava fazendo acusações no ar e eu fui pedir educadamente que parasse. Quando saí, ele estava vivo e falando. Sempre falando.")
                        .lie("Nunca pus os pés naquela rádio. Estava de patrulha no leste da cidade.", 3),
                    topic("armazem", "O senhor conhecia Ray Suarez, morto no armazém da Burnside?", "Informante de merda. Vendia a mãe por uma dose. Ele sacou primeiro. Se o relatório diz outra coisa, o relatório está errado.")
                        .lie("Nunca ouvi falar. Tem muito viciado morto nesta cidade.", 10)
                        .req(Req::Clue(5)),
                    topic("apreensao", "Faltam dois quilos de heroína na apreensão que o senhor registrou.", "Erro de balança. Acontece. Se o senhor continuar mexendo nisso, forasteiro, vai descobrir que Portland também tem rios fundos.")
                        .req(Req::Clue(6)),
                ]),
            person("Buddy", "Kaminski", false, 40, Job::RadioHost, Role::Suspect, B(BKind::Apartment, 2), "Locutor da manhã da KPDX. Queria o horário de Walt e não escondia isso de ninguém.")
                .temper(50, 50, 45, 60)
                .topics(vec![
                    topic("carta", "O senhor escreveu esta carta a Walt?", "Escrevi. Em junho, bêbado, depois de perder o horário nobre para ele. 'Um dia você vai calar a boca.' Foi uma frase. Eu sou locutor, eu vivo de frases. Eu nunca encostei nele.")
                        .lie("Que carta? Walt e eu éramos amigos.", 4),
                    topic("noite", "Onde o senhor estava às 3:17?", "No Blue Moon Tavern, até fecharem às quatro. Perdi quarenta dólares no pôquer para o dono. Pergunte a ele, ele adora contar."),
                ]),
            person("Loretta", "Pruett", true, 58, Job::Maid, Role::Witness, B(BKind::House, 9), "Faxineira noturna do prédio da KPDX. Viu o que não queria ter visto e tem medo de quem viu.")
                .temper(85, 30, 75, 15)
                .topics(vec![
                    topic("noite", "A senhora estava no prédio naquela madrugada?", "Estava. Às três e quinze, no corredor, tinha um homem parado. Velho, de cabelo grisalho, sobretudo comprido, uma cicatriz no queixo. Não se mexia. Olhava o relógio da parede. Ele tinha a sua cara, moço. Mais velha, cansada, mas a sua. Ele me disse: 'Não entre no estúdio agora, Loretta.' Eu não entrei. Às três e vinte, um policial saiu de lá correndo, com um rolo de fita debaixo do braço.")
                        .lie("Não vi nada. Eu limpo e vou embora. Não quero confusão com a polícia.", 13)
                        .reveals(&[8]),
                    topic("homem", "Para onde foi o homem de sobretudo?", "Para lugar nenhum. Eu pisquei e o corredor estava vazio. Só ficou um cheiro de ozônio, como depois de um raio.")
                        .req(Req::Topic("noite")),
                ]),
            person("Hal", "Brody", false, 50, Job::Detective, Role::Contact, B(BKind::House, 11), "Detetive de homicídios. Prefere um culpado civil a um escândalo na corporação.")
                .temper(40, 55, 50, 45)
                .topics(vec![
                    topic("suspeito", "A polícia tem um suspeito?", "Buddy Kaminski. Queria o horário de Walt, mandou uma carta ameaçadora, tem temperamento. Caso simples. É assim que o chefe quer.")
                        .reveals(&[11]),
                    topic("patrulha", "Este botão de farda estava embaixo da mesa de som.", "...Isso complica. O rádio da central registra as viaturas. A de Gary Lindqvist ficou fora da rota das 3:10 às 3:30, estacionada a uma quadra da KPDX. Eu não disse isso ao senhor.")
                        .req(Req::Clue(3))
                        .reveals(&[12]),
                ]),
            person("Lupe", "Suarez", true, 29, Job::Waiter, Role::Witness, B(BKind::Apartment, 5), "Garçonete de uma lanchonete 24 horas. Irmã de Ray Suarez, o homem morto no armazém da Burnside.")
                .temper(55, 70, 75, 20)
                .topics(vec![
                    topic("ray", "O que Ray fazia no armazém da Burnside?", "Ray era informante da Corregedoria. Tinha visto um policial, Lindqvist, tirar heroína das apreensões e revender na rua. Naquela noite, ia buscar a prova numa sacola escondida no armazém. Ele ouviu o programa do Walt, sabia que a voz tinha avisado. Foi mesmo assim. Ray nunca teve medo da hora certa.")
                        .reveals(&[10]),
                ]),
        ],
        clues: vec![
            clue("Corpo de Walt no estúdio", ClueKind::Physical, Some(B(BKind::Radio, 0)), Look3d::Blood, "Walt está caído sobre a mesa de som, com o cabo dos fones enrolado no pescoço. O microfone ficou aberto: a cidade ouviu três minutos de silêncio.", &[2])
                .forensic("O cabo foi puxado por trás, com o antebraço contra a nuca — a técnica de imobilização ensinada na academia de polícia de Portland."),
            clue("Carretel vazio", ClueKind::Physical, Some(B(BKind::Radio, 0)), Look3d::Object, "O carretel da fita 317 está vazio no gravador. A fita foi arrancada às pressas.", &[])
                .forensic("Pedaços de fita magnética presos no cabeçote. Alguém puxou a fita com o gravador ainda rodando."),
            clue("Livro de chamadas", ClueKind::Document, Some(B(BKind::Radio, 0)), Look3d::Paper, "O caderno de Nadia com todas as ligações. Três semanas seguidas, 3:17: 'Ouvinte sem nome'. Ao lado da primeira: 'policial — armazém da Burnside — sacola'.", &[2])
                .forensic("Na linha de ontem, com a letra de Nadia: 'ele disse que o locutor morre às 3:17 e que Elias chega tarde'."),
            clue("Botão de farda", ClueKind::Physical, Some(B(BKind::Radio, 0)), Look3d::Object, "Um botão de metal da farda da polícia de Portland, embaixo da mesa de som.", &[2])
                .forensic("Ainda tem um fio azul-marinho preso. Arrancado numa luta, não perdido.")
                .hidden(1),
            clue("Carta de Buddy", ClueKind::Document, Some(HomeOf(3)), Look3d::Paper, "'Um dia você vai calar essa boca, Walt.' Assinada: Buddy.", &[3])
                .forensic("Datada de junho. Papel com mancha de uísque. Nenhuma outra carta depois disso.")
                .herring(),
            clue("Sangue no armazém da Burnside", ClueKind::Physical, Some(B(BKind::Warehouse, 0)), Look3d::Blood, "Uma mancha de sangue seco e uma sacola de lona rasgada no chão do armazém onde Ray Suarez morreu.", &[2])
                .forensic("A bala tirada da parede é .38 Special, munição de serviço da polícia. Ray estava desarmado: não há resíduo de pólvora nas mãos dele."),
            clue("Registro de apreensões", ClueKind::Document, Some(B(BKind::Police, 0)), Look3d::Paper, "Apreensão de 4 kg de heroína, registrada pelo patrulheiro G. Lindqvist. No depósito de provas, só há 2 kg.", &[2])
                .hidden(2)
                .network(Job::Police),
            clue("Fita 317", ClueKind::Physical, Some(HomeOf(2)), Look3d::Object, "No fundo da lareira de Lindqvist, um rolo de fita magnética meio queimado.", &[2])
                .forensic("Sobraram quarenta segundos. Uma voz masculina, cansada: 'São 3:17. Um policial vai matar Walt às 3:17 de amanhã. Nadia, diga ao Elias que eu sinto muito.' A voz é a sua, Elias. Mais velha.")
                .hidden(1),
            testimony("O homem de sobretudo", "Loretta viu um homem grisalho de sobretudo, com o rosto de Elias mais velho, parado no corredor às 3:15. Às 3:20, um policial saiu do estúdio com um rolo de fita.", &[2]),
            testimony("A voz de madrugada", "Nadia: a voz cansada liga sempre às 3:17. Na última ligação, disse o nome de Elias e que ele chegaria tarde.", &[]),
            testimony("Ray era informante", "Lupe: Ray era informante da Corregedoria e ia buscar a prova de que Lindqvist revendia heroína apreendida.", &[2]),
            testimony("Buddy queria o horário", "Brody: a polícia quer Buddy Kaminski, que queria o horário de Walt e mandou uma carta ameaçadora.", &[3]),
            testimony("A viatura fora da rota", "Brody: a viatura de Lindqvist ficou parada a uma quadra da KPDX das 3:10 às 3:30.", &[2]),
            clue("Eco: o estúdio às 3:17", ClueKind::Temporal, Some(B(BKind::Radio, 0)), Look3d::None, "No eco, um policial entra por trás de Walt e puxa o cabo dos fones. No vidro do estúdio, um homem de sobretudo observa do corredor, imóvel. O relógio marca 3:17.", &[2]),
            clue("Eco: o armazém da Burnside", ClueKind::Temporal, Some(B(BKind::Warehouse, 0)), Look3d::None, "No eco, Ray segura uma sacola de lona. Um policial aponta o revólver: 'Me dá isso, Suarez.' Um estampido.", &[2]),
            clue("Bilhete no bolso do sobretudo", ClueKind::Document, Some(B(BKind::Radio, 0)), Look3d::Glow, "Preso atrás do relógio do corredor, um papel dobrado com a sua letra: 'Não dá para salvar o Walt. Salve a fita. Salve a Nadia. Delta, 1971 — você já sabe o caminho.'", &[])
                .hidden(3),
        ],
        echoes: vec![
            echo(B(BKind::Radio, 0), Ghost::Struggle, &["A luz vermelha: NO AR.", "'Linha Aberta, boa noite, você está no ar...'", "Um cabo se fecha em volta de um pescoço.", "No vidro, um homem de sobretudo. Ele não se mexe.", "3:17."], 13),
            echo(B(BKind::Warehouse, 0), Ghost::Argue, &["Goteiras no telhado de zinco.", "'Me dá a sacola, Suarez.'", "'A rádio avisou que você vinha.'", "Um estampido. A sacola cai."], 14),
        ],
        culprit: 2,
        methods: &[
            "Buddy o golpeou com um microfone",
            "Estrangulado por trás com o cabo dos fones de ouvido, com técnica de imobilização policial; a fita 317 foi levada",
            "Ataque cardíaco durante o programa",
            "Envenenado no café da madrugada",
        ],
        method: 1,
        motives: &[
            "Ficar com o horário da madrugada",
            "Roubo do equipamento da rádio",
            "Recuperar a fita 317, que provava que um policial havia matado o informante Ray Suarez",
            "Ciúme de Nadia",
        ],
        motive: 2,
        anomalies: &[
            "Nenhuma — é apenas um crime",
            "O ouvinte era um vidente de verdade",
            "Walt forjou as ligações para ganhar audiência",
            "A voz que liga às 3:17 é a sua, mais velha — e o homem de sobretudo no corredor é o Outro Elias",
        ],
        anomaly: 3,
        threads: &[(2, 0), (2, 6), (1, 0), (4, 2), (5, 2), (3, 0)],
        story: "Gary Lindqvist desviava heroína das apreensões e a revendia na rua. Ray Suarez, informante da Corregedoria, ia buscar a prova num armazém da Burnside quando Lindqvist o matou — exatamente como uma voz tinha anunciado no rádio na noite anterior. Walt Ferrigan percebeu que a fita com as ligações provava tudo e ia entregá-la a um jornal. Às 3:17, Lindqvist entrou no estúdio, estrangulou Walt com o cabo dos fones e levou a fita 317 para queimar em casa.",
        sphere: "A voz que avisava os crimes era a sua. O homem de sobretudo no corredor também. O Outro Elias não salva ninguém: ele assiste, liga, avisa — e deixa acontecer, porque cada morte às 3:17 é um ponto de costura que ele não pode desfazer. Agora você sabe que ele existe. E ele sabe que você sabe.",
        on_true: outcome(
            "Gary Lindqvist foi preso na delegacia, diante dos colegas. A Corregedoria reabriu o caso de Ray Suarez. Nadia assumiu o 'Linha Aberta'. Às 3:17, toda madrugada, ela deixa a linha um minuto em silêncio. Ninguém mais ligou.",
            "POLICIAL É PRESO PELA MORTE DO LOCUTOR DA KPDX E DE INFORMANTE",
            "...a KPDX volta ao ar esta noite, com Nadia Sorensen no comando da madrugada...",
            vec![(2, "jailed"), (1, "grateful"), (6, "grateful"), (3, "survived"), (5, "police")],
        )
        .flags(vec!["radio_solved", "other_elias_seen"]),
        on_false: outcome(
            "Buddy Kaminski foi preso e condenado com base numa carta de junho. Lindqvist foi promovido a sargento. Nadia deixou Portland. No último dia, ela atendeu uma ligação às 3:17. A voz disse apenas: 'De novo, Elias.'",
            "LOCUTOR DA MANHÃ É CONDENADO PELA MORTE DE WALT FERRIGAN",
            "...a polícia de Portland encerra o caso do estúdio...",
            vec![(3, "jailed"), (2, "police"), (1, "left_city"), (6, "hates_elias")],
        )
        .flags(vec!["radio_false", "other_elias_seen"]),
        digit: "1",
        reward: 110,
    }
}
