//! ERA II — 1930–1945 — Baviera (Hinterfeld) e Londres (Blitz).

use super::defs::*;
use crate::city::gen::CityId;
use crate::city::map::BKind;
use crate::sim::people::Job;
use Place::*;

pub fn cases() -> Vec<CaseDef> {
    vec![case06(), case07(), case08(), case09(), case10()]
}

pub fn case06() -> CaseDef {
    CaseDef {
        id: 6,
        title: "A Fazenda de Hinterfeld",
        city: CityId::Bavaria,
        year: 1932,
        intro: "Baviera, 1932. Na fazenda dos Haldenmayr, nos arredores de Hinterfeld, seis pessoas foram mortas no celeiro e na casa. Ninguém percebeu por quatro dias: a chaminé fumegava, as vacas estavam alimentadas, o cachorro amarrado. Na neve, pegadas saem da floresta e vão até a fazenda. Nenhuma pegada volta.",
        brief: "Seis mortos na fazenda Haldenmayr. O gado foi tratado depois das mortes. Pegadas chegam da floresta, nenhuma sai. No sótão, alguém viveu por meses.",
        start: HomeOf(0),
        cast: vec![
            person("Andreas", "Haldenmayr", false, 63, Job::Farmer, Role::Victim, B(BKind::Farmhouse, 0), "O patriarca. Duro, avarento, temido no vilarejo. Encontrado no celeiro sob o feno.").dead(),
            person("Viktoria", "Reiter", true, 35, Job::Housewife, Role::Victim, B(BKind::Farmhouse, 0), "Filha de Andreas, 'viúva de guerra'. Cantava no coro da igreja. Encontrada no celeiro.").dead(),
            person("Lorenz", "Brandl", false, 45, Job::Farmer, Role::Suspect, B(BKind::Farmhouse, 2), "Vizinho. Foi quem arrombou o celeiro e encontrou os corpos. Registrou-se como pai do filho caçula de Viktoria.")
                .temper(55, 50, 55, 40)
                .topics(vec![
                    topic("descoberta", "Como o senhor encontrou os corpos?", "...Eu entrei. Tirei o feno de cima deles com as mãos. Procurava o menino, o pequeno Sepp. Mexi em tudo antes de chamar a gendarmaria. Se isso me faz culpado, então sou.")
                        .lie("Só abri a porta, vi o sangue e corri chamar a gendarmaria. Não toquei em nada.", 15)
                        .reveals(&[9]),
                    topic("menino", "Dizem que o filho caçula de Viktoria era seu.", "Assinei como pai para calar o padre e as comadres. Mas o menino não era meu. Todo mundo em Hinterfeld sabe de quem era. E ninguém diz em voz alta."),
                    topic("carta", "E esta carta, exigindo casar com Viktoria?", "Escrevi bêbado, no ano passado. Ela riu na minha cara. Eu fiquei com raiva, sim. Mas raiva de homem sóbrio passa. Aquilo no celeiro não passa.").req(Req::Clue(6)),
                ]),
            person("Anton", "Weiss", false, 24, Job::Drifter, Role::Suspect, B(BKind::Hotel, 0), "Forasteiro hospedado na estalagem. Diz ser peão à procura de trabalho. Manca da perna esquerda e paga em moedas antigas.")
                .temper(35, 65, 25, 30)
                .flees()
                .topics(vec![
                    topic("chegada", "Quando o senhor chegou a Hinterfeld?", "...Novembro. Vim pela floresta, na primeira neve. Dormi onde pude. Num sótão quente, que cheirava a feno e a ela. O senhor sabe de quem eu falo.")
                        .lie("Terça-feira passada. Vim de Ingolstadt, de trem e depois a pé. Nunca pisei naquela fazenda.", 0),
                    topic("guerra", "O senhor serviu na guerra?", "Regimento de Infantaria Bávaro nº 12. Arras, 1915. Uma noite o céu ficou roxo, às três e dezessete. Fechei os olhos na trincheira e abri na floresta daqui. Um lenhador me disse que era 1931. Meu nome é Karl Reiter.")
                        .lie("Nunca fui soldado. Sou peão, trabalho com gado desde menino.", 4),
                    topic("viktoria", "Viktoria recebia pensão de viúva pela sua morte.", "Trinta e oito marcos por mês pela minha morte. E o velho dormindo no meu lugar. O menino tinha os olhos dele, não os meus. Eu fiquei no sótão ouvindo tudo por meses. Uma noite eu desci.").req(Req::Clue(5)),
                ]),
            person("Matthias", "Kögl", false, 58, Job::Priest, Role::Witness, B(BKind::House, 3), "Pároco de Hinterfeld. Casou Viktoria e Karl em 1913. Guarda segredos de confissão como quem guarda brasas.")
                .temper(60, 45, 75, 15)
                .topics(vec![
                    topic("confissao", "Viktoria confessou algo nas últimas semanas?", "O segredo da confissão é sagrado... mas ela está morta, e Deus me perdoe. Viktoria disse que via o rosto do marido na janela à noite. O marido morto em Arras. E que o pai... que o pai tinha pecado com ela por anos. Ela pediu perdão por coisas que não eram culpa dela.").reveals(&[10]),
                    topic("doacao", "Viktoria deu dinheiro à igreja?", "Setecentos Reichsmark, em outubro. 'Para a alma do Karl', disse. Como se soubesse que a alma dele ainda andava por aí."),
                    topic("rosto", "O senhor viu o forasteiro da estalagem?", "Vi. E rezei o terço inteiro depois. Aquele rapaz tem o rosto do Karl Reiter no dia do casamento. Exatamente aquele rosto. Vinte e quatro anos. Como se o tempo não tivesse passado por ele.").req(Req::Topic("confissao")),
                ]),
            person("Kreszenz", "Obermaier", true, 50, Job::Maid, Role::Witness, B(BKind::House, 6), "Ex-empregada dos Haldenmayr. Largou o serviço em dezembro sem dar explicação.")
                .temper(80, 30, 70, 30)
                .topics(vec![
                    topic("saida", "Por que a senhora deixou a fazenda?", "Por causa dos passos. No sótão, toda noite. Alguém andando devagar, parando em cima do quarto da Viktoria. E um jornal de Munique na cozinha que ninguém comprou. O velho disse que eram martas. Marta não lê jornal.")
                        .lie("O salário era pouco. O velho era sovina. Só isso.", 2)
                        .reveals(&[7]),
                    topic("manco", "A senhora viu alguém estranho perto da fazenda?", "Em outubro, na beira da floresta. Um rapaz de capote cinza de soldado, mancando da perna esquerda. Olhava para a casa como quem olha para a própria casa.").reveals(&[11]),
                ]),
            person("Xaver", "Huber", false, 30, Job::Clerk, Role::Witness, B(BKind::House, 8), "O carteiro de Hinterfeld. Passa na fazenda todo sábado.")
                .temper(50, 55, 65, 35)
                .topics(vec![
                    topic("correio", "O senhor passou na fazenda depois das mortes?", "No sábado e na segunda. A chaminé fumegava nos dois dias. O jornal da caixa foi recolhido, mas a carta do banco ficou. Pensei: estão todos em casa. Estavam. Só que mortos desde sexta.").reveals(&[8]),
                    topic("estalagem", "O que sabe do forasteiro Weiss?", "Paga a estalagem com moedas de marco do Império. De antes da inflação! Ninguém tem mais disso. O estalajadeiro aceitou porque é prata."),
                ]),
        ],
        clues: vec![
            clue("Pegadas na neve", ClueKind::Physical, Some(Woods), Look3d::Prints, "Pegadas de botas saem da floresta e vão até os fundos da fazenda. Não há nenhuma pegada no caminho de volta.", &[3])
                .forensic("Botas militares de 1914, sola com cravos. O peso cai mais sobre a perna direita: o dono manca da esquerda. E as pegadas não começam na estrada: começam no meio de uma clareira, do nada."),
            clue("Picareta ensanguentada", ClueKind::Physical, Some(B(BKind::Barn, 0)), Look3d::Weapon, "Uma picareta de cavar raízes, escondida sob as tábuas do celeiro. Sangue seco no ferro.", &[3])
                .forensic("O cabo está gasto no lado de quem segura com a mão esquerda. Há farpas de palha do sótão presas no sangue."),
            clue("Ninho no sótão", ClueKind::Physical, Some(HomeOf(0)), Look3d::Object, "No sótão, um ninho de feno amassado, cascas de pão, tocos de vela e um buraco no assoalho que dá para o quarto de Viktoria.", &[3])
                .forensic("Jornais de Munique datados de novembro a março, empilhados em ordem. Alguém morou aqui por pelo menos quatro meses.")
                .hidden(1),
            clue("Gado alimentado", ClueKind::Physical, Some(B(BKind::Barn, 0)), Look3d::Object, "As vacas estão saciadas, o cocho cheio, o esterco limpo. Alguém cuidou dos animais por dias depois das mortes.", &[3]),
            clue("Plaqueta de identificação", ClueKind::Physical, Some(B(BKind::Hotel, 0)), Look3d::Object, "Escondida no forro do quarto de Weiss na estalagem: 'K. REITER — B.I.R. 12 — 1914'. O metal está limpo, como novo.", &[3])
                .forensic("Não há um grão de oxidação. Uma plaqueta de dezessete anos atrás deveria estar negra.")
                .hidden(2),
            clue("Certidão de morte e pensão", ClueKind::Document, Some(B(BKind::Office, 0)), Look3d::Paper, "Karl Reiter, dado como morto em Arras em 1915. Pensão de viúva de guerra: 38 RM mensais, pagos a Viktoria até este mês.", &[0, 1])
                .network(Job::Clerk),
            clue("Carta de Brandl", ClueKind::Document, Some(HomeOf(2)), Look3d::Paper, "'Case comigo, Viktoria, ou o vilarejo inteiro vai saber de quem é o menino.' Assinada: Lorenz.", &[2])
                .forensic("O papel é do ano passado. Tinta desbotada, dobrada e redobrada. Nunca foi enviada.")
                .herring(),
            testimony("Passos no sótão", "Kreszenz ouvia passos no sótão toda noite e achou um jornal de Munique que ninguém comprou.", &[3]),
            testimony("Chaminé fumegando", "O carteiro viu a chaminé fumegar no sábado e na segunda, dias depois das mortes. O jornal foi recolhido.", &[3]),
            testimony("Brandl mexeu nos corpos", "Brandl entrou no celeiro e moveu os corpos procurando o menino antes de chamar a gendarmaria.", &[2]),
            testimony("A confissão de Viktoria", "O padre: Viktoria via o rosto do marido morto na janela, e o pai abusava dela havia anos.", &[0, 3]),
            testimony("O soldado manco", "Kreszenz viu em outubro um rapaz de capote de soldado, mancando, olhando a casa da beira da floresta.", &[3]),
            clue("Eco: o celeiro", ClueKind::Temporal, Some(B(BKind::Barn, 0)), Look3d::None, "No eco, um vulto desce do sótão e chama um por um ao celeiro, imitando o mugido de uma vaca doente. A picareta sobe e desce.", &[3]),
            clue("Eco: o sótão", ClueKind::Temporal, Some(HomeOf(0)), Look3d::None, "No eco, um rapaz magro deitado no feno, olho no buraco do assoalho, sussurra 'Viktoria' toda noite. O relógio da sala bate 3:17.", &[3]),
            clue("Círculo queimado na clareira", ClueKind::Physical, Some(Woods), Look3d::Glow, "Onde as pegadas começam, um círculo perfeito de neve derretida e cinza roxa. No centro, um botão de farda de 1915.", &[3])
                .hidden(2),
            clue("Lenço de Brandl no celeiro", ClueKind::Physical, Some(B(BKind::Barn, 0)), Look3d::Blood, "Um lenço bordado 'L.B.', manchado de sangue, caído perto dos corpos.", &[2])
                .forensic("O sangue foi limpo das mãos, não espirrado. Quem o usou tocou os corpos depois que já estavam frios."),
        ],
        echoes: vec![
            echo(B(BKind::Barn, 0), Ghost::Struggle, &["Um mugido falso vem do celeiro.", "Uma lanterna entra. Depois outra.", "A picareta sobe no escuro.", "Um homem manco espalha feno sobre os corpos."], 12),
            echo(HomeOf(0), Ghost::Hide, &["Tábuas rangem no teto.", "Um olho no buraco do assoalho.", "'Viktoria... sou eu. Voltei.'", "O relógio da sala: 3:17."], 13),
        ],
        culprit: 3,
        methods: &[
            "Brandl atacou a família com um machado de lenha por ciúme de Viktoria",
            "Escondido no sótão por meses, atraiu cada um ao celeiro e os matou com a picareta; depois tratou do gado e viveu na casa",
            "Envenenamento do leite, depois os corpos foram arrastados ao celeiro",
            "Assaltantes vindos de Munique atrás das economias da família",
        ],
        method: 1,
        motives: &[
            "Herdar a fazenda dos Haldenmayr",
            "Voltou de uma guerra que o dava como morto e encontrou o sogro no seu lugar, a esposa recebendo pensão pela sua morte e um filho que não era seu",
            "Roubar os Reichsmark escondidos no colchão",
            "Brandl queria o menino só para si",
        ],
        motive: 1,
        anomalies: &[
            "Nenhuma — é apenas um crime",
            "Karl Reiter sumiu numa trincheira de Arras às 3:17 de 1915 e saiu de uma clareira de Hinterfeld em 1931 sem envelhecer um dia — as pegadas começam do nada",
            "O gado foi alimentado por fantasmas",
            "Os seis mortos já estavam mortos em 1915",
        ],
        anomaly: 1,
        threads: &[(3, 1), (3, 0), (0, 1), (2, 1), (4, 1), (5, 3), (6, 3)],
        story: "Karl Reiter, dado como morto em Arras, reapareceu na floresta de Hinterfeld dezesseis anos depois. Escondeu-se no sótão da própria casa e, por meses, ouviu tudo: a pensão paga pela sua morte, o sogro que tomara seu lugar, o menino que não era seu. Numa sexta-feira de março, atraiu cada um ao celeiro com a picareta. Depois cuidou do gado, acendeu a lareira e comeu na mesa dos mortos por quatro dias, antes de descer ao vilarejo como o forasteiro Anton Weiss.",
        sphere: "As pegadas não voltam porque não vieram de lugar nenhum: a esfera tocou a trincheira de Arras às 3:17 e devolveu Karl dezesseis anos depois, numa clareira da Baviera. Você não é o único que ela arrasta. E nem todos que ela arrasta chegam inteiros.",
        on_true: outcome(
            "Karl Reiter foi preso na estalagem, ainda com as botas de 1914. No tribunal de Augsburgo, os peritos juraram que o réu tinha vinte e quatro anos. A certidão dizia quarenta e um. Brandl pagou o enterro das seis vítimas.",
            "HINTERFELD: SOLDADO 'MORTO' EM ARRAS É PRESO PELO MASSACRE DA FAZENDA",
            "...o réu insiste que ainda estamos em 1915...",
            vec![(3, "jailed"), (2, "survived"), (4, "grateful")],
        )
        .flags(vec!["hinterfeld_solved"]),
        on_false: outcome(
            "O acusado foi levado a Munique entre cusparadas. Na primavera, o forasteiro manco sumiu da estalagem sem pagar. Na clareira, as pegadas agora vão para dentro da floresta, e param no meio dela.",
            "MASSACRE DE HINTERFELD: POLÍCIA ANUNCIA CULPADO",
            "...o caso Haldenmayr continua a assombrar a Alta Baviera...",
            vec![(3, "left_city"), (2, "hates_elias")],
        )
        .flags(vec!["hinterfeld_unsolved"]),
        digit: "9",
        reward: 70,
    }
}

pub fn case07() -> CaseDef {
    CaseDef {
        id: 7,
        title: "O Sino da Meia-Noite",
        city: CityId::Bavaria,
        year: 1936,
        intro: "Baviera, 1936. Há quatro noites o sino da igreja de Hinterfeld toca sozinho. Não à meia-noite, como dizem as comadres: às 3:17 em ponto. Florian Mayr, o sacristão, sumiu na primeira dessas noites. O novo prefeito diz que ele fugiu para a Suíça com a filha do farmacêutico judeu. A filha do farmacêutico ainda está aqui.",
        brief: "O sacristão Florian Mayr sumiu. O sino toca sozinho às 3:17 desde então. O prefeito culpa a família Rosenthal, cuja farmácia ele quer tomar.",
        start: B(BKind::Church, 0),
        cast: vec![
            person("Florian", "Mayr", false, 19, Job::Student, Role::Victim, B(BKind::House, 2), "Sacristão da igreja. Tocava o sino desde menino. Sumiu há quatro noites.").dead(),
            person("Matthias", "Kögl", false, 62, Job::Priest, Role::Witness, B(BKind::House, 3), "O pároco, quatro anos mais velho e mais cansado. Criou Florian como filho.")
                .temper(65, 50, 80, 10)
                .topics(vec![
                    topic("noite", "O que o senhor ouviu na noite em que Florian sumiu?", "Vozes na torre, às três. Uma era do Florian. A outra gritava 'Judenknecht', lacaio de judeu. Depois um barulho de saco caindo na escada. Eu... eu fiquei na cama. Deus sabe que fiquei na cama.")
                        .lie("Nada. Durmo pesado, sou velho.", 14)
                        .reveals(&[12]),
                    topic("sino", "O sino realmente toca sozinho?", "Tirei o badalo ontem, com as minhas mãos. Às três e dezessete ele tocou mesmo assim. Três badaladas. Era o sinal do Florian: três quer dizer 'fujam'."),
                    topic("hinterfeld", "O senhor se lembra da fazenda Haldenmayr?", "Como esquecer? Enterrei seis pessoas numa manhã. E o senhor... o senhor estava lá, não estava? Com esse mesmo casaco. Não mudou nada em quatro anos. Nem o Karl mudava."),
                ]),
            person("Egon", "Rautenberg", false, 44, Job::Politician, Role::Suspect, B(BKind::Mansion, 0), "O novo prefeito, nomeado pelo partido. Broche na lapela, anel de sinete, sorriso de funcionário.")
                .temper(25, 70, 10, 85)
                .flees()
                .topics(vec![
                    topic("noite", "Onde o senhor estava na madrugada em que Florian sumiu?", "...Subi à torre, sim. Para mandar o moleque parar com aquele sino. Ele me chamou de coisa que não se repete. Eu o segurei pelo colarinho, ele escorregou. A escada é íngreme. Foi um acidente, está entendendo? Um acidente.")
                        .lie("Na reunião do partido, na estalagem, até as quatro. Vinte camaradas me viram.", 13),
                    topic("florian", "O que aconteceu com Florian?", "Um acidente, já disse. O Böhm cuidou do resto. Ninguém sentiria falta de um sacristão que protegia judeus.")
                        .lie("Fugiu para a Suíça com a judiazinha. Todos sabem.", 7),
                    topic("farmacia", "Por que a farmácia dos Rosenthal foi vendida por 800 RM?", "Negócio legal, assinado no cartório. O velho Rosenthal quis vender. Meu cunhado pagou o preço justo — para um judeu.").req(Req::Clue(5)),
                ]),
            person("Kurt", "Böhm", false, 33, Job::Police, Role::Suspect, B(BKind::House, 5), "Gendarme do vilarejo. Entrou para as SA no ano passado. Não olha nos olhos.")
                .temper(70, 40, 35, 50)
                .topics(vec![
                    topic("carrinho", "O carrinho do cemitério tem a marca da gendarmaria.", "O prefeito bateu na minha porta às três e meia. O rapaz estava no pé da escada da torre, com o pescoço torto. Ele mandou levar para o ossuário e cobrir com os ossos velhos. Eu obedeci. Eu sempre obedeço.")
                        .lie("Não sei de carrinho nenhum. Deve ter sido o coveiro.", 8)
                        .reveals(&[11]),
                    topic("ordens", "Quem dá ordens ao senhor?", "O Estado. O partido. O prefeito. Hoje em dia é tudo a mesma coisa, forasteiro. Aconselho o senhor a lembrar disso."),
                ]),
            person("Lea", "Rosenthal", true, 20, Job::Pharmacist, Role::Witness, B(BKind::House, 7), "Filha do farmacêutico Samuel Rosenthal. Atende no balcão desde que o pai adoeceu. Amava Florian.")
                .temper(70, 60, 80, 15)
                .topics(vec![
                    topic("sino", "Por que Florian tocava o sino de madrugada?", "Para nos avisar. Quando os caminhões das SA vinham da cidade, ele via da torre. Uma badalada: estão na estrada. Três: fujam. Ele salvou meu pai duas vezes.").reveals(&[9]),
                    topic("florian", "Vocês iam fugir juntos?", "Ele queria. Eu não podia deixar meu pai. Florian disse que ia ficar tocando o sino até o último judeu sair de Hinterfeld. Não fugiu. Ele nunca fugiria."),
                    topic("soda", "Há um frasco de soda cáustica na farmácia.", "Está lacrado. Pode abrir, pode cheirar. O prefeito disse à gendarmaria que eu lavei sangue com ele. Nem sangue houve aqui.").req(Req::Clue(6)),
                ]),
            person("Hedwig", "Lanz", true, 49, Job::Housewife, Role::Witness, B(BKind::House, 9), "Vizinha da igreja. Chefe de quarteirão do partido. Sabe quem acende a luz depois das dez.")
                .temper(60, 40, 25, 60)
                .topics(vec![
                    topic("denuncia", "A senhora denunciou Florian?", "Denunciei. É meu dever. Escrevi que o sacristão tocava o sino para avisar os judeus. Naquela noite eu mesma fui chamar o prefeito. Ele subiu a torre às três. Eu voltei para casa. Não vi mais nada, não quis ver.")
                        .lie("Eu não denuncio ninguém. Sou uma dona de casa.", 4)
                        .reveals(&[10]),
                ]),
            person("Johanna", "Brunner", true, 29, Job::Teacher, Role::Contact, B(BKind::House, 11), "Professora da escola do vilarejo. Recusou-se a pendurar o retrato do Führer na sala. Por enquanto.")
                .temper(35, 80, 85, 15)
                .soul(1)
                .topics(vec![
                    topic("prefeito", "O prefeito diz que estava na reunião do partido.", "Eu não durmo, Elias. Às três eu estava na janela corrigindo cadernos. O carro do prefeito estava parado atrás da igreja. Motor frio. Ele não estava em reunião nenhuma.").reveals(&[13]),
                    topic("voce", "Você disse meu nome. Eu não me apresentei.", "Disse? ...Às vezes eu sei nomes antes de ouvir. O seu eu sei desde menina. Sonhava com um homem de casaco numa redação de jornal, cheia de fumaça. Você morria no sonho. Às vezes era eu."),
                    topic("escola", "O que ensinam às crianças agora?", "A cantar marchas e a apontar para as casas certas. Eu ensino os números. O número que as crianças mais desenham, não sei por quê, é três-um-sete."),
                ]),
        ],
        clues: vec![
            clue("Corda do sino ensanguentada", ClueKind::Physical, Some(B(BKind::Church, 0)), Look3d::Blood, "A corda do sino tem sangue seco a um metro do chão. Alguém se agarrou a ela ao cair.", &[2])
                .forensic("Entre as fibras: fios de lã cinza-escura de sobretudo caro e um botão dourado com a águia do partido."),
            clue("Degrau da torre raspado", ClueKind::Physical, Some(B(BKind::Church, 0)), Look3d::Prints, "Um degrau da escada da torre foi raspado com faca. Na parede, marcas de dedos arrastadas para baixo.", &[])
                .hidden(1),
            clue("Mecanismo do relógio parado", ClueKind::Physical, Some(B(BKind::Church, 0)), Look3d::Glow, "O relógio da torre está desligado há dias, ponteiros em 3:17. O badalo foi retirado — mas o bronze do sino está quente.", &[])
                .forensic("Riscado na madeira do mecanismo, com letra adulta e nervosa: 'E.V. — ainda não. Espere 1971.'")
                .hidden(2),
            clue("Caderno de sinais de Florian", ClueKind::Document, Some(HomeOf(0)), Look3d::Paper, "'1 badalada: caminhões na estrada. 3 badaladas: fujam.' Seguem datas de batidas das SA, e ao lado de cada uma: 'Rosenthal — salvos'.", &[4]),
            clue("Relatórios de quarteirão", ClueKind::Document, Some(B(BKind::Office, 0)), Look3d::Paper, "Relatórios assinados 'H. L.': 'O sacristão avisa os judeus com o sino.' Na margem, com a letra do prefeito: 'Resolver antes de domingo.'", &[2, 5])
                .hidden(1)
                .network(Job::Police),
            clue("Contrato de venda da farmácia", ClueKind::Document, Some(B(BKind::Pharmacy, 0)), Look3d::Paper, "A farmácia Rosenthal 'vendida' ao cunhado do prefeito por 800 RM. O imóvel vale mais de quinze mil. Assinatura de Samuel Rosenthal trêmula.", &[2]),
            clue("Frasco de soda cáustica", ClueKind::Physical, Some(B(BKind::Pharmacy, 0)), Look3d::Object, "O prefeito aponta este frasco como prova de que Lea lavou sangue na farmácia.", &[4])
                .forensic("O lacre de cera está intacto. Ninguém abriu este frasco desde que saiu da fábrica.")
                .herring(),
            clue("Corpo no ossuário", ClueKind::Physical, Some(Cemetery), Look3d::Blood, "Sob as caveiras empilhadas do ossuário, o corpo de Florian, ainda de batina de sacristão.", &[2, 3])
                .forensic("Pescoço quebrado, compatível com queda de escada. No peito, um hematoma com o desenho de um anel de sinete.")
                .hidden(2),
            clue("Carrinho de mão do cemitério", ClueKind::Physical, Some(Cemetery), Look3d::Object, "Um carrinho de mão com a marca da gendarmaria pintada no cabo. Terra da torre nas rodas.", &[3]),
            testimony("O sino avisava", "Lea: Florian tocava o sino para avisar os Rosenthal das batidas das SA. Três badaladas: fujam.", &[]),
            testimony("A denúncia de Hedwig", "Hedwig denunciou Florian e foi chamar o prefeito, que subiu à torre às três.", &[2, 5]),
            testimony("Böhm carregou o corpo", "Böhm levou o corpo de Florian ao ossuário por ordem do prefeito, às três e meia.", &[2, 3]),
            testimony("A voz na torre", "O padre ouviu uma voz gritar 'Judenknecht' na torre, e um corpo cair na escada.", &[2]),
            testimony("O carro atrás da igreja", "Johanna viu o carro do prefeito atrás da igreja às três, de motor frio.", &[2]),
            clue("Eco: a torre", ClueKind::Temporal, Some(B(BKind::Church, 0)), Look3d::None, "No eco, um homem de sobretudo agarra o sacristão pelo colarinho. Florian se solta, agarra a corda e cai. O sino toca três vezes. Ninguém o puxou.", &[2]),
            clue("Eco: o ossuário", ClueKind::Temporal, Some(Cemetery), Look3d::None, "No eco, um gendarme empurra um carrinho de mão entre as covas e cobre algo com ossos velhos, chorando.", &[3]),
        ],
        echoes: vec![
            echo(B(BKind::Church, 0), Ghost::Struggle, &["Passos pesados na escada da torre.", "'Judenknecht!'", "Uma mão na corda. Uma queda.", "O sino, sozinho: três badaladas. 3:17."], 14),
            echo(Cemetery, Ghost::Drag, &["Uma roda range no cascalho.", "Ossos caindo sobre um corpo.", "'Perdoe, Florian. Ordens.'"], 15),
        ],
        culprit: 2,
        methods: &[
            "Lea o envenenou com remédios da farmácia e lavou o sangue com soda",
            "Discutiu com ele na torre e o derrubou escada abaixo; o gendarme escondeu o corpo no ossuário",
            "Florian fugiu para a Suíça e o sino é uma farsa",
            "O padre o matou para esconder os sinais do sino",
        ],
        method: 1,
        motives: &[
            "Ciúme — queria Lea Rosenthal para si",
            "Calar o rapaz que avisava os perseguidos com o sino e garantir a tomada da farmácia",
            "Roubar a coleta da igreja",
            "Ordem direta de Berlim",
        ],
        motive: 1,
        anomalies: &[
            "Nenhuma — é apenas um crime",
            "O sino toca às 3:17 sem badalo e sem mão: a esfera repete o último aviso de Florian em todas as noites de Hinterfeld, como um sinal que atravessa os anos",
            "O sacristão virou um fantasma",
            "O sino foi fundido com o metal da esfera",
        ],
        anomaly: 1,
        threads: &[(2, 0), (5, 2), (3, 2), (4, 0), (1, 0), (6, 2)],
        story: "Florian usava o sino da torre para avisar os Rosenthal e outros perseguidos quando os caminhões das SA vinham da cidade. Hedwig Lanz denunciou o sinal. Na madrugada seguinte, o prefeito Rautenberg subiu à torre para calá-lo; na briga, Florian caiu da escada e quebrou o pescoço. O gendarme Böhm escondeu o corpo no ossuário, e o prefeito inventou a fuga para a Suíça para acusar a família cuja farmácia estava tomando por 800 RM.",
        sphere: "O sino não toca nesta noite. Toca em todas as noites de Hinterfeld ao mesmo tempo: 1932, 1936, 1945. Florian deu o aviso uma vez; a esfera o repete às 3:17 porque alguém, em algum ano, ainda precisa fugir. Talvez você.",
        on_true: outcome(
            "O corpo de Florian foi enterrado com os sinos da comarca inteira tocando. O partido preferiu sacrificar Rautenberg a um escândalo: foi julgado e mandado para longe. Com a ajuda do padre e de Johanna, os Rosenthal cruzaram a fronteira suíça numa carroça de feno.",
            "PREFEITO DE HINTERFELD AFASTADO APÓS MORTE DE SACRISTÃO",
            "...o partido lamenta a conduta isolada de um funcionário...",
            vec![(2, "jailed"), (4, "left_city"), (1, "grateful"), (6, "survived")],
        )
        .flags(vec!["bell_true", "rosenthal_escaped"]),
        on_false: outcome(
            "A gendarmaria fechou o caso do jeito que o prefeito queria. A farmácia ganhou letreiro novo. Os Rosenthal foram levados num caminhão numa madrugada de novembro. Naquela noite o sino tocou três vezes, e ninguém fugiu.",
            "FARMÁCIA DE HINTERFELD SOB NOVA DIREÇÃO",
            "...o prefeito Rautenberg inaugura a nova farmácia ariana...",
            vec![(2, "criminal"), (4, "hates_elias")],
        )
        .close(vec![BKind::Pharmacy])
        .flags(vec!["bell_false"]),
        digit: "0",
        reward: 75,
    }
}

pub fn case08() -> CaseDef {
    CaseDef {
        id: 8,
        title: "O Estrangulador do Blecaute",
        city: CityId::London,
        year: 1940,
        intro: "Londres, 1940. A cidade apaga as luzes toda noite para os bombardeiros não a encontrarem. Alguém aprendeu a caçar nesse escuro. Três mulheres em seis noites, estranguladas em casa, as cadernetas de racionamento roubadas. A quarta sobreviveu, e o homem fugiu deixando para trás o estojo da máscara de gás.",
        brief: "Doris Pemberton, ex-corista, foi estrangulada em seu apartamento durante o blecaute. Três vítimas em seis noites. Uma sobrevivente. Um estojo de máscara de gás.",
        start: HomeOf(0),
        cast: vec![
            person("Doris", "Pemberton", true, 32, Job::Dancer, Role::Victim, B(BKind::Apartment, 2), "Ex-corista do West End. Morava sozinha. Encontrada estrangulada no chão da sala.").dead(),
            person("Leonard", "Whitlow", false, 26, Job::Student, Role::Suspect, B(BKind::Apartment, 6), "Cadete da RAF em treinamento, alojado num prédio requisitado. Educado, sotaque de escola cara, asas bordadas no peito.")
                .temper(20, 75, 10, 60)
                .flees()
                .topics(vec![
                    topic("noite", "Onde você estava na madrugada de terça?", "...Saí pela escada de incêndio. Todo cadete sai. Fui a Piccadilly procurar companhia. Paguei, bebi, voltei. Nada disso é crime numa guerra, é?")
                        .lie("No alojamento. Assinei o livro às dez e só saí às seis. Pergunte ao sargento.", 5),
                    topic("mascara", "Este estojo de máscara de gás é seu?", "Parece o meu, admito. Todo mundo tem um. Alguém deve ter trocado no pub. Não pode provar que eu estava naquele beco.")
                        .lie("Perdi minha máscara no trem, semana passada. Já dei parte.", 9),
                    topic("escuro", "Por que sempre durante o blecaute?", "Você já reparou? Por volta das três e quinze, todas as lanternas da rua apagam ao mesmo tempo. Um minuto inteiro de escuro perfeito. Eu ouço um zumbido antes. Como se Londres respirasse fundo.").req(Req::Clue(8)),
                ]),
            person("Stanley", "Crook", false, 45, Job::Pawnbroker, Role::Suspect, B(BKind::House, 4), "Dono de uma casa de penhores no East End. Conhecia Doris: ela penhorava joias quando o teatro fechou.")
                .temper(60, 35, 30, 80)
                .topics(vec![
                    topic("penhor", "A caderneta de Doris estava na sua loja.", "Tá bom, tá bom. Um rapaz da RAF me vendeu. Uma cigarreira de prata e três cadernetas de racionamento. Paguei duas libras e dez xelins. Não pergunto de onde vem, ninguém pergunta, tem guerra.")
                        .lie("Não compro coisa roubada. Sou comerciante honesto.", 3)
                        .reveals(&[11]),
                    topic("doris", "O senhor visitou Doris na véspera da morte?", "Fui cobrar. Ela devia quatro libras de um broche. Discutimos na porta, a vizinha ouviu. Mas saí às nove e ela estava viva, me xingando."),
                ]),
            person("Frank", "Mallory", false, 48, Job::Detective, Role::Contact, B(BKind::House, 9), "Inspetor-detetive da Scotland Yard. Dorme três horas por noite desde setembro.")
                .temper(30, 70, 70, 30)
                .topics(vec![
                    topic("padrao", "O que as mortes têm em comum?", "Três mulheres em seis noites. Todas sozinhas, todas durante o blecaute, todas estranguladas com a mão esquerda. E de todas levaram a caderneta de racionamento. Quem mata por açúcar e manteiga? Alguém que precisa vender algo rápido.").reveals(&[12]),
                    topic("crook", "E o penhorista Crook?", "Crook tem ficha. Receptação. Meu sargento quer prendê-lo amanhã e fechar o caso. Seria conveniente. Conveniente demais."),
                    topic("raf", "A RAF colabora?", "A RAF protege os seus. Um cadete com sotaque de Eton vale mais para o Ministério do que três coristas mortas. Traga-me um número de serviço e eu arrombo a porta do alojamento."),
                ]),
            person("Ivy", "Collins", true, 29, Job::Waiter, Role::Witness, B(BKind::Apartment, 4), "Garçonete. Foi atacada num beco na noite seguinte e sobreviveu. Tem marcas roxas no pescoço.")
                .temper(75, 55, 70, 30)
                .topics(vec![
                    topic("ataque", "O que aconteceu no beco?", "Ele me ofereceu cigarro, educado, voz de rapaz rico. Asas bordadas no peito. Aí as mãos dele estavam no meu pescoço. Um menino de entregas virou a esquina com a bicicleta e ele fugiu. Deixou o estojo da máscara no chão.").reveals(&[9]),
                    topic("rosto", "Você viu o rosto dele?", "Não. As lanternas apagaram todas de uma vez, como se alguém tivesse desligado a rua. Só lembro do cheiro: brilhantina e querosene de avião."),
                ]),
            person("Reginald", "Pike", false, 58, Job::Retired, Role::Witness, B(BKind::House, 7), "Guarda de defesa antiaérea do quarteirão. Capacete branco, apito, caderno de ocorrências.")
                .temper(40, 65, 75, 20)
                .topics(vec![
                    topic("ronda", "O senhor fazia ronda na rua de Doris?", "Às três, vi um rapaz de uniforme da RAF com lanterna azulada perto da porta dela. Achei que era namorado. Às três e dezessete todas as lanternas da rua morreram juntas, a minha também. Quando voltaram, ele não estava mais lá. Anotei no caderno.").reveals(&[8]),
                ]),
            person("Ronald", "Birch", false, 23, Job::Student, Role::Witness, B(BKind::Apartment, 6), "Cadete da RAF, colega de quarto de Leonard no alojamento.")
                .temper(70, 40, 60, 30)
                .topics(vec![
                    topic("alibi", "Leonard passou a noite no alojamento?", "Não. Ele desce pela escada de incêndio quase toda noite. Voltou às cinco, com uma caixinha de pó de arroz de mulher no bolso. Disse que era presente. Eu não quero problema com a RAF, senhor.")
                        .lie("Dormiu a noite toda. Ele ronca, eu saberia.", 5)
                        .reveals(&[10]),
                    topic("leonard", "Como é Leonard?", "Fala bem, gasta muito. Vive devendo. Tem sempre dinheiro na segunda-feira e nunca sei de onde."),
                ]),
        ],
        clues: vec![
            clue("Estojo de máscara de gás", ClueKind::Physical, Some(Alley(2)), Look3d::Object, "Um estojo cáqui de máscara de gás, abandonado no beco onde Ivy foi atacada. O número de serviço foi raspado.", &[1])
                .forensic("Sob pó de grafite, o número raspado reaparece: 525987 — RAF. Pertence a um cadete alojado em St John's Wood."),
            clue("Marcas no pescoço", ClueKind::Physical, Some(HomeOf(0)), Look3d::Blood, "Doris foi estrangulada no chão da sala. Marcas de dedos profundas, mais fortes de um lado.", &[1])
                .forensic("Mão esquerda dominante. A luva deixou costura em relevo: couro de luva regulamentar de oficial."),
            clue("Gaveta revirada", ClueKind::Document, Some(HomeOf(0)), Look3d::Paper, "A gaveta onde Doris guardava documentos está aberta. A caderneta de racionamento sumiu; a bolsa com dinheiro ficou.", &[]),
            clue("Caderneta de Doris no penhor", ClueKind::Document, Some(B(BKind::Pawn, 0)), Look3d::Paper, "A caderneta de racionamento de Doris Pemberton e uma cigarreira de prata gravada 'D.P.', na loja de Crook.", &[1, 2])
                .forensic("Impressões digitais de dedos longos, jovens. Na capa, uma mancha de tinta azul-acinzentada: a tinta dos formulários da RAF.")
                .network(Job::Pawnbroker),
            clue("Livro de entrada do alojamento", ClueKind::Document, Some(HomeOf(1)), Look3d::Paper, "Leonard Whitlow assinou a entrada às 22h e a saída às 6h. Nenhuma saída no meio da noite.", &[1]),
            clue("Escada de incêndio com fuligem", ClueKind::Physical, Some(HomeOf(1)), Look3d::Prints, "Na escada de incêndio do alojamento, pegadas de botina e tinta preta de blecaute raspada do corrimão.", &[1])
                .forensic("Botina regulamentar da RAF, número 42. As marcas descem e sobem na mesma noite.")
                .hidden(1),
            clue("Nota de cobrança de Crook", ClueKind::Document, Some(HomeOf(0)), Look3d::Paper, "'Doris — 4 libras do broche. Última vez que peço. — S.C.' Deixada sob a porta na véspera do crime.", &[2])
                .forensic("A tinta é de caneta de balcão barata. Crook é destro, e o assassino não.")
                .herring(),
            clue("Relógio de Doris parado", ClueKind::Physical, Some(HomeOf(0)), Look3d::Glow, "O relógio de pulso de Doris parou às 3:17 — mas o legista diz que ela morreu por volta das três. No verso, arranhado recentemente: 'o escuro tem hora'.", &[])
                .hidden(2),
            testimony("O cadete na porta", "O guarda antiaéreo viu um rapaz da RAF perto da porta de Doris às 3h. Às 3:17 todas as lanternas da rua apagaram ao mesmo tempo.", &[1]),
            testimony("Ivy escapou", "Ivy foi atacada por um rapaz educado de asas bordadas. Ele fugiu e deixou o estojo da máscara.", &[1]),
            testimony("A escada de incêndio", "Birch: Leonard desce pela escada de incêndio quase toda noite. Voltou às cinco com um pó de arroz de mulher.", &[1]),
            testimony("O rapaz da RAF no penhor", "Crook comprou de um rapaz da RAF a cigarreira e três cadernetas por duas libras e dez xelins.", &[1]),
            testimony("O padrão", "Mallory: todas estranguladas com a mão esquerda, durante o blecaute, cadernetas roubadas.", &[]),
            clue("Eco: a sala de Doris", ClueKind::Temporal, Some(HomeOf(0)), Look3d::None, "No eco, um rapaz de uniforme entra educadamente, tira as luvas, depois as veste de novo. Todas as luzes da rua morrem ao mesmo tempo.", &[1]),
            clue("Eco: o beco", ClueKind::Temporal, Some(Alley(2)), Look3d::None, "No eco, uma bicicleta vira a esquina. Um homem de asas bordadas solta a mulher e corre, deixando cair um estojo cáqui.", &[1]),
        ],
        echoes: vec![
            echo(HomeOf(0), Ghost::Struggle, &["Uma batida educada na porta.", "'Boa noite, senhorita. Tem fogo?'", "As lanternas da rua apagam todas juntas.", "3:17. Silêncio."], 13),
            echo(Alley(2), Ghost::Flee, &["Uma campainha de bicicleta.", "Um homem corre pelo beco.", "Um estojo cáqui quica nas pedras."], 14),
        ],
        culprit: 1,
        methods: &[
            "Crook a matou numa briga por dívida",
            "Saía pela escada de incêndio do alojamento, estrangulava as vítimas no escuro do blecaute e roubava as cadernetas para vender",
            "Um ladrão comum aproveitou um bombardeio",
            "Envenenamento por gás da própria máscara",
        ],
        method: 1,
        motives: &[
            "Cobrança de uma dívida de quatro libras",
            "Prazer em matar no escuro — e dinheiro fácil vendendo as cadernetas para pagar suas dívidas",
            "Espionagem alemã",
            "Ciúme de um antigo namorado",
        ],
        motive: 1,
        anomalies: &[
            "Nenhuma — é apenas um crime",
            "Às 3:17 todas as luzes da rua morrem ao mesmo tempo — a esfera escurece a linha do tempo por um minuto, e o assassino aprendeu a esperar por esse escuro",
            "A máscara de gás pertence a um homem morto em 1918",
            "As vítimas se estrangularam sozinhas",
        ],
        anomaly: 1,
        threads: &[(1, 0), (1, 4), (1, 2), (6, 1), (5, 0), (3, 2)],
        story: "Leonard Whitlow, cadete da RAF endividado, fugia do alojamento pela escada de incêndio e caçava mulheres sozinhas durante o blecaute. Estrangulava-as com a mão esquerda, de luvas, e levava as cadernetas de racionamento para vender a Crook. Com Ivy, foi interrompido por um menino de bicicleta e perdeu o estojo da máscara de gás com o próprio número de serviço.",
        sphere: "Leonard não inventou o escuro. Às 3:17 a esfera toca Londres, e por um minuto nenhuma luz obedece. Ele sentia o zumbido e esperava. Há homens que a esfera não arrasta, só acorda.",
        on_true: outcome(
            "O número de serviço levou Mallory à porta do alojamento. Leonard Whitlow foi preso ainda de uniforme e enforcado em Wandsworth. Ivy depôs de pescoço erguido. Na noite da sentença, as lanternas de Londres não apagaram.",
            "CADETE DA RAF É O ESTRANGULADOR DO BLECAUTE",
            "...a Scotland Yard confirma a prisão de um jovem aviador em St John's Wood...",
            vec![(1, "jailed"), (4, "survived"), (3, "grateful"), (5, "grateful")],
        )
        .flags(vec!["blackout_caught"]),
        on_false: outcome(
            "A Yard anunciou a prisão e Londres voltou a dormir no escuro. Duas semanas depois, uma enfermeira foi encontrada estrangulada em Paddington, sem a caderneta de racionamento.",
            "PRESO SUSPEITO DOS CRIMES DO BLECAUTE",
            "...nova vítima em Paddington. A polícia fala em imitador...",
            vec![(1, "criminal"), (2, "jailed")],
        )
        .flags(vec!["blackout_free"]),
        digit: "0",
        reward: 80,
    }
}

pub fn case09() -> CaseDef {
    CaseDef {
        id: 9,
        title: "Quem Pôs a Mulher no Olmo?",
        city: CityId::London,
        year: 1941,
        intro: "Londres, 1941. Quatro meninos procuravam ninhos num quarteirão bombardeado e encontraram uma caveira dentro do oco de um olmo velho. Um esqueleto inteiro, de pé, com um vestido de tafetá e um único sapato. Uma semana depois, apareceu nos muros de tijolo queimado, em giz branco: QUEM PÔS HETTY NO OLMO? Ninguém sabe quem escreve. Ninguém sabia o nome dela.",
        brief: "Um esqueleto de mulher dentro de um olmo oco num quarteirão bombardeado. Pichações perguntam quem pôs 'Hetty' ali. A polícia quer arquivar.",
        start: Park,
        cast: vec![
            person("Hetty", "van Aalst", true, 29, Job::Dancer, Role::Victim, B(BKind::Apartment, 3), "Cantora de cabaré holandesa, chegou a Londres em 1939. Desapareceu há onze meses, numa noite de bombardeio.").dead(),
            person("Victor", "Ashby", false, 47, Job::Merchant, Role::Suspect, B(BKind::House, 5), "Relojoeiro e antiquário no Soho. Corrente de relógio de ouro, chapéu-coco, voz mansa. Frequentador do cabaré.")
                .temper(20, 60, 10, 70)
                .flees()
                .topics(vec![
                    topic("hetty", "O senhor conhecia Hetty van Aalst?", "...Conhecia. Ela me trazia envelopes de Lisboa, eu consertava relógios que não estavam quebrados. Era um arranjo. Ela quis desfazer o arranjo. Ninguém desfaz esse tipo de arranjo, Sr. Vale.")
                        .lie("Nunca ouvi falar. Não frequento cabarés, sou um homem de relógios.", 8),
                    topic("radio", "O que é o transmissor debaixo do seu assoalho?", "Um rádio amador. Todo inglês tem um hobby.")
                        .lie("Não há transmissor nenhum. Conserto relógios, nada mais.", 4),
                    topic("olmo", "O senhor visita o olmo aos domingos.", "Levo flores. Mesmo um relojoeiro sabe quando parou um relógio que não devia. Na noite do bombardeio, o tronco estava aberto como uma boca. Parecia esperar.").req(Req::Clue(10)),
                ]),
            person("Mabel", "Drury", true, 52, Job::Madam, Role::Witness, B(BKind::Apartment, 5), "Dona do cabaré Lanterna Azul, onde Hetty cantava. Batom vermelho e memória seletiva.")
                .temper(55, 50, 45, 65)
                .topics(vec![
                    topic("noite", "Com quem Hetty saiu na última noite?", "Com o relojoeiro. O Sr. Ashby. Corrente de ouro, chapéu-coco. Pagou a conta dela, três libras, e saíram no meio do alarme antiaéreo. Eu não disse antes porque ele paga bem e paga em dia.")
                        .lie("Saiu sozinha, como sempre. Pegou o metrô para se abrigar.", 5)
                        .reveals(&[8]),
                    topic("hetty", "Como era Hetty?", "Cantava em holandês, alemão e num inglês que partia o coração. Nos últimos meses tinha medo. Pediu adiantamento para um bilhete de navio para a América."),
                ]),
            person("Pieter", "Jansen", false, 34, Job::Musician, Role::Suspect, B(BKind::Apartment, 7), "Pianista holandês do cabaré. Ex-amante de Hetty. Bebe gim antes do meio-dia.")
                .temper(60, 45, 55, 30)
                .topics(vec![
                    topic("carta", "Você escreveu que mataria Hetty se ela o deixasse.", "Escrevi. Em 1939, bêbado, em Roterdã. Ela me deixou mesmo assim e eu não matei ninguém. Na noite em que ela sumiu eu estava em Liverpool com a orquestra. Pergunte a quarenta músicos.").req(Req::Clue(2)).reveals(&[9]),
                    topic("hetty", "Hetty estava metida em alguma coisa?", "Ela trazia envelopes na bolsa de maquiagem. Eu não perguntava. Holandeses em Londres aprendem a não perguntar."),
                ]),
            person("Peggy", "Lowe", true, 30, Job::Journalist, Role::Contact, B(BKind::Apartment, 8), "Repórter do Evening Courier. Cobre o caso do olmo e parece saber mais do que escreve.")
                .temper(25, 80, 80, 20)
                .soul(1)
                .topics(vec![
                    topic("pichacao", "Quem escreve as pichações?", "...Eu. Acho. Acordo às três e dezessete com giz nos dedos e os joelhos sujos de tijolo. A letra é a minha. Eu não me lembro de escrever. E eu não sabia o nome 'Hetty' até ler no muro.").reveals(&[12]),
                    topic("voce", "Nós já nos conhecemos, Peggy?", "Em outra redação, eu acho. Em outra cidade, cheia de jazz. Você sempre chega depois do crime e sempre vai embora antes de mim. Da próxima vez, Elias, me diga o ano antes de sumir."),
                    topic("jornal", "O que o Courier sabe do caso?", "Que a Yard recebeu ordem de cima para arquivar. Que tem gente do serviço secreto rondando o olmo. E que ninguém quer saber de uma cantora estrangeira morta."),
                ]),
            person("Tommy", "Grove", false, 13, Job::Child, Role::Witness, B(BKind::House, 2), "Um dos meninos que acharam a caveira. Voltou ao olmo todos os dias desde então.")
                .temper(50, 70, 70, 20)
                .topics(vec![
                    topic("olmo", "Como você achou a caveira?", "Eu subi pra ver um ninho e enfiei o braço no oco. Estava quente lá dentro, moço. Quente em fevereiro. Aí eu vi os dentes.").reveals(&[10]),
                    topic("homem", "Alguém mais visita o olmo?", "Um senhor de chapéu-coco, todo domingo. Deixa flores e fica olhando o relógio. Na semana passada ele chorou."),
                ]),
            person("Harold", "Quayle", false, 50, Job::Detective, Role::Contact, B(BKind::House, 10), "Superintendente da polícia local. Quer o caso arquivado antes que os jornais mexam demais.")
                .temper(45, 55, 40, 45)
                .topics(vec![
                    topic("arquivo", "Por que arquivar o caso?", "Porque o MI5 mandou. A moça era estrangeira registrada, e o nome dela estava numa lista de mensageiros de uma rede alemã. Eles preferem que a rede não saiba que a gente sabe. Uma morta não ganha guerra.")
                        .lie("É uma cigana qualquer, morta há anos. Bruxaria, dizem. Não há caso.", 6)
                        .reveals(&[11]),
                ]),
        ],
        clues: vec![
            clue("Esqueleto no olmo", ClueKind::Physical, Some(Park), Look3d::Blood, "Dentro do oco do olmo, um esqueleto de mulher, com um pedaço de tafetá enfiado na boca e um único sapato de salto.", &[])
                .forensic("O tafetá é novo, de um vestido de palco. Mas os ossos estão secos e porosos como se tivessem trinta anos. Ela sumiu há onze meses."),
            clue("Pichação no muro", ClueKind::Document, Some(Near(BKind::Abandoned, 0)), Look3d::Paper, "Em giz branco, nos muros de tijolo queimado: 'QUEM PÔS HETTY NO OLMO?'. A mesma frase em seis muros diferentes.", &[4])
                .forensic("A letra é de mulher, canhota, pressionando forte. Cada pichação foi feita num muro que ainda não tinha sido bombardeado quando a primeira apareceu."),
            clue("Carta de Pieter", ClueKind::Document, Some(HomeOf(3)), Look3d::Paper, "'Se você me deixar, Hetty, eu te mato. Juro por Deus. — P.'", &[3])
                .forensic("O papel tem timbre de um hotel de Roterdã, 1939. Dois anos antes do crime.")
                .herring(),
            clue("O sapato sem par", ClueKind::Physical, Some(B(BKind::General, 1)), Look3d::Object, "No porão da loja de Ashby, atrás de caixas de relógios, um sapato de salto — o par do que estava no olmo.", &[1])
                .forensic("Sola arranhada de arrastar em cascalho. Um fio de tafetá preso na fivela.")
                .hidden(1),
            clue("Transmissor sob o assoalho", ClueKind::Physical, Some(HomeOf(1)), Look3d::Object, "Sob as tábuas da casa de Ashby, um transmissor de ondas curtas, fones e uma tabela de códigos.", &[1])
                .forensic("A tabela de horários de transmissão tem uma linha só riscada à mão: 'H. — não aparece mais'.")
                .hidden(2),
            clue("Bilhete no camarim", ClueKind::Document, Some(B(BKind::Cabaret, 0)), Look3d::Paper, "Escondido no forro do espelho do camarim de Hetty: 'Vou procurar o Ministério. Tenho nomes. O relojoeiro é o primeiro. — H.'", &[1]),
            clue("Ficha de estrangeira", ClueKind::Document, Some(B(BKind::Police, 0)), Look3d::Paper, "Henrietta van Aalst, holandesa, registrada em 1939. Anotação a lápis: 'mensageira — contato: A. Não interferir. MI5'.", &[1])
                .network(Job::Police),
            clue("Serrote com seiva de olmo", ClueKind::Physical, Some(HomeOf(1)), Look3d::Weapon, "No barracão do quintal de Ashby, um serrote com lascas e seiva escura de olmo presas nos dentes.", &[1])
                .forensic("Alguém alargou a abertura do tronco por dentro. Há serragem de olmo na barra das calças penduradas ao lado."),
            testimony("A última noite", "Mabel: Hetty saiu do cabaré com o relojoeiro Ashby durante o alarme antiaéreo.", &[1]),
            testimony("O álibi de Pieter", "Pieter estava em Liverpool com a orquestra na noite em que Hetty sumiu.", &[3]),
            testimony("O oco quente", "Tommy: o oco do olmo estava quente em fevereiro. Um senhor de chapéu-coco deixa flores lá todo domingo.", &[1]),
            testimony("O MI5 abafou", "Quayle: Hetty era mensageira de uma rede alemã e o MI5 mandou arquivar.", &[1]),
            testimony("A letra de Peggy", "Peggy acorda às 3:17 com giz nos dedos. As pichações estão na letra dela.", &[4]),
            clue("Eco: o olmo", ClueKind::Temporal, Some(Park), Look3d::None, "No eco, sob as bombas, um homem de chapéu-coco empurra uma mulher de tafetá para dentro do tronco. O tronco se fecha um pouco, como uma boca.", &[1]),
            clue("Eco: a loja", ClueKind::Temporal, Some(B(BKind::General, 1)), Look3d::None, "No eco, entre relógios que batem juntos: 'Você não vai ao Ministério, Hetty.' Uma corrente de relógio de ouro aperta um pescoço.", &[1]),
        ],
        echoes: vec![
            echo(Park, Ghost::Drag, &["Sirenes. Holofotes no céu.", "Um homem arrasta algo pesado entre os escombros.", "O olmo abre-se como uma boca.", "3:17. O tronco se fecha."], 13),
            echo(B(BKind::General, 1), Ghost::Argue, &["Cem relógios batem ao mesmo tempo.", "'Você não vai ao Ministério, Hetty.'", "'Tenho nomes, Victor. O seu é o primeiro.'", "Uma corrente de ouro."], 14),
        ],
        culprit: 1,
        methods: &[
            "Pieter a estrangulou num acesso de ciúme",
            "Estrangulou-a com a corrente do relógio na loja e, durante um bombardeio, escondeu o corpo no oco do olmo",
            "Ritual de bruxaria cigana",
            "Morreu num bombardeio e saqueadores esconderam o corpo",
        ],
        method: 1,
        motives: &[
            "Ciúme de ex-amante",
            "Ela ia entregar a rede de espionagem ao Ministério, começando por ele",
            "Roubar as joias dela",
            "Sacrifício ritual",
        ],
        motive: 1,
        anomalies: &[
            "Nenhuma — é apenas um crime",
            "Os ossos têm trinta anos, mas ela sumiu há onze meses: o oco do olmo é uma dobra onde o tempo corre depressa — e as pichações aparecem na letra de Peggy, escritas às 3:17 sem que ela lembre",
            "O olmo é mais velho que Londres",
            "Hetty nunca existiu",
        ],
        anomaly: 1,
        threads: &[(1, 0), (2, 0), (3, 0), (6, 0), (4, 5), (1, 6)],
        story: "Hetty van Aalst era mensageira de uma rede de espionagem alemã comandada, em Londres, pelo relojoeiro Victor Ashby. Cansada e com medo, decidiu entregar a rede ao Ministério. Ashby soube, levou-a do cabaré durante um alarme antiaéreo, estrangulou-a com a corrente do relógio no porão da loja e, sob as bombas, escondeu o corpo no oco de um olmo no quarteirão destruído. O MI5 sabia de tudo e mandou arquivar, para não queimar a rede que vigiava.",
        sphere: "O olmo não é só uma árvore: é um lugar onde a esfera dobra o tempo sobre si mesmo. Onze meses lá dentro valeram trinta anos. E Peggy escreve o nome de uma morta que nunca conheceu porque, em outra linha, foi ela quem ficou no olmo. A alma que te segue lembra das próprias mortes.",
        on_true: outcome(
            "Ashby foi preso ao tentar embarcar para Lisboa. O MI5 o levou antes que a polícia terminasse de algemá-lo, e o julgamento foi secreto. Peggy publicou o nome de Hetty na primeira página. As pichações pararam.",
            "ANTIQUÁRIO DO SOHO PRESO: A MULHER DO OLMO TINHA NOME",
            "...o Ministério não comenta rumores de espionagem...",
            vec![(1, "jailed"), (3, "survived"), (4, "journalist"), (5, "grateful")],
        )
        .flags(vec!["elm_true"]),
        on_false: outcome(
            "O caso foi arquivado como 'desconhecida, morte antiga'. Ashby continuou consertando relógios. Durante décadas, alguém continuou escrevendo nos muros de Londres: QUEM PÔS HETTY NO OLMO?",
            "OSSOS DO OLMO: CASO ARQUIVADO",
            "...as pichações voltaram a aparecer em Birmingham...",
            vec![(1, "criminal"), (3, "hates_elias")],
        )
        .flags(vec!["elm_false"]),
        digit: "7",
        reward: 85,
    }
}

pub fn case10() -> CaseDef {
    CaseDef {
        id: 10,
        title: "A Voz no Rádio",
        city: CityId::London,
        year: 1944,
        intro: "Londres, 1944. As bombas voadoras chegam sem piloto e sem aviso: primeiro o zumbido, depois o silêncio, depois a explosão. Há três semanas, às 3:17, a Rádio do Império transmite sozinha uma voz masculina que diz uma rua e uma hora. Doze horas depois, uma V-1 cai exatamente ali. Ontem, o engenheiro que gravava a voz morreu eletrocutado no transmissor. A voz é a sua.",
        brief: "Cyril Hollis, engenheiro da rádio, morreu eletrocutado. Ele gravava uma voz que anuncia às 3:17 onde cairão as V-1. A voz é a de Elias.",
        start: B(BKind::Radio, 0),
        cast: vec![
            person("Cyril", "Hollis", false, 52, Job::Scientist, Role::Victim, B(BKind::House, 3), "Engenheiro-chefe de transmissão. Metódico, gravava tudo em discos de acetato. Morto na sala do transmissor.").dead(),
            person("Dennis", "Rook", false, 34, Job::Scientist, Role::Suspect, B(BKind::Apartment, 5), "Técnico assistente de Hollis. Magro, canhoto, dentes manchados de nicotina. Dispensado do exército por um pulmão ruim.")
                .temper(40, 50, 15, 85)
                .flees()
                .topics(vec![
                    topic("noite", "Onde você estava na madrugada da morte de Hollis?", "...Fiquei até a uma, na sala do transmissor. Ajustando válvulas. Saí pela doca de carga porque a porta da frente range. Quando saí, o Cyril ainda nem tinha chegado. Não pode provar que mexi em nada.")
                        .lie("Saí às dez, fui ao pub da esquina. O dono me conhece.", 9),
                    topic("doyle", "Você conhece Maxie Doyle?", "Todo mundo em Stepney conhece o Maxie. Tá bom: eu copiava as mensagens da voz antes de irem para o Ministério e vendia pra ele. Quarenta libras por rua. Ninguém morria por causa disso, as bombas caíam de qualquer jeito!")
                        .lie("Gângster? Eu? Sou técnico de rádio, senhor.", 3),
                    topic("cyril", "Hollis sabia do seu negócio?", "O velho anotava tudo. Ia contar ao major na segunda. Eu tenho mãe, dívidas e um pulmão só. Às três e dezessete ele sempre abria o painel para gravar a voz. Sempre.").req(Req::Both(0, 4)),
                ]),
            person("Vera", "Ashcombe", true, 31, Job::RadioHost, Role::Contact, B(BKind::Apartment, 9), "Locutora do turno da madrugada. Foi ela quem ouviu a voz pela primeira vez. Olha para Elias como quem reconhece uma música.")
                .temper(30, 75, 85, 15)
                .soul(1)
                .topics(vec![
                    topic("voz", "O que exatamente a voz diz?", "Às três e dezessete o ponteiro sai sozinho da nossa frequência e volta. E uma voz de homem diz: 'Rua tal, onze e doze.' Sempre acerta. Tem um sotaque que não é daqui. É a sua voz, Elias. Eu ouvi você falar na recepção e precisei sentar.").reveals(&[8]),
                    topic("voce", "Você já me conheceu antes, Vera?", "Em Nova Orleans eu era repórter. Em Hinterfeld eu dava aula. Em Londres eu pichava muros sem saber. Não me olhe assim, eu também não sei como sei disso. Mas eu sei como termina: numa sala branca, em 1971, e você diz 'não toque'. Sempre tarde demais."),
                    topic("fita", "O rolo com meu nome no estúdio. Foi você?", "Não. Apareceu ao lado do relógio parado, na noite em que o Cyril morreu. A letra é sua, não é? 'Para Vera. Não deixe ele tocar.' Eu não sei quem é 'ele'. Acho que você sabe.").req(Req::Clue(7)),
                ]),
            person("Rupert", "Kell", false, 55, Job::Politician, Role::Suspect, B(BKind::Mansion, 1), "Major do Ministério da Informação, censor da rádio. Bigode encerado, pasta de couro, nenhum senso de humor.")
                .temper(35, 60, 45, 40)
                .topics(vec![
                    topic("censura", "O Ministério sabia da voz?", "Sabia. Nós a censuramos. Imagine o pânico: uma voz fantasma anunciando bombas. Mandei silenciar Hollis — transferi-lo para Bletchley, não matá-lo, pelo amor de Deus. Ele recusou. Queria levar os discos à imprensa.")
                        .lie("Não existe voz nenhuma. Interferência atmosférica. Assunto encerrado.", 1)
                        .reveals(&[12]),
                    topic("ruas", "Por que não evacuar as ruas anunciadas?", "Evacuamos duas. Em sigilo. Ninguém acreditaria na origem da informação. E se o inimigo soubesse que sabemos... A guerra tem uma aritmética feia, Sr. Vale."),
                ]),
            person("Maxie", "Doyle", false, 40, Job::Gangster, Role::Suspect, B(BKind::House, 8), "Chefe de um bando de Stepney. Mercado negro, saques, cupons de gasolina falsos.")
                .temper(15, 85, 10, 95)
                .topics(vec![
                    topic("dicas", "O senhor comprava as ruas antes das bombas?", "Comprava. Do técnico magrinho, o Rook. Quarenta libras a rua. Meus rapazes esvaziavam as casas na véspera: prata, casacos, cupons. Depois a V-1 cobria o resto. Negócio limpo. Mas matar o engenheiro? Isso é coisa do magrinho, não minha.")
                        .lie("Eu vendo frutas no mercado, amigo. Frutas.", 4)
                        .reveals(&[11]),
                ]),
            person("Edna", "Hollis", true, 50, Job::Housewife, Role::Witness, B(BKind::House, 3), "Viúva de Cyril. Mantém a mesa posta para dois.")
                .temper(75, 40, 80, 20)
                .topics(vec![
                    topic("cyril", "Cyril estava com medo de alguém?", "Dizia que alguém na rádio copiava as mensagens antes de o Ministério ler. 'Estão vendendo as bombas, Edna.' Ia contar ao major Kell na segunda-feira. Não chegou à segunda.").reveals(&[10]),
                    topic("discos", "Ele trazia gravações para casa?", "Os discos de acetato, na caixa de chapéus. Tocava de madrugada, de fone, e chorava. Dizia que conhecia aquela voz. Que era a voz de alguém que ainda não tinha nascido direito."),
                ]),
            person("Walter", "Pryce", false, 61, Job::Retired, Role::Witness, B(BKind::House, 11), "Vigia noturno da rádio. Veterano de Ypres. Não dorme em serviço, jura.")
                .temper(50, 60, 70, 25)
                .topics(vec![
                    topic("vigia", "O senhor viu alguém na madrugada?", "O Rook. Sozinho na sala do transmissor até quase uma. Saiu pela doca de carga com uma bolsa de ferramentas. Às três chegou o Sr. Hollis, como toda noite. Às três e dezessete as luzes do prédio piscaram, e ele gritou.").reveals(&[9]),
                ]),
        ],
        clues: vec![
            clue("Painel do transmissor", ClueKind::Physical, Some(B(BKind::Radio, 0)), Look3d::Weapon, "A trava do painel do transmissor estava ligada à alta tensão. Quem abrisse a porta recebia cinco mil volts.", &[1])
                .forensic("O fio de aterramento foi cortado e uma ponte de cobre nova liga a trava ao circuito. A solda foi feita por um canhoto, com o estanho do almoxarifado da rádio."),
            clue("Discos de acetato", ClueKind::Document, Some(HomeOf(0)), Look3d::Paper, "Gravações de Hollis: 'Aldgate, onze e doze.' 'Lambeth Road, nove e quarenta.' Datadas e cruzadas com as quedas das V-1. Todas certas.", &[])
                .forensic("A voz é a sua, Elias, mais velha e cansada. No fundo do terceiro disco, uma sirene que não é inglesa, e alguém diz baixinho: 'Laboratório Delta, gravação 1971, número três.'"),
            clue("Caderno de Hollis", ClueKind::Document, Some(HomeOf(0)), Look3d::Paper, "'D.R. chega antes de mim e copia as mensagens. Para quem? Contar ao Kell na segunda.'", &[1])
                .hidden(1),
            clue("Lista de ruas na casa de Doyle", ClueKind::Document, Some(HomeOf(4)), Look3d::Paper, "Uma lista de ruas e horários, com a letra inclinada de um canhoto. Ao lado de cada uma: '£40 — pago'.", &[1, 4])
                .hidden(1),
            clue("Depósitos de Rook", ClueKind::Document, Some(B(BKind::Bank, 0)), Look3d::Paper, "Conta de Dennis Rook: depósitos de 40 libras no dia seguinte a cada queda de V-1 anunciada pela voz.", &[1, 4])
                .network(Job::Banker),
            clue("Carta do Ministério", ClueKind::Document, Some(HomeOf(3)), Look3d::Paper, "Papel timbrado do Ministério da Informação: 'Silenciar Hollis a qualquer custo.' Assinado: R. Kell.", &[3])
                .forensic("No verso, a lápis, com a mesma letra: 'transferência para Bletchley — providenciar'. Silenciar, não matar.")
                .herring(),
            clue("Alicate de Rook", ClueKind::Physical, Some(HomeOf(1)), Look3d::Weapon, "Um alicate de corte com cabo gasto do lado esquerdo e restos de cobre novo entre as lâminas.", &[1])
                .forensic("O cobre é da mesma bobina da ponte instalada no painel do transmissor."),
            clue("Relógio do estúdio parado", ClueKind::Physical, Some(B(BKind::Radio, 0)), Look3d::Glow, "O relógio do estúdio parou às 3:17. Ao lado, um rolo de fita que ninguém da rádio sabe usar — fita magnética não existe na Inglaterra de 1944. Etiqueta com a sua letra: 'Para Vera. Não deixe ele tocar.'", &[])
                .hidden(2),
            testimony("A voz é a sua", "Vera: às 3:17 a frequência muda sozinha e uma voz, a de Elias, anuncia rua e hora de uma V-1.", &[]),
            testimony("Rook na doca de carga", "O vigia viu Rook sozinho no transmissor até a uma, saindo pela doca de carga. Às 3:17 as luzes piscaram.", &[1]),
            testimony("Estão vendendo as bombas", "Edna: Cyril descobriu que alguém da rádio vendia as mensagens e ia contar ao major na segunda.", &[1]),
            testimony("Doyle comprava as ruas", "Doyle comprava de Rook as ruas anunciadas, a quarenta libras, e saqueava as casas antes da bomba.", &[1, 4]),
            testimony("O Ministério abafava", "Kell: o Ministério censurava a voz e queria transferir Hollis, que ameaçava ir à imprensa.", &[3]),
            clue("Eco: a sala do transmissor", ClueKind::Temporal, Some(B(BKind::Radio, 0)), Look3d::None, "No eco, Hollis abre o painel para gravar a voz. Um clarão azul. Do alto-falante, a voz de Elias: 'Cyril, não abra.' Tarde demais.", &[1]),
            clue("Eco: a doca de carga", ClueKind::Temporal, Some(Near(BKind::Radio, 0)), Look3d::None, "No eco, um homem magro sai pela doca de carga com uma bolsa de ferramentas, tossindo, e olha para o relógio.", &[1]),
        ],
        echoes: vec![
            echo(B(BKind::Radio, 0), Ghost::Struggle, &["Um zumbido distante, de motor de V-1.", "O ponteiro do rádio corre sozinho.", "'Cyril, não abra.'", "Um clarão azul. 3:17."], 13),
            echo(Near(BKind::Radio, 0), Ghost::Walk, &["A porta da doca range.", "Um homem magro tosse no escuro.", "Ferramentas tilintam numa bolsa."], 14),
        ],
        culprit: 1,
        methods: &[
            "Kell mandou um agente do Ministério matá-lo",
            "Ligou a trava do painel do transmissor à alta tensão, sabendo que Hollis o abriria às 3:17 para gravar a voz",
            "Um curto-circuito causado pela explosão de uma V-1",
            "Doyle o espancou e simulou o choque",
        ],
        method: 1,
        motives: &[
            "Vingança por uma promoção negada",
            "Hollis ia denunciar que ele vendia as ruas anunciadas a Doyle",
            "Ordem do Ministério para abafar a voz",
            "Roubar os discos de acetato para vendê-los aos alemães",
        ],
        motive: 1,
        anomalies: &[
            "Nenhuma — é apenas um crime",
            "A voz é a sua, transmitida de 1971: o Laboratório Delta gravou os avisos e a esfera os devolve a 1944 às 3:17 — e é você, no futuro, quem tenta avisar",
            "A rádio capta transmissões de Marte",
            "Hollis falsificava a voz com um imitador",
        ],
        anomaly: 1,
        threads: &[(1, 0), (1, 4), (3, 0), (5, 0), (6, 1), (2, 0)],
        story: "Às 3:17, uma voz atravessava a frequência da Rádio do Império anunciando onde cairiam as V-1. Dennis Rook copiava as mensagens antes do Ministério e as vendia a Maxie Doyle, que saqueava as casas na véspera das bombas. Hollis descobriu e ia denunciá-lo ao major Kell. Rook ficou até a uma na sala do transmissor e ligou a trava do painel à alta tensão, sabendo que Hollis o abriria às 3:17 para gravar a voz.",
        sphere: "Agora você sabe de quem é a voz. Em 1971, no Laboratório Delta, você vai sentar diante da esfera e falar para dentro dela, tentando avisar todos os anos ao mesmo tempo. Ela devolve cada palavra às 3:17. A esfera não foi encontrada por você, Elias. Ela foi feita por você. E você ainda não sabe como, nem para quê.",
        on_true: outcome(
            "Rook foi preso na doca de carga, com a bolsa de ferramentas. Doyle caiu uma semana depois, com um caminhão de prata saqueada. Kell entregou os discos ao arquivo secreto do Ministério. Na última noite antes de você partir, a voz não disse uma rua. Disse: 'Obrigado, Vera.'",
            "TÉCNICO DE RÁDIO É PRESO PELA MORTE DE ENGENHEIRO",
            "...e aqui é a Rádio do Império. A transmissão da madrugada termina às 3:17...",
            vec![(1, "jailed"), (4, "jailed"), (2, "grateful"), (5, "survived")],
        )
        .flags(vec!["radio_true", "voice_known"]),
        on_false: outcome(
            "O caso foi arquivado como acidente de trabalho em tempo de guerra. A voz continuou, e Doyle continuou comprando ruas. Numa noite de agosto, a voz anunciou a rua da própria rádio. Ninguém evacuou.",
            "ENGENHEIRO MORRE EM ACIDENTE NA RÁDIO DO IMPÉRIO",
            "...o Ministério da Informação desmente boatos sobre transmissões fantasmas...",
            vec![(1, "criminal"), (4, "criminal"), (2, "hates_elias")],
        )
        .flags(vec!["radio_false"]),
        digit: "1",
        reward: 95,
    }
}
