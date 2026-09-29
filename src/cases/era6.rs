//! ERA VI — 2001 — Nova Orleans. A era final: o círculo se fecha onde começou.

use super::defs::*;
use crate::city::gen::CityId;
use crate::city::map::BKind;
use crate::sim::people::Job;
use Place::*;

pub fn cases() -> Vec<CaseDef> {
    vec![case26(), case27(), case28(), case29(), case30()]
}

pub fn case26() -> CaseDef {
    CaseDef {
        id: 26,
        title: "O Retorno do Machado",
        city: CityId::NewOrleans,
        year: 2001,
        intro: "Nova Orleans, 2001. Os bondes ainda rangem na St. Charles, os bares da Bourbon vendem 'Axeman Tours' para turistas de pochete, e ninguém leva a lenda a sério. Até esta madrugada. Na mesma casa da Rua Royal onde Giuseppe Cavaretta morreu em 1920, um antiquário foi morto a machadadas. A porta dos fundos foi aberta com um cinzel. E o jornal recebeu um e-mail assinado 'Do Inferno'.",
        brief: "Vincent Moreau, antiquário, foi morto a machadadas na antiga casa dos Cavaretta. A esposa, Denise, sobreviveu ferida. Um e-mail 'Do Inferno' chegou à redação às 3:17. O detetive do caso se chama Hale.",
        start: HomeOf(0),
        cast: vec![
            person("Vincent", "Moreau", false, 58, Job::Merchant, Role::Victim, B(BKind::House, 3), "Antiquário. Comprou a velha casa dos Cavaretta com os móveis dentro. Morto a machadadas na própria cama.").dead(),
            person("Denise", "Moreau", true, 52, Job::Merchant, Role::Witness, B(BKind::House, 3), "Esposa de Vincent. Sobreviveu com um corte no ombro. Segura o celular como se fosse um crucifixo.")
                .temper(80, 30, 70, 30)
                .topics(vec![
                    topic("noite", "O que a senhora viu naquela noite?", "Acordei com o rangido da porta... Uma lanterna na minha cara, dessas de polícia. E no cinto, brilhando, um distintivo. Depois o Vincent gritou. Pelo amor de Deus, não diga que fui eu que contei.")
                        .lie("Nada. Eu dormia. Acordei com o sangue no lençol.", 12)
                        .reveals(&[7]),
                    topic("taxa", "Seu marido tinha inimigos?", "Um policial vinha toda sexta buscar 'a taxa de segurança do Quarter'. Duzentos dólares em dinheiro. Vincent pagou dois anos. No mês passado disse chega, e que ia levar tudo pro jornal. Na semana seguinte começaram os bipes.").reveals(&[8]),
                    topic("relogio", "Esse relógio da cozinha parou às 3:17. E esses riscos?", "Veio com a casa. Vincent dizia que era de 1920, que era a peça mais valiosa da loja e nunca vendeu. Os riscos 1921, 1946, 1971 já estavam lá. O 2001... esse eu nunca tinha visto.").req(Req::Clue(13)),
                ]),
            person("Raymond", "Hale", false, 46, Job::Detective, Role::Suspect, B(BKind::House, 11), "Detetive da Divisão de Homicídios. Bisneto de Walter Hale. Usa o anel de pedra vermelha da família e odeia cada centímetro dele.")
                .temper(35, 70, 70, 25)
                .topics(vec![
                    topic("bisavo", "O senhor sabe o que seu bisavô fez nesta casa?", "Walter Hale. O primeiro Homem do Machado. Minha avó queimou as fotos dele. Eu virei policial pra limpar esse nome. E agora alguém repete o crime dele no endereço dele, no meu plantão. Acha que é coincidência?"),
                    topic("anel", "Acharam um anel de pedra vermelha perto da cama.", "O meu está aqui, ó. Não sai do dedo desde o enterro do meu pai. Esse aí é réplica de loja de turista — vendem na Decatur por quinze dólares. Alguém quer que você olhe pra mim.").req(Req::Clue(4)),
                    topic("parceiro", "Quem mais sabia que Vincent parou de pagar?", "...Meu parceiro. Gil Thibodeaux. Ele trocou o turno comigo naquela noite, pediu a escala emprestada. Disse que era aniversário da filha. Gil não tem filha.")
                        .lie("Ninguém. Gil é um bom policial, dezoito anos de rua. Esqueça essa história de taxa.", 5)
                        .reveals(&[10]),
                ]),
            person("Gil", "Thibodeaux", false, 49, Job::Police, Role::Suspect, B(BKind::House, 8), "Sargento, parceiro de Hale. Bigode grosso, botas de serviço, um pager que não para de vibrar.")
                .temper(20, 80, 15, 85)
                .flees()
                .topics(vec![
                    topic("noite", "Onde o senhor estava às três da manhã?", "...Tá. Passei na Royal. Fui buscar o que ele me devia, só isso. Quando eu saí o velho estava vivo, xingando a minha mãe. Vivo!")
                        .lie("Patrulhando o porto. Pergunte pro rádio da central, está tudo gravado.", 9),
                    topic("planilha", "O que é esta planilha com nomes de antiquários?", "Isso é assunto interno, forasteiro. Você não entende como o Quarter funciona. Meu avô cobrava, o avô dele cobrava. Sempre teve alguém cobrando. E quem não paga...").req(Req::Clue(5)),
                    topic("bipe", "Alguém mandou bipes ameaçando Vincent. Do orelhão em frente à delegacia.", "Qualquer um usa aquele orelhão. Bêbado, turista, advogado. Você vai prender a cidade inteira?").req(Req::Clue(6)),
                ]),
            person("Clara", "Bell", true, 112, Job::Retired, Role::Contact, B(BKind::Apartment, 5), "A mulher mais velha da Louisiana. Mora no mesmo apartamento desde 1920. Foi repórter. Olha para você como quem reencontra alguém.")
                .temper(10, 90, 85, 10)
                .soul(1)
                .topics(vec![
                    topic("machado", "A senhora se lembra do Homem do Machado?", "Eu cobri o caso, meu bem. Em 1920 foi um guarda. Em 2001 vai ser outro. Nesta cidade o machado sempre fica com quem cobra. Olhe para quem vai buscar o dinheiro na sexta-feira.").reveals(&[11]),
                    topic("voce", "A senhora me conhece?", "Você não envelheceu um dia, Elias. Eu envelheci por nós dois. Oitenta e um anos esperando você bater na porta. Senta. Eu fiz café. Eu sempre faço café às três da manhã, caso seja hoje."),
                    topic("relogio", "O relógio dos Cavaretta voltou a parar às 3:17.", "Aquele relógio para toda vez que você chega. Parou em 1920. Parou de novo agora. Eu risquei algumas datas nele com a unha, sabia? Na primeira vez. Pra você achar.").req(Req::Clue(13)),
                ]),
            person("Marcus", "Reed", false, 34, Job::Bartender, Role::Witness, B(BKind::House, 4), "Bartender da Bourbon, vizinho dos Moreau. Bisneto de um estivador. Volta pra casa às três da manhã.")
                .temper(70, 40, 60, 50)
                .topics(vec![
                    topic("noite", "Você viu algo na noite do crime?", "Eu voltava do bar, três e pouco. Um cara saiu pelos fundos dos Moreau e entrou num Crown Victoria sem placa. Jaqueta de polícia, botas, bigodão. Não era o Hale — o Hale eu conheço, ele toma soda no meu balcão. Esse era mais largo.")
                        .lie("Não vi nada, cara. Fone no ouvido, discman no último volume. Não quero confusão com a polícia.", 7)
                        .reveals(&[9]),
                ]),
            person("Leon", "Picard", false, 61, Job::Butcher, Role::Witness, B(BKind::House, 7), "Açougueiro do French Market, quarta geração. O bisavô dele brigava com Cavaretta por fregueses.")
                .temper(40, 60, 60, 40)
                .topics(vec![
                    topic("vincent", "O senhor conhecia Vincent Moreau?", "Comprava linguiça comigo todo sábado. Semana passada me disse: 'Leon, vou parar de pagar os tiras. Se me acontecer alguma coisa, foi o bigodudo, não o Hale.' Eu ri. Achei que era exagero de velho.").reveals(&[8]),
                    topic("machado", "Seu cutelo está limpo?", "Todo mundo olha pro açougueiro. Em 1920 olharam pro meu bisavô também. Pode levar, moço, é sangue de porco. Sempre foi."),
                ]),
        ],
        clues: vec![
            clue("Porta dos fundos arrombada", ClueKind::Physical, Some(HomeOf(0)), Look3d::Object, "O painel inferior da porta foi removido com um cinzel. Exatamente como em 1920 — alguém estudou o método.", &[])
                .forensic("As marcas da alavanca têm a ponta em garra das barras de arrombamento que só as viaturas carregam."),
            clue("Machado de antiquário", ClueKind::Physical, Some(HomeOf(0)), Look3d::Weapon, "Um machado antigo que ficava pendurado na parede como peça de coleção. Foi largado no quintal.", &[])
                .forensic("Sangue humano. No cabo, talco de luva de látex — das caixas que a polícia distribui nas viaturas."),
            clue("E-mail 'Do Inferno'", ClueKind::Document, Some(B(BKind::Newspaper, 0)), Look3d::Paper, "Impresso na redação: 'Serei poupado em toda casa onde tocarem jazz. — Do Inferno.' O mesmo texto de 1920, palavra por palavra.", &[])
                .forensic("Cabeçalho: enviado às 3:17 de um cibercafé na Rua Decatur, por uma conta criada vinte minutos antes."),
            clue("Registro do cibercafé", ClueKind::Document, Some(B(BKind::Office, 2)), Look3d::Paper, "Caderno de clientes: 'Máquina 4 — 2:55 às 3:20 — pago em dinheiro'. O nome assinado: 'W. Hale'.", &[2])
                .forensic("Letra de forma forçada, apertando a caneta. Alguém escreveu o nome de um morto de 1920 para apontar o bisneto.")
                .network(Job::Hacker),
            clue("Anel de pedra vermelha", ClueKind::Physical, Some(HomeOf(0)), Look3d::Object, "Um anel de ouro com pedra vermelha, caído ao lado da cama. Igual ao de Walter Hale nas fotos de 1920.", &[2])
                .forensic("Latão banhado, pedra de vidro. Na parte de dentro: 'AXEMAN TOURS — SOUVENIR'.")
                .herring(),
            clue("Planilha de 'taxas'", ClueKind::Document, Some(B(BKind::Police, 0)), Look3d::Paper, "Uma planilha impressa de um disquete: nomes de antiquários do French Quarter e valores semanais. Ao lado de 'Moreau', em vermelho: RECUSOU.", &[3])
                .forensic("No rodapé da impressão, o nome de usuário do Windows: GTHIBODEAUX.")
                .hidden(2)
                .network(Job::Police),
            clue("Pager de Vincent", ClueKind::Physical, Some(HomeOf(0)), Look3d::Object, "O bipe de Vincent guarda três mensagens da última semana: 'PAGUE OU O MACHADO VOLTA'. A última chegou às 3:17 da madrugada do crime.", &[3])
                .forensic("Todas enviadas do mesmo orelhão — o que fica na calçada da delegacia central."),
            testimony("Lanterna e distintivo", "Denise viu uma lanterna tática e o brilho de um distintivo no cinto do agressor.", &[2, 3]),
            testimony("A taxa das sextas", "Um policial cobrava $200 por semana de Vincent. Ele parou de pagar e ameaçou ir ao jornal — 'foi o bigodudo, não o Hale'.", &[3]),
            testimony("Marcus viu o bigode", "Às 3h, Marcus viu um homem largo, de bigode e jaqueta da polícia, sair pelos fundos e entrar num Crown Victoria sem placa. Não era Hale.", &[3]),
            testimony("A troca de turno", "Raymond: Gil Thibodeaux trocou o turno com ele na noite do crime, com uma desculpa falsa.", &[3]),
            testimony("Sempre é quem cobra", "Clara: 'Nesta cidade o machado sempre fica com quem cobra.'", &[]),
            clue("Eco: a porta dos fundos", ClueKind::Temporal, Some(HomeOf(0)), Look3d::None, "No eco, um homem de jaqueta da polícia força a porta com uma barra. Ele não usa anel. O relógio da cozinha marca 3:17.", &[3]),
            clue("Relógio de 1920", ClueKind::Physical, Some(HomeOf(0)), Look3d::Glow, "O mesmo relógio de cozinha dos Cavaretta, parado às 3:17. No mostrador, riscados a unha: 1921, 1946, 1971 — e um risco novo, ainda com pó: 2001.", &[])
                .hidden(1),
            clue("Botas com lama cinzenta", ClueKind::Physical, Some(HomeOf(3)), Look3d::Prints, "Botas de serviço escondidas na garagem de Gil, com lama cinzenta misturada a serragem.", &[3])
                .forensic("A mesma serragem do canteiro de restauração no quintal dos Moreau. E uma gota de sangue na costura.")
                .hidden(1),
            clue("Eco: o orelhão", ClueKind::Temporal, Some(Near(BKind::Police, 0)), Look3d::None, "No eco, um homem de bigode digita no orelhão em frente à delegacia: 7-2-2-3... PAGUE OU O MACHADO VOLTA. O relógio da delegacia marca 3:17.", &[3]),
        ],
        echoes: vec![
            echo(HomeOf(0), Ghost::Struggle, &["Uma porta range no escuro.", "O facho de uma lanterna. Um distintivo brilha.", "O relógio antigo: 3:17.", "Um machado de colecionador desce da parede."], 12),
            echo(Near(BKind::Police, 0), Ghost::Wait, &["Um orelhão sob a luz de sódio.", "Um homem de bigode olha para os dois lados.", "Bipes. Números. Uma ameaça.", "Ele volta para a delegacia assobiando."], 15),
        ],
        culprit: 3,
        methods: &[
            "Roubou o cutelo do açougue de Picard",
            "Arrombou a porta dos fundos com uma barra da viatura e usou o machado antigo pendurado na parede",
            "Atirou com a arma de serviço e simulou o machado",
            "Raymond Hale repetiu o crime do bisavô, com o anel da família",
        ],
        method: 1,
        motives: &[
            "Ciúme — era amante de Denise",
            "Vender mais passeios da 'Axeman Tours'",
            "Vincent parou de pagar a 'taxa de segurança' e ia entregar o esquema ao jornal",
            "Herdar a casa da Rua Royal",
        ],
        motive: 2,
        anomalies: &[
            "Nenhuma — é apenas um imitador",
            "O relógio de 1920 parou de novo às 3:17 e ganhou um risco novo, 2001: a mesma casa, o mesmo crime, oitenta e um anos depois — a linha do tempo está rebobinando até o começo",
            "O machado é o mesmo de 1920 e ainda tem o sangue de Giuseppe",
            "Denise é Rosa Cavaretta reencarnada",
        ],
        anomaly: 1,
        threads: &[(3, 0), (0, 1), (3, 2), (3, 5), (4, 2), (0, 6)],
        story: "Gil Thibodeaux cobrava 'taxa de segurança' dos antiquários do French Quarter. Quando Vincent Moreau parou de pagar e ameaçou ir ao jornal, Gil trocou o turno com o parceiro, arrombou a porta com a barra da viatura e matou Vincent com o machado de coleção da parede. O e-mail 'Do Inferno', a assinatura 'W. Hale' no cibercafé e o anel de souvenir existiam para uma coisa: fazer a cidade olhar para o bisneto do primeiro Homem do Machado.",
        sphere: "A casa é a mesma. O relógio é o mesmo. O crime é o mesmo. Nova Orleans voltou a 1920 porque a linha vermelha está se enrolando de volta para o ponto de partida. Você está perto do fim — e o fim se parece muito com o começo.",
        on_true: outcome(
            "Gil Thibodeaux foi preso na saída da delegacia, com o pager ainda vibrando. A planilha de 'taxas' foi parar na primeira página. Raymond Hale pediu para ficar com o caso até o julgamento. Na parede da casa da Rua Royal, Denise pendurou o relógio parado — e nunca mais deu corda.",
            "SARGENTO DA POLÍCIA É PRESO COMO O NOVO 'HOMEM DO MACHADO'",
            "...e o chefe de polícia promete investigar a cobrança de 'taxas' no French Quarter...",
            vec![(3, "jailed"), (2, "police"), (1, "survived"), (4, "grateful")],
        )
        .flags(vec!["axeman2_caught"]),
        on_false: outcome(
            "A cidade comemorou a prisão. Na sexta seguinte, um policial de bigode passou recolhendo a 'taxa' nas lojas da Royal. Ninguém recusou.",
            "PRESO O 'NOVO HOMEM DO MACHADO'. OS TURISTAS VOLTAM À ROYAL",
            "...a Axeman Tours registra recorde de passeios nesta temporada...",
            vec![(3, "criminal"), (2, "hates_elias")],
        )
        .flags(vec!["axeman2_free"]),
        digit: "5",
        reward: 105,
    }
}

pub fn case27() -> CaseDef {
    CaseDef {
        id: 27,
        title: "O Terreno Baldio",
        city: CityId::NewOrleans,
        year: 2001,
        intro: "Nova Orleans, 2001. Há um terreno baldio perto do canal onde nada cresce e nenhum cachorro entra. Uma incorporadora comprou o lote para erguer duzentos condomínios de luxo. O agrimensor da prefeitura foi encontrado no fundo de uma cisterna velha, no meio do mato, com o relógio parado às 3:17. Na picape dele havia plantas de um prédio que ninguém encomendou — desenhadas com a sua letra.",
        brief: "Dwayne Fontenot, agrimensor, morreu no terreno baldio que a Aucoin Development quer lotear. Na picape, plantas de um 'Laboratório Delta' escritas com a sua caligrafia.",
        start: B(BKind::Abandoned, 0),
        cast: vec![
            person("Dwayne", "Fontenot", false, 47, Job::Clerk, Role::Victim, B(BKind::House, 6), "Agrimensor da prefeitura. Meticuloso, teimoso, incorruptível. Encontrado no fundo da cisterna do terreno.").dead(),
            person("Preston", "Aucoin", false, 56, Job::Merchant, Role::Suspect, B(BKind::Mansion, 1), "Dono da Aucoin Development. Terno de linho, casaco de pelo de camelo, bengala de castão de prata. Nokia de última geração no bolso.")
                .temper(25, 60, 15, 90)
                .flees()
                .topics(vec![
                    topic("terreno", "Por que esse terreno?", "Porque é o último lote grande perto do canal. Duzentas unidades, piscina, portaria 24 horas. Eu comprei dentro da lei. Quem diz o contrário quer propina."),
                    topic("noite", "Onde o senhor estava na madrugada da morte?", "...Tudo bem. Fui ao terreno. Ele me bipou, queria conversar. Ofereci um acordo, ele recusou. Quando eu fui embora ele estava de pé, gritando comigo. De pé!")
                        .lie("Em casa, dormindo. Minha esposa confirma. Eu não piso naquele mato nem de dia.", 12),
                    topic("bengala", "Sua bengala foi lavada com água sanitária. Por quê?", "Pisei em sujeira de cachorro. Você vai me prender por higiene? Fale com o meu advogado.").req(Req::Clue(11)),
                    topic("escritura", "A página 'de 1971' da escritura foi reimpressa.", "Todo mundo compra um carimbo nesta cidade, rapaz. Eu só paguei mais caro. Isso não é crime de sangue.").req(Req::Clue(4)),
                ]),
            person("Lucille", "Fontenot", true, 44, Job::Nurse, Role::Witness, B(BKind::House, 6), "Viúva de Dwayne. Enfermeira do Charity. Não chorou ainda — está com raiva demais.")
                .temper(50, 70, 80, 20)
                .topics(vec![
                    topic("noite", "Quando a senhora viu Dwayne pela última vez?", "Duas e meia. O bipe tocou, ele leu e ficou branco. Disse: 'vou resolver isso com o homem do terreno de uma vez'. Pegou a lanterna e foi. Eu voltei a dormir. Eu voltei a dormir.").reveals(&[7]),
                    topic("plantas", "E essas plantas na picape?", "Apareceram no para-brisa uma semana atrás, presas no limpador. Com um bilhete: 'Meça direito desta vez.' Dwayne passou três noites olhando pra elas. Disse que a letra parecia a de um homem que ele ainda ia conhecer.").req(Req::Clue(1)),
                ]),
            person("Tyrell", "Guidry", false, 22, Job::Student, Role::Witness, B(BKind::Apartment, 3), "Estagiário de Dwayne, estudante de engenharia na UNO. Carregava o tripé e anotava as medições.")
                .temper(75, 35, 65, 45)
                .topics(vec![
                    topic("bussola", "O que havia de estranho nas medições?", "A bússola do teodolito não apontava pro norte. Apontava pro meio do terreno. Sempre o mesmo ponto, perto da cisterna. E o relógio do equipamento travava às 3:17, mesmo de tarde."),
                    topic("dinheiro", "Alguém ofereceu dinheiro a vocês?", "O Aucoin. Vinte mil dólares pro Dwayne 'arredondar' a divisa do lote. Dwayne rasgou o cheque na frente dele. Disse que o terreno não era do vendedor, que tinha um dono desde 1971.")
                        .lie("Não sei de proposta nenhuma. Eu só carrego o tripé.", 3)
                        .reveals(&[8]),
                ]),
            person("Earl", "Boudreaux", false, 61, Job::Drifter, Role::Suspect, B(BKind::House, 12), "Veterano do Vietnã. Dorme num barraco ao lado do terreno. A polícia já decidiu que foi ele.")
                .temper(60, 40, 60, 30)
                .topics(vec![
                    topic("noite", "O senhor viu alguma coisa naquela noite?", "Vi. Um Lexus prata parou às três. Desceu um cara de casaco de camelo e bengala. Discutiu com o agrimensor lá perto da cisterna. Depois só o Lexus foi embora. E tinha outro... um velho de sobretudo, do outro lado da cerca, só olhando.")
                        .lie("Não vi nada. Eu tava bêbado, dormindo. Pergunte pra garrafa.", 13)
                        .reveals(&[9]),
                    topic("velho", "Quem é o velho de sobretudo?", "Aparece toda noite há um mês. Fica parado no meio do mato falando sozinho: 'aqui vai a escada, aqui a sala blindada, aqui ela fica'. Uma vez ele me deu vinte dólares e disse que eu ia ver o prédio pronto. Eu ri. Ele não.").req(Req::Topic("noite")),
                ]),
            person("Marianne", "Leblanc", true, 38, Job::Clerk, Role::Suspect, B(BKind::Apartment, 7), "Escrivã do cartório de registro de imóveis. Carro novo, bolsa nova, olheiras novas.")
                .temper(70, 30, 40, 70)
                .topics(vec![
                    topic("escritura", "A escritura do terreno é legítima?", "Não. Aucoin me pagou quinze mil dólares pra trocar uma página. O registro verdadeiro dizia que o lote tem dono desde 1971, e que não pode ser vendido. Eu reimprimi a página e carimbei. Eu tenho uma filha, entende?")
                        .lie("Perfeitamente legítima. Eu mesma conferi cada carimbo.", 4)
                        .reveals(&[10]),
                    topic("dono", "Quem é o dono de 1971?", "Estava escrito: 'E. Vale — Projeto Delta'. Eu achei que era erro de datilografia. Aí você entrou aqui e disse seu nome.").req(Req::Clue(3)),
                ]),
        ],
        clues: vec![
            clue("Corpo na cisterna", ClueKind::Physical, Some(B(BKind::Abandoned, 0)), Look3d::Blood, "Dwayne no fundo de uma cisterna de tijolos, no centro do terreno. O relógio Casio no pulso parou às 3:17.", &[])
                .forensic("Pancada na nuca, antes da queda, com um objeto de ponta arredondada e pesada. Na gola da camisa, fibras de pelo de camelo."),
            clue("Plantas na sua letra", ClueKind::Document, Some(Near(BKind::Abandoned, 0)), Look3d::Glow, "Na picape: plantas de um prédio de três andares com subsolo blindado. No carimbo: LABORATÓRIO DELTA. A letra é a sua — o mesmo 'a' fechado, a mesma seta de sublinhar.", &[])
                .forensic("Papel vegetal com marca d'água de um fabricante que só vai existir em 2029.")
                .hidden(1),
            clue("Teodolito enlouquecido", ClueKind::Physical, Some(B(BKind::Abandoned, 0)), Look3d::Glow, "A bússola do teodolito aponta para o centro do terreno, não para o norte. A última medição gravada: 03:17:00.", &[]),
            clue("Relatório de Dwayne", ClueKind::Document, Some(HomeOf(0)), Look3d::Paper, "'A divisa não bate com a escritura. O vendedor não é o dono. Registro original de 1971 em nome de E. VALE — PROJETO DELTA. Venda nula. Vou embargar.'", &[1, 5]),
            clue("Escritura adulterada", ClueKind::Document, Some(B(BKind::Office, 0)), Look3d::Paper, "A escritura de 1998 que passa o lote para a Aucoin Development. Carimbo e rubrica de Marianne Leblanc.", &[1, 5])
                .forensic("A página 'de 1971' foi impressa numa jato de tinta moderna. O papel é mais branco que o resto do livro.")
                .network(Job::Lawyer),
            clue("Garrafa e cobertor", ClueKind::Physical, Some(B(BKind::Abandoned, 0)), Look3d::Object, "Perto da cisterna: uma garrafa de bourbon barato e um cobertor com manchas escuras. A polícia já ensacou como prova contra o mendigo.", &[4])
                .forensic("As manchas são de vinho tinto e ferrugem. Não há sangue.")
                .herring(),
            clue("Bipe de Dwayne", ClueKind::Physical, Some(B(BKind::Abandoned, 0)), Look3d::Object, "O pager alfanumérico, preso no cinto do morto. Última mensagem, 2:24: 'TERRENO 3H. VAMOS RESOLVER ISSO. — P.A.'", &[1]),
            testimony("O bipe das duas e meia", "Lucille: Dwayne recebeu um bipe às 2:30 e saiu para 'resolver com o homem do terreno'.", &[1]),
            testimony("Vinte mil dólares", "Tyrell: Aucoin ofereceu $20.000 para Dwayne 'arredondar' a divisa. Dwayne rasgou o cheque.", &[1]),
            testimony("O Lexus prata", "Earl viu, às 3h, um homem de casaco de camelo e bengala discutir com Dwayne perto da cisterna. E um velho de sobretudo olhando de longe.", &[1]),
            testimony("A página trocada", "Marianne trocou a página do registro de 1971 por $15.000, a pedido de Aucoin.", &[1, 5]),
            clue("Bengala de castão de prata", ClueKind::Physical, Some(HomeOf(1)), Look3d::Weapon, "A bengala de Aucoin, no porta-guarda-chuvas. O castão brilha demais — foi esfregado.", &[1])
                .forensic("Lavado com água sanitária. Mas na rosca do castão ainda há sangue seco e um fio de cabelo grisalho.")
                .hidden(1),
            clue("Registro do celular", ClueKind::Document, Some(B(BKind::Office, 1)), Look3d::Paper, "Relatório da operadora: o celular de Aucoin se conectou à torre do canal às 2:58 e às 3:21 da madrugada do crime.", &[1])
                .network(Job::Hacker),
            clue("Eco: a cisterna", ClueKind::Temporal, Some(B(BKind::Abandoned, 0)), Look3d::None, "No eco, dois homens discutem sobre papéis. Uma bengala sobe. Um relógio digital pisca 3:17. Do outro lado da cerca, um bêbado se esconde atrás de um barril.", &[1]),
            clue("Eco: a pedra fundamental", ClueKind::Temporal, Some(B(BKind::Abandoned, 0)), Look3d::None, "Num eco que ainda não aconteceu, um homem grisalho crava uma estaca no centro do terreno e escreve DELTA no concreto fresco. É você.", &[]),
        ],
        echoes: vec![
            echo(B(BKind::Abandoned, 0), Ghost::Argue, &["Mato alto, uma lanterna no chão.", "'Você não vai embargar nada, Fontenot.'", "Uma bengala de prata sobe.", "O relógio digital: 3:17."], 13),
            echo(Near(BKind::Abandoned, 0), Ghost::Write, &["Concreto fresco sob a lua.", "Um homem de cabelo branco se ajoelha.", "Ele escreve com o dedo: D-E-L-T-A.", "Ergue o rosto. É o seu."], 14),
        ],
        culprit: 1,
        methods: &[
            "O mendigo o empurrou na cisterna para roubar a lanterna",
            "Aucoin o golpeou na nuca com a bengala de prata e jogou o corpo na cisterna",
            "Queda acidental no escuro",
            "Tyrell o atingiu com o tripé do teodolito",
        ],
        method: 1,
        motives: &[
            "Roubo do equipamento",
            "Ciúme de Lucille",
            "Dwayne descobriu que a escritura foi adulterada e ia embargar o empreendimento",
            "Dívida de jogo",
        ],
        motive: 2,
        anomalies: &[
            "Nenhuma — é só uma briga por terra",
            "O teodolito vem do futuro",
            "O lote pertence a 'E. Vale — Projeto Delta' desde 1971, as plantas estão na sua letra e a bússola aponta para o ponto exato onde a esfera vai ficar: é aqui que você vai construir o Laboratório Delta",
            "Dwayne era uma versão sua de outra linha do tempo",
        ],
        anomaly: 2,
        threads: &[(1, 0), (1, 5), (0, 2), (0, 3), (4, 0), (4, 1)],
        story: "Dwayne Fontenot descobriu que o terreno não podia ser vendido: o registro original, de 1971, tinha outro dono. Aucoin já tinha pago uma escrivã para trocar a página e comprado o lote pela metade do preço. Quando Dwayne recusou os vinte mil dólares e anunciou o embargo, Aucoin o chamou ao terreno às três da manhã, golpeou sua nuca com a bengala e jogou o corpo na cisterna. O mendigo do barraco ao lado era o culpado perfeito.",
        sphere: "O terreno não tem dono porque o dono ainda não nasceu para ele. Você vai comprar este lote com outro nome, erguer o Laboratório Delta com estas plantas e pôr a esfera exatamente onde a bússola aponta. O velho que deixou as plantas no para-brisa sabia disso. Ele desenhou com a sua mão.",
        on_true: outcome(
            "Preston Aucoin foi preso no aeroporto com uma passagem só de ida para Cancún. Marianne Leblanc entregou o registro original. A venda foi anulada e o terreno voltou a ser de ninguém — ou de alguém que ainda não chegou. A prefeitura cercou o lote. Earl continuou dormindo ao lado, agora com um cobertor novo.",
            "INCORPORADOR É PRESO PELA MORTE DE AGRIMENSOR DA PREFEITURA",
            "...o empreendimento Canal Gardens foi embargado pela justiça...",
            vec![(1, "jailed"), (5, "jailed"), (2, "grateful"), (4, "survived"), (3, "grateful")],
        )
        .flags(vec!["lot_saved"]),
        on_false: outcome(
            "O acusado foi condenado em dois dias, sem advogado que prestasse. Em seis meses, as máquinas da Aucoin Development chegaram ao terreno. Os operários se recusaram a trabalhar depois das três da manhã. Ninguém soube explicar por quê.",
            "CASO DO TERRENO BALDIO: POLÍCIA APONTA CRIME DE RUA",
            "...a Aucoin Development anuncia o lançamento do Canal Gardens...",
            vec![(1, "criminal"), (2, "hates_elias")],
        )
        .flags(vec!["lot_sold"]),
        digit: "3",
        reward: 108,
    }
}

pub fn case28() -> CaseDef {
    CaseDef {
        id: 28,
        title: "A Mulher que Esperou",
        city: CityId::NewOrleans,
        year: 2001,
        intro: "Nova Orleans, 2001. Clara Bell, cento e doze anos, a mulher mais velha da Louisiana, morreu às 3:17 num quarto da enfermaria. A certidão diz 'causas naturais'. Mas ela ia assinar um testamento novo às nove da manhã. No apartamento dela, na mesma rua desde 1920, há caixas de sapato com oitenta anos de cartas — todas endereçadas a você.",
        brief: "Clara Bell, 112 anos, morreu às 3:17 na véspera de assinar um novo testamento. Deixou oitenta anos de cartas endereçadas a 'Elias'.",
        start: WorkOf(2),
        cast: vec![
            person("Clara", "Bell", true, 112, Job::Retired, Role::Victim, B(BKind::Apartment, 5), "Ex-repórter. Viu o século inteiro passar. Morreu segurando uma caneta.")
                .dead()
                .soul(1),
            person("Bradley", "Bell", false, 36, Job::Banker, Role::Suspect, B(BKind::Apartment, 8), "Sobrinho-neto de Clara. Corretor de ações, anel de formatura da Tulane, celular Motorola StarTAC. Perdeu tudo na bolha das pontocom.")
                .temper(40, 50, 20, 90)
                .flees()
                .topics(vec![
                    topic("tia", "Como era sua relação com Clara?", "Tia Clara era a matriarca. A família inteira esperava ela morrer desde que eu era criança. Ela nunca morria. Piada de família: 'a tia está esperando o namorado'."),
                    topic("noite", "Onde você estava na madrugada da morte?", "Tá bom, eu fui lá. Duas e quarenta. Queria convencer ela a desistir daquela loucura de testamento. Ela nem me ouviu. Estava rindo, conversando com um velho. Quando eu saí ela estava viva!")
                        .lie("Em casa, vendo reprise na TV a cabo. Sozinho. Isso não é crime.", 3),
                    topic("dinheiro", "Como estão suas finanças?", "...Trezentos e quarenta mil negativos. Chamada de margem na sexta. Aquele apartamento vale uma fortuna no Quarter. E as cartas — um colecionador de Nova York ofereceu cinquenta mil só pelas cartas. Era meu por direito!")
                        .lie("Ótimas. O mercado se recupera, é cíclico.", 6),
                    topic("cartas", "O que você sabe das cartas de Clara?", "Cartas de uma velha maluca pra um namorado imaginário. Oitenta anos escrevendo pra um homem que nunca veio. ...Por que você está me olhando assim?").req(Req::Clue(1)),
                ]),
            person("Ines", "Mouton", true, 29, Job::Nurse, Role::Suspect, B(BKind::House, 10), "Enfermeira do plantão noturno. Neta de uma camareira do velho Hotel Saint-Aubin. Faz dois turnos seguidos há um mês.")
                .temper(70, 40, 75, 30)
                .topics(vec![
                    topic("plantao", "Você passou no quarto de Clara durante a noite?", "...Eu dormi. Das duas e meia às três e vinte, na sala do café. Acordei e o alarme da bomba de infusão estava silenciado — só dá pra silenciar no painel. Vi um homem jovem, de terno, descendo pela escada de incêndio. Eu não contei porque ia perder o emprego.")
                        .lie("Fiz a ronda de hora em hora, como manda o protocolo. Não vi nada fora do normal.", 0)
                        .reveals(&[7]),
                    topic("clara", "Como era Clara?", "Ela pedia café às três da manhã. Todo dia. Dizia: 'caso seja hoje'. E me perguntava se um homem de olhos cansados tinha vindo. Um dia eu perguntei o nome. Ela disse o seu."),
                    topic("frasco", "O frasco de morfina vazio no seu armário.", "Lote vencido. Eu devia ter jogado fora faz uma semana, está no livro de descarte, assinado por duas pessoas. Pode conferir.").req(Req::Clue(5)),
                ]),
            person("Harriet", "Voss", true, 60, Job::Lawyer, Role::Contact, B(BKind::House, 2), "Tabeliã. Ia registrar o novo testamento de Clara às nove da manhã.")
                .temper(30, 60, 80, 30)
                .topics(vec![
                    topic("testamento", "O que mudava no novo testamento?", "Tudo. O apartamento e as cartas iam para um único herdeiro. O sobrinho-neto ficava sem nada. E ele sabia: me ligou ontem à tarde, do celular, pra confirmar o horário da assinatura. Nove em ponto.").reveals(&[10]),
                    topic("herdeiro", "Quem é o herdeiro?", "'Elias Vale, que chegará em 2001.' Ela me fez escrever assim, exatamente. O senhor tem algum documento?").req(Req::Clue(2)),
                ]),
            person("Jerome", "Toussaint", false, 78, Job::Musician, Role::Witness, B(BKind::Apartment, 9), "Trompetista aposentado. Neto de Lucien Toussaint. Visitava Clara todas as noites para tocar baixinho no quarto.")
                .temper(40, 50, 75, 20)
                .topics(vec![
                    topic("clara", "O senhor visitava Clara?", "Toda noite, desde que meu avô morreu. Ontem o sobrinho dela apareceu gritando por causa do testamento. Disse: 'se a senhora assinar isso, eu não respondo por mim'. Ela nem piscou.").reveals(&[8]),
                    topic("velho", "Ela recebeu outra visita?", "Recebeu. Às dez pras três, um velho de sobretudo. Ela riu, riu como moça. Ele segurou a mão dela e chorou. Saiu às três em ponto. Tinha o seu rosto, rapaz. Só que gasto.").req(Req::Topic("clara")).reveals(&[9]),
                    topic("avo", "Seu avô conheceu Clara?", "Lucien tocou pra ela em 1920, numa noite em que a cidade inteira tocava jazz com medo do machado. Ele dizia que ela passou a vida esperando um homem que tinha 'lido o final do livro'."),
                ]),
            person("Dana", "Keller", true, 33, Job::Journalist, Role::Witness, B(BKind::Apartment, 4), "Repórter de uma revista de variedades. Queria escrever sobre 'a mulher que viu o século inteiro'.")
                .temper(35, 70, 55, 50)
                .topics(vec![
                    topic("cartas", "Por que você se interessa pelas cartas?", "Porque alguém tentou vendê-las pra mim. O sobrinho, anteontem. Cinquenta mil dólares. Disse que elas 'previam o futuro' — que falavam de um incêndio em Chicago, de um trem, de um avião que sumiu. Eu achei que era golpe. Agora não sei.").reveals(&[11]),
                    topic("clara", "Você chegou a entrevistar Clara?", "Uma vez. Ela disse: 'Escreva sobre ele, não sobre mim. Eu só esperei. Ele é que atravessou.' Depois pediu que eu fosse embora porque 'ele podia chegar a qualquer momento'."),
                ]),
        ],
        clues: vec![
            clue("Bomba de infusão reprogramada", ClueKind::Physical, Some(WorkOf(2)), Look3d::Object, "A bomba de morfina de Clara foi reprogramada: dez vezes a dose, liberada às 3:17. O alarme foi silenciado no painel.", &[])
                .forensic("O painel foi destravado com o código de fábrica, o que vem impresso no manual. A enfermagem usa o código do hospital. Quem mexeu não era da casa."),
            clue("Oitenta anos de cartas", ClueKind::Document, Some(HomeOf(0)), Look3d::Glow, "Caixas de sapato com milhares de cartas. 'Querido Elias, hoje é 1921...' 'Querido Elias, hoje é 1946...' A última, de ontem: 'Ele voltou. O velho. Disse que você chega amanhã.'", &[]),
            clue("Rascunho do testamento", ClueKind::Document, Some(B(BKind::Office, 0)), Look3d::Paper, "'Deixo meu apartamento e minhas cartas a Elias Vale, que chegará em 2001.' Assinatura marcada para as 9h.", &[1])
                .network(Job::Lawyer),
            clue("Livro de visitas", ClueKind::Document, Some(WorkOf(2)), Look3d::Paper, "Na recepção da enfermaria: '2:41 — B. Bell (sobrinho-neto)'. Logo abaixo, '2:50 — E. Vale'. Nenhuma saída registrada para B. Bell.", &[1]),
            clue("Luvas e manual na lixeira", ClueKind::Physical, Some(Alley(2)), Look3d::Object, "Na lixeira do beco atrás da enfermaria: um par de luvas de látex e o manual da bomba de infusão, com a página do código marcada.", &[1])
                .forensic("Dentro de uma das luvas, a marca de um anel grande, de formatura. Tulane, turma de 87.")
                .hidden(1),
            clue("Frasco de morfina vazio", ClueKind::Physical, Some(WorkOf(2)), Look3d::Object, "Um frasco de morfina vazio no armário da enfermeira da noite.", &[2])
                .forensic("Lote vencido, com baixa no livro de descarte há uma semana. A morfina da bomba de Clara é de outro lote.")
                .herring(),
            clue("Extratos de Bradley", ClueKind::Document, Some(HomeOf(1)), Look3d::Paper, "Extratos da corretora: saldo negativo de $340.000. Uma chamada de margem vence sexta-feira. Ao lado, o cartão de um colecionador de manuscritos de Nova York.", &[1])
                .hidden(1)
                .network(Job::Banker),
            testimony("O homem da escada", "Ines dormiu das 2:30 às 3:20; o alarme da bomba foi silenciado no painel e um homem jovem de terno fugiu pela escada de incêndio.", &[1]),
            testimony("A ameaça do sobrinho", "Jerome: na véspera, Bradley gritou com Clara — 'se a senhora assinar isso, eu não respondo por mim'.", &[1]),
            testimony("O velho que chorou", "Jerome: às 2:50 um velho de sobretudo, com o rosto de Elias, visitou Clara. Ela riu. Ele chorou. Saiu às 3h.", &[]),
            testimony("O horário da assinatura", "Harriet: Bradley ligou na véspera para confirmar o horário exato da assinatura do testamento.", &[1]),
            testimony("Cartas à venda", "Dana: Bradley tentou vender as cartas por $50.000 — 'elas preveem o futuro'.", &[1]),
            clue("Eco: o quarto", ClueKind::Temporal, Some(WorkOf(2)), Look3d::None, "No eco, um homem jovem de luvas digita no painel da bomba. A velha abre os olhos e diz: 'Elias?'. Ele não responde. O relógio da parede: 3:17.", &[1]),
            clue("Eco: 1920", ClueKind::Temporal, Some(HomeOf(0)), Look3d::None, "No eco, uma jovem de chapéu cloche escreve à luz de vela: 'Querido Elias, hoje você foi embora'. Ela levanta os olhos e sorri — para você.", &[]),
            clue("A carta para o dia 30", ClueKind::Document, Some(HomeOf(0)), Look3d::Glow, "Escondida no forro de uma caixa: 'Para Elias, no último caso. Não o deixe fechar a linha sozinho. O velho do sobretudo é você. Eu sei porque ele chorou igualzinho.'", &[])
                .hidden(2),
        ],
        echoes: vec![
            echo(WorkOf(2), Ghost::Hide, &["Um corredor de hospital, luz verde.", "Um homem de terno calça luvas.", "Bipes de uma máquina. Depois, silêncio.", "'Elias?' O relógio: 3:17."], 12),
            echo(HomeOf(0), Ghost::Write, &["Uma vela. Um mata-borrão.", "Uma pena arranha o papel: 'Querido Elias'.", "O calendário na parede diz 1920.", "Ela olha para você. Sorri. Continua escrevendo."], 13),
        ],
        culprit: 1,
        methods: &[
            "A enfermeira aplicou uma 'dose de misericórdia'",
            "Bradley reprogramou a bomba de morfina com o código de fábrica do manual",
            "Morte natural aos 112 anos",
            "O velho do sobretudo a sufocou com o travesseiro",
        ],
        method: 1,
        motives: &[
            "Herdar o apartamento e vender as cartas antes que o novo testamento deixasse tudo para Elias Vale",
            "Piedade — ela sofria",
            "Vingança por uma briga de família antiga",
            "Ela sabia de um crime de Bradley",
        ],
        motive: 0,
        anomalies: &[
            "Nenhuma — ela era muito velha",
            "As cartas foram escritas por Bradley para valorizar a herança",
            "Clara era o Outro Elias disfarçado",
            "Nesta linha do tempo Clara não reencarnou: a esfera a manteve viva por oitenta e um anos, esperando você — e as cartas descrevem casos que você ainda nem tinha resolvido",
        ],
        anomaly: 3,
        threads: &[(1, 0), (0, 4), (0, 3), (1, 5), (2, 0), (1, 3)],
        story: "Bradley Bell estava falido e contava com a herança da tia-avó. Quando soube que Clara ia deixar o apartamento e as cartas para um tal 'Elias Vale', ligou para a tabeliã para saber o horário da assinatura, entrou na enfermaria às 2:41, esperou a visita do velho sair e reprogramou a bomba de morfina com o código do manual. Às 3:17, Clara Bell morreu chamando o seu nome.",
        sphere: "Em todas as outras linhas do tempo ela voltou com outro nome — enfermeira, locutora, repórter. Nesta, ela se recusou a ir embora. A esfera a segurou aqui por oitenta e um anos, porque alguém precisava lembrar de você do lado de cá. Ela esperou até a véspera. O velho do sobretudo foi se despedir dela antes de você chegar. Ele sabia a hora.",
        on_true: outcome(
            "Bradley Bell foi preso tentando embarcar as caixas de cartas num FedEx para Nova York. O testamento foi reconhecido como a última vontade de Clara. Harriet Voss entregou a você uma chave e uma caixa de sapato. Jerome tocou no enterro. Tocou o jazz de 1920.",
            "SOBRINHO-NETO É PRESO PELA MORTE DA MULHER MAIS VELHA DA LOUISIANA",
            "...Clara Bell, 112 anos, foi enterrada ontem no cemitério St. Louis nº 1, ao som de trompete...",
            vec![(1, "jailed"), (2, "grateful"), (4, "grateful"), (5, "journalist"), (3, "grateful")],
        )
        .flags(vec!["clara_letters"]),
        on_false: outcome(
            "A enfermeira foi indiciada. Bradley herdou o apartamento e leiloou as cartas em lotes, pela internet. Ninguém comprou a última. Ficou no site por meses: 'Para Elias, no último caso'.",
            "ENFERMEIRA É ACUSADA DE MATAR IDOSA DE 112 ANOS",
            "...as cartas de Clara Bell vão a leilão online nesta semana...",
            vec![(2, "jailed"), (1, "criminal"), (4, "hates_elias")],
        )
        .flags(vec!["clara_letters_lost"]),
        digit: "1",
        reward: 112,
    }
}

pub fn case29() -> CaseDef {
    CaseDef {
        id: 29,
        title: "Elias Vale, Desaparecido",
        city: CityId::NewOrleans,
        year: 2001,
        intro: "Nova Orleans, 2001. Amanheceram cartazes em todos os postes do French Quarter: DESAPARECIDO — ELIAS VALE. A foto é a sua. A data do desaparecimento: fevereiro de 2025. Quem colou os cartazes foi um rapaz chamado Kevin Arnaud, que mantinha um fórum na internet sobre 'o homem que aparece às 3:17'. Esta madrugada, Kevin foi encontrado enforcado no próprio apartamento. O computador dele estava apagado.",
        brief: "Kevin Arnaud, hacker e fundador do fórum 'Fio Vermelho', morreu às 3:17 depois de espalhar cartazes de desaparecido com o seu rosto, datados de 2025. A polícia diz suicídio.",
        start: HomeOf(0),
        cast: vec![
            person("Kevin", "Arnaud", false, 27, Job::Hacker, Role::Victim, B(BKind::Apartment, 13), "Hacker, fundador do fórum Fio Vermelho. Juntava relatos de famílias que viram 'o homem das 3:17'. Encontrado enforcado.").dead(),
            person("Russell", "Crane", false, 48, Job::Detective, Role::Suspect, B(BKind::Hotel, 1), "Agente federal de uma divisão que não aparece em nenhum organograma. Terno cinza, crachá plastificado, hospedado num hotel do centro.")
                .temper(15, 85, 20, 60)
                .flees()
                .topics(vec![
                    topic("kevin", "O senhor conhecia Kevin Arnaud?", "Tá. Eu fui atrás do garoto. Ele estava publicando material classificado — coisa de 1919, de 1971, do Projeto Vermilion. É assunto de segurança nacional, Vale. Você devia saber: você é o assunto.")
                        .lie("Nunca ouvi falar desse rapaz. Estou na cidade para uma conferência sobre fraude bancária.", 9),
                    topic("noite", "Onde o senhor estava às três da manhã?", "Estive no apartamento. Conversamos. Ele estava vivo quando eu saí — pergunte ao computador dele. Ah, é. Não dá mais.")
                        .lie("No hotel. Dormindo. O porteiro da noite pode confirmar.", 7)
                        .req(Req::Topic("kevin")),
                    topic("pasta", "O que é esta pasta com fotos minhas em seis décadas?", "É o trabalho de três gerações de agentes. Desde 1928 a gente tenta pegar você antes da esfera. Você não devia existir, Vale. E em 2025 você vai deixar de existir. Está no cartaz, não está?").req(Req::Clue(13)),
                ]),
            person("Tasha", "Arnaud", true, 24, Job::Student, Role::Witness, B(BKind::Apartment, 11), "Irmã de Kevin. Estudante de enfermagem. Diz que o irmão nunca se mataria 'no meio de uma descoberta'.")
                .temper(55, 65, 80, 20)
                .topics(vec![
                    topic("irmao", "Quando você falou com Kevin pela última vez?", "Uma da manhã, pelo celular. Ele estava sussurrando. Disse que um cara de terno seguia ele desde a copiadora e que tinha estacionado na frente do prédio. Disse: 'se eu sumir, olha o disquete'.").reveals(&[8]),
                    topic("cartaz", "De onde veio o cartaz?", "De um fax. Chegou na copiadora onde ele imprimia o fanzine, às 3:17 de uma terça. Sem número de origem. Só a sua cara e 'DESAPARECIDO'. Ele passou a noite comparando com as fotos do fórum. Você aparece em todas, Elias. Em todas as décadas."),
                ]),
            person("Minh", "Phan", false, 52, Job::Merchant, Role::Witness, B(BKind::Apartment, 10), "Dono da copiadora da Rua Magazine. Imprimiu os quinhentos cartazes. Não quer mais saber do assunto.")
                .temper(75, 35, 70, 40)
                .topics(vec![
                    topic("copias", "Alguém perguntou sobre os cartazes?", "Um homem de terno, ontem à tarde. Mostrou um distintivo federal rápido demais pra eu ler. Quis saber quem encomendou, pagou em dinheiro por uma cópia da nota fiscal, com o endereço do Kevin.").reveals(&[9]),
                    topic("fax", "O fax com o cartaz chegou aqui?", "Chegou. Às três e dezessete da madrugada, com a loja fechada. Eu achei no chão de manhã. A máquina nem tinha papel naquela hora, moço. Eu tinha tirado tudo pra limpar."),
                ]),
            person("Henri", "Duval", false, 45, Job::Teacher, Role::Suspect, B(BKind::House, 5), "Henri Duval III, professor de história. Neto do menino que sumiu em 1922. Moderador do fórum. Brigou com Kevin por causa da publicação.")
                .temper(50, 55, 75, 25)
                .topics(vec![
                    topic("rede", "O que é a Rede?", "Famílias. A minha, a dos Cavaretta, a dos Marsh, dezenas. Cada uma tem uma história de um homem chamado Elias que apareceu às 3:17 e mudou tudo. Meu avô foi tirado de um orfanato por sua causa. Kevin juntou todos nós num fórum. Nós somos o que sobrou das suas escolhas.").reveals(&[11]),
                    topic("emails", "Você ameaçou Kevin?", "Ameacei, sim. Escrevi coisas horríveis. Eu tinha medo — se ele publicasse, eles viriam atrás de todos nós. E vieram. Mas eu estava em Baton Rouge, dando aula de recuperação. Tenho trinta alunos de testemunha.")
                        .lie("Nunca brigamos. A Rede é uma família, a gente discorda com respeito.", 6),
                ]),
            person("Odette", "Laveau", true, 74, Job::Retired, Role::Witness, B(BKind::House, 9), "Dona do prédio onde Kevin morava. Aluga o mesmo apartamento desde 1965 e tem memória de elefante.")
                .temper(30, 60, 70, 40)
                .topics(vec![
                    topic("inquilino", "Quem morou no apartamento antes de Kevin?", "Em 1971, um senhor grisalho chamado E. Vale. Pagava adiantado, em dinheiro, e colava papéis nas paredes com linha vermelha. Um dia sumiu e deixou tudo emparedado. E ontem, juro por Deus, um velho de sobretudo veio perguntar se o apartamento estaria vago 'em 2025'.").reveals(&[10]),
                    topic("parede", "Tem papéis atrás do reboco.", "Eu nunca mandei abrir. Meu marido dizia que dava azar. Aquele homem me disse, em 1971: 'um dia eu mesmo venho buscar'.").req(Req::Clue(5)),
                ]),
        ],
        clues: vec![
            clue("Cartaz de desaparecido", ClueKind::Document, Some(Near(BKind::Bar, 2)), Look3d::Glow, "DESAPARECIDO — ELIAS VALE. Visto pela última vez em fevereiro de 2025, no Laboratório Delta. A foto é a sua, com a barba de hoje.", &[])
                .forensic("Impresso a laser na copiadora da Rua Magazine. Mas a foto original tem uma resolução que nenhuma câmera digital de 2001 alcança."),
            clue("Corpo de Kevin", ClueKind::Physical, Some(HomeOf(0)), Look3d::Blood, "Kevin pendurado por um cabo de telefone no cano do teto. A cadeira caída não alcançaria os pés dele.", &[1])
                .forensic("Duas marcas no pescoço: uma horizontal, de estrangulamento, e outra em V, da forca. Ele foi estrangulado antes de ser pendurado."),
            clue("Computador apagado", ClueKind::Physical, Some(HomeOf(0)), Look3d::Object, "Um PC bege com monitor CRT ainda ligado. O HD foi apagado às 3:17. Um ímã pesado está grudado no gabinete.", &[1])
                .forensic("O ímã tem uma etiqueta de patrimônio: 'PROPRIEDADE DO GOVERNO DOS EUA — DIVISÃO D'."),
            clue("Disquete escondido", ClueKind::Document, Some(HomeOf(0)), Look3d::Paper, "Um disquete colado com fita embaixo da gaveta. Um único arquivo de texto: 'Um agente chamado Crane esteve aqui. Se eu aparecer morto, foi ele. Backup do fórum no servidor da UNO.'", &[1])
                .hidden(2),
            clue("Log do fórum", ClueKind::Document, Some(B(BKind::Office, 2)), Look3d::Paper, "Último post de Kevin, 2:59: 'um cara de terno está lá embaixo. Amanhã publico tudo.' Às 3:31 alguém entrou na conta dele e apagou 400 mensagens — de uma conexão discada do Hotel Monteleone.", &[1])
                .network(Job::Hacker),
            clue("Anotações atrás do reboco", ClueKind::Document, Some(HomeOf(0)), Look3d::Glow, "Onde o reboco rachou: papéis amarelados de 1971, na SUA letra, ligados por fio vermelho. Trinta recortes. No centro: 'Não pare no 30. O 30 é você.'", &[])
                .hidden(1),
            clue("E-mails de Henri", ClueKind::Document, Some(HomeOf(4)), Look3d::Paper, "Impressões de e-mails de Henri para Kevin: 'Se você publicar, mata todos nós.' 'Eu mesmo vou aí te impedir.'", &[4])
                .forensic("Os cabeçalhos mostram que o último foi enviado de um laboratório de informática em Baton Rouge, às 3:05 — a cento e trinta quilômetros daqui.")
                .herring(),
            clue("Cartão-chave de hotel", ClueKind::Physical, Some(HomeOf(0)), Look3d::Object, "Um cartão magnético caído atrás da porta. Hotel Monteleone, quarto 317.", &[1])
                .hidden(1),
            testimony("O homem de terno", "Tasha: à 1h, Kevin disse ao celular que um homem de terno o seguia desde a copiadora e estava parado na frente do prédio.", &[1]),
            testimony("O distintivo na copiadora", "Phan: um homem de terno com distintivo federal pagou em dinheiro pela nota com o endereço de Kevin.", &[1]),
            testimony("O inquilino de 1971", "Odette: em 1971 um senhor grisalho chamado E. Vale morou no apartamento e colava papéis com linha vermelha. Ontem, um velho perguntou se estaria vago 'em 2025'.", &[]),
            testimony("A Rede", "Henri: a Rede são as famílias que Elias salvou ao longo do século. Kevin as reuniu.", &[]),
            clue("Eco: o apartamento", ClueKind::Temporal, Some(HomeOf(0)), Look3d::None, "No eco, um homem de terno aperta um cabo de telefone. O monitor CRT pisca: 3:17. Lá fora, um cartaz com o seu rosto se solta do poste.", &[1]),
            clue("Pasta 'Vermilion/Delta'", ClueKind::Document, Some(HomeOf(1)), Look3d::Photo, "No quarto 317 do hotel: uma pasta 'VALE, ELIAS — LOCALIZAR ANTES DE 2025'. Fotos suas em 1920, 1946, 1971, 1989. Carimbos do Projeto Vermilion.", &[1])
                .forensic("As fotos mais antigas foram tiradas por agentes diferentes, em câmeras diferentes. Sempre às 3:17, pelo relógio no fundo.")
                .hidden(1),
            clue("Eco: o fax", ClueKind::Temporal, Some(WorkOf(3)), Look3d::None, "No eco, a copiadora escura. Um fax sem papel começa a imprimir. Uma folha sai: o seu rosto. O relógio de parede marca 3:17.", &[]),
        ],
        echoes: vec![
            echo(HomeOf(0), Ghost::Struggle, &["Um monitor verde na escuridão.", "Um homem de terno por trás de uma cadeira.", "Um cabo de telefone se estica.", "O relógio do Windows: 3:17 AM."], 12),
            echo(WorkOf(3), Ghost::Vanish, &["Uma copiadora fechada, as grades baixadas.", "Um fax sem papel começa a zumbir.", "Uma folha desliza até o chão.", "DESAPARECIDO. Você."], 14),
        ],
        culprit: 1,
        methods: &[
            "Suicídio — ele se enforcou com o cabo do telefone",
            "Crane o estrangulou com o cabo do telefone, simulou o enforcamento e apagou o HD com um ímã",
            "Henri Duval III o envenenou e montou a cena",
            "Choque elétrico no computador",
        ],
        method: 1,
        motives: &[
            "Roubar o computador",
            "Ciúme por causa da irmã",
            "Uma briga de fórum que saiu do controle",
            "Silenciar a Rede e confiscar tudo sobre Elias Vale e a esfera antes que fosse publicado",
        ],
        motive: 3,
        anomalies: &[
            "Nenhuma — é um crime de Estado, mais nada",
            "O cartaz foi impresso em 1971",
            "O cartaz de 2025 chegou por fax a uma máquina sem papel, às 3:17: alguém do futuro está procurando por você — e a Rede é feita das pessoas que você salvou em todas as eras",
            "Kevin era um clone de Elias",
        ],
        anomaly: 2,
        threads: &[(1, 0), (0, 2), (0, 4), (1, 3), (5, 0), (4, 2)],
        story: "O Projeto Vermilion nunca acabou; só mudou de nome. Quando Kevin Arnaud recebeu o fax impossível e começou a juntar as famílias da Rede num fórum, a divisão mandou Russell Crane. Ele rastreou os cartazes até a copiadora, esperou Kevin na frente do prédio, subiu às três da manhã, estrangulou o rapaz com o cabo do telefone, pendurou o corpo e apagou o HD com um ímã do governo. Depois, do quarto do hotel, entrou no fórum e apagou quatrocentas mensagens.",
        sphere: "Você desaparece em 2025. É o que o cartaz diz, e o cartaz não mente: você tocou a esfera e sumiu. Alguém do outro lado — alguém que te ama — mandou o fax. E as pessoas que você salvou ao longo de oitenta anos formaram, sem saber, uma rede para te procurar. O fio vermelho é feito delas.",
        on_true: outcome(
            "A polícia de Nova Orleans prendeu Russell Crane antes que Washington pudesse ligar. Uma semana depois, a cela estava vazia e ninguém tinha registro da transferência. Mas o backup do fórum estava no servidor da UNO. Tasha e Henri o puseram de volta no ar. A Rede tem, agora, dois mil membros.",
            "'SUICÍDIO' DE JOVEM HACKER ERA ASSASSINATO, DIZ POLÍCIA",
            "...o Departamento de Justiça não comenta a prisão de um homem que diz ser agente federal...",
            vec![(1, "jailed"), (2, "grateful"), (4, "grateful"), (3, "survived"), (5, "grateful")],
        )
        .flags(vec!["network_saved"]),
        on_false: outcome(
            "O caso foi arquivado como suicídio, e depois reaberto contra o acusado errado. O fórum Fio Vermelho saiu do ar. Os cartazes foram arrancados dos postes em uma noite. Em alguns, alguém escreveu com caneta vermelha: 'ainda estamos procurando'.",
            "JOVEM HACKER SE MATA APÓS ESPALHAR CARTAZES BIZARROS",
            "...a prefeitura multa quem colar cartazes nos postes do French Quarter...",
            vec![(1, "criminal"), (2, "hates_elias"), (4, "left_city")],
        )
        .flags(vec!["network_lost"]),
        digit: "7",
        reward: 115,
    }
}

pub fn case30() -> CaseDef {
    CaseDef {
        id: 30,
        title: "O Último Caso",
        city: CityId::NewOrleans,
        year: 2001,
        intro: "Nova Orleans, 2001. Às 3:17 desta madrugada, um rapaz de vinte e quatro anos foi morto no centro exato do terreno baldio — no ponto para onde toda bússola aponta. Ele tinha um quadro de cortiça com trinta casos ligados por fio vermelho, os seus olhos e a sua letra. Ao lado do corpo, um machado que você já viu antes. E há um velho de sobretudo que não se esconde mais. Ele está esperando você.",
        brief: "Eli Valcourt, 24 anos, pesquisador obcecado por crimes sem solução, foi morto a machadadas no terreno onde o Laboratório Delta será construído. A arma é o machado de 1920.",
        start: B(BKind::Abandoned, 0),
        cast: vec![
            person("Eli", "Valcourt", false, 24, Job::Student, Role::Victim, B(BKind::Apartment, 12), "Estudante de história, insone, obcecado por crimes sem solução. Tem os seus olhos, a sua letra, a sua mania de sublinhar com seta. Um dia ele encontraria a esfera.").dead(),
            person("Elias", "Vale", false, 70, Job::Scientist, Role::Suspect, B(BKind::House, 14), "O Outro Elias. Sobretudo gasto, cabelo branco, as suas mãos com cicatrizes que você ainda não tem. Esteve em trinta cenas de crime. Não foge de você — ainda.")
                .temper(10, 90, 30, 10)
                .flees()
                .topics(vec![
                    topic("quem", "Quem é você?", "Você, depois de tudo. Depois do limbo, das portas, das cadeiras, das cartas da Clara. Eu resolvi os trinta casos também, Elias. E descobri o que você vai descobrir hoje: eles precisavam acontecer."),
                    topic("noite", "Onde você estava às 3:17?", "No terreno, claro. Eu sempre estou lá às 3:17. Na Rua Royal em 1920, no trem em 1924, no hotel em 1928, na cisterna em junho. Onde a esfera toca, eu estou. Hoje ela tocou no rapaz.")
                        .lie("Dormindo. Velhos dormem cedo, Elias. Você vai descobrir.", 9),
                    topic("machado", "O machado de 1920 estava ao lado do corpo.", "É o mesmo machado. Ele sempre volta pra mim. Eu tirei do depósito de provas com o nome do Walter Hale, na minha letra — na sua letra. Achei que você ia gostar da rima.")
                        .lie("Nunca toquei nesse machado. Qualquer um podia ter tirado do depósito.", 1),
                    topic("caderno", "O que é este caderno com trinta datas?", "Trinta crimes às 3:17. Eu não matei todos. Eu empurrei. Uma carta no lugar certo, um vinte dólares pra um trompetista, um bilhete num para-brisa. A esfera come o que a gente dá pra ela. E sem ela, você nunca teria existido pra me tornar.").req(Req::Clue(4)),
                    topic("porque", "Por que Eli? Ele era só um garoto.", "Porque ele ia encontrar a esfera antes de você. Uma versão nova, sem cicatriz nenhuma. Se ele tocasse nela, a linha se partia e você — eu — nunca teria acontecido. Só pode haver um de nós, Elias. Eu escolhi o que já sabe o final.").req(Req::Topic("caderno")),
                ]),
            person("Yvonne", "Valcourt", true, 50, Job::Teacher, Role::Witness, B(BKind::Apartment, 12), "Mãe de Eli. Professora primária. Ainda está com o avental da escola.")
                .temper(65, 50, 85, 15)
                .topics(vec![
                    topic("filho", "O que Eli fez na última noite?", "Não dormiu, como sempre. Às duas pegou a jaqueta e disse que ia 'ver onde tudo começa'. Disse que um velho o seguia fazia semanas, um velho com a cara dele. Eu ri. Eu ri do meu filho.").reveals(&[8]),
                    topic("velho", "A senhora viu esse velho?", "Uma vez. Ele trouxe flores. Ficou parado na porta, olhando o Eli dormir no sofá. Chorou. Disse: 'desculpe, eu não tenho outro jeito'. Achei que era um vizinho senil."),
                ]),
            person("Raymond", "Hale", false, 46, Job::Detective, Role::Contact, B(BKind::House, 11), "O detetive Hale, do caso da Rua Royal. Agora trata você como parceiro — e isso o assusta.")
                .temper(35, 75, 75, 20)
                .topics(vec![
                    topic("machado", "Como o machado saiu do depósito de provas?", "Alguém assinou a retirada ontem à noite com o nome 'W. Hale'. Meu bisavô. A letra... Elias, olha isso. A letra é a sua. Eu comparei com os seus relatórios.").reveals(&[11]),
                    topic("fim", "Você acredita em mim?", "Eu passei a vida achando que minha família era amaldiçoada. Agora eu vejo você e ele, lado a lado, e entendo: não é maldição. É um círculo. Alguém tem que quebrar."),
                ]),
            person("Earl", "Boudreaux", false, 61, Job::Drifter, Role::Witness, B(BKind::House, 12), "O veterano que dorme ao lado do terreno. Viu tudo. De novo.")
                .temper(55, 50, 65, 25)
                .topics(vec![
                    topic("noite", "O que o senhor viu esta madrugada?", "O velho do sobretudo e o rapaz, no meio do terreno. O velho tinha um machado embrulhado em jornal. Conversaram baixinho, parecia pai e filho. Aí o céu ficou vermelho às três e dezessete. Quando voltou a ficar escuro, só o velho estava de pé.").reveals(&[9]),
                ]),
            person("Anselm", "Roy", false, 66, Job::Priest, Role::Witness, B(BKind::House, 1), "Padre da igreja de St. Augustine. Ouviu uma confissão que não consegue esquecer.")
                .temper(40, 60, 90, 5)
                .topics(vec![
                    topic("velho", "Um velho de sobretudo veio se confessar?", "O sigilo da confissão é sagrado, meu filho. Mas... ele sabia que você viria. Disse que você teria o rosto dele e as perguntas dele."),
                    topic("confissao", "Padre, o que ele disse?", "Não vou repetir os pecados. Só a frase que ele disse ao sair, fora do confessionário: 'Eu matei trinta vezes para salvar uma. Amanhã é a última.' Deus me perdoe, eu achei que era metáfora.").req(Req::Topic("velho")).reveals(&[10]),
                ]),
            person("Wade", "Lanier", false, 27, Job::Musician, Role::Suspect, B(BKind::Apartment, 6), "Colega de apartamento de Eli até o mês passado. Baixista de uma banda de sludge. Estava cobrando uma dívida.")
                .temper(45, 60, 45, 60)
                .topics(vec![
                    topic("divida", "Eli te devia dinheiro?", "Devia dois mil. E pagou ontem! Em notas de cem novinhas, mas da série de 1969. Disse que um velho deu o dinheiro pra ele sumir da cidade. Eli riu e disse que não ia a lugar nenhum — que 'estava perto'.")
                        .lie("Não me devia nada. A gente era parceiro.", 7)
                        .reveals(&[12]),
                    topic("noite", "Onde você estava às três?", "Tocando no bar da Frenchmen até as quatro. Oitenta bêbados me viram errar o mesmo riff a noite inteira."),
                ]),
        ],
        clues: vec![
            clue("Corpo no centro do terreno", ClueKind::Physical, Some(B(BKind::Abandoned, 0)), Look3d::Blood, "Eli no centro exato do terreno, onde a bússola apontava. O relógio de pulso parou às 3:17. Os olhos abertos parecem os seus no espelho.", &[])
                .forensic("Um único golpe, de uma lâmina forjada à mão. No ferimento, ferrugem de oitenta anos."),
            clue("O machado de 1920", ClueKind::Physical, Some(B(BKind::Abandoned, 0)), Look3d::Weapon, "O machado da cozinha dos Cavaretta, o mesmo da parede dos Moreau. A etiqueta de evidência da polícia foi cortada.", &[1])
                .forensic("As digitais no cabo são idênticas às suas. Idênticas — mas com as linhas cortadas por cicatrizes que você ainda não tem.")
                .hidden(1),
            clue("Quadro de cortiça de Eli", ClueKind::Document, Some(HomeOf(0)), Look3d::Glow, "Trinta casos, de 1920 a 2001, ligados por fio vermelho. A letra é quase a sua. No centro, circulado três vezes: 'LABORATÓRIO DELTA — 2025?'", &[]),
            clue("Diário de Eli", ClueKind::Document, Some(HomeOf(0)), Look3d::Paper, "'Um velho me segue. Tem o meu rosto. Ontem ele disse: você não vai encontrar a esfera. Ele vai.' Na margem, uma seta de sublinhar — igual à sua.", &[1]),
            clue("Caderno do Outro Elias", ClueKind::Document, Some(HomeOf(1)), Look3d::Paper, "Datas: 1920, 1922, 1924... 2001. Ao lado de cada uma, um nome e '3:17'. Riscados, um a um. O último: 'Eli Valcourt — o último. Depois dele, só ele.'", &[1])
                .forensic("A tinta das primeiras páginas é mais nova que a das últimas. O caderno foi escrito de trás para a frente no tempo.")
                .hidden(2),
            clue("Plantas originais do Delta", ClueKind::Document, Some(HomeOf(1)), Look3d::Glow, "As mesmas plantas da picape do agrimensor — o original, com correções em tinta vermelha: 'a esfera fica AQUI. Porta blindada. Ele tem que achar sozinho.'", &[1]),
            clue("Sobretudo manchado", ClueKind::Physical, Some(HomeOf(1)), Look3d::Object, "O sobretudo que você viu em trinta cenas de crime, pendurado atrás da porta. Na barra, lama do terreno e sangue fresco.", &[1])
                .forensic("O sangue é de Eli. A lama tem o mesmo pó vitrificado do centro do terreno.")
                .hidden(1),
            clue("Nota promissória de Wade", ClueKind::Document, Some(HomeOf(6)), Look3d::Paper, "'Eli Valcourt deve $2.000 a Wade Lanier.' Embaixo, a caneta: 'Paga até sexta ou eu quebro a tua cara.'", &[6])
                .forensic("Carimbada 'PAGO' ontem. E o livro de ponto do bar da Frenchmen mostra Wade no palco até as 4h.")
                .herring(),
            testimony("Ver onde tudo começa", "Yvonne: às 2h, Eli saiu para 'ver onde tudo começa'. Um velho com a cara dele o seguia havia semanas.", &[1]),
            testimony("O céu vermelho", "Earl: às 3:17 o velho do sobretudo e o rapaz estavam no centro do terreno; o velho tinha um machado embrulhado em jornal. O céu ficou vermelho. Só o velho ficou de pé.", &[1]),
            testimony("Trinta vezes para salvar uma", "Padre Anselm: o velho disse 'Eu matei trinta vezes para salvar uma. Amanhã é a última.'", &[1]),
            testimony("A assinatura de W. Hale", "Raymond: o machado foi retirado do depósito de provas com a assinatura 'W. Hale' — na sua caligrafia.", &[1]),
            testimony("Notas de 1969", "Wade: Eli pagou a dívida com notas novas da série de 1969, dadas por um velho para ele sumir da cidade.", &[1]),
            clue("Eco: o centro do terreno", ClueKind::Temporal, Some(B(BKind::Abandoned, 0)), Look3d::None, "No eco, dois homens com o mesmo rosto, cinquenta anos de diferença. 'Desculpe. Só pode haver um de nós.' O machado sobe. O céu fica vermelho às 3:17.", &[1]),
            clue("Eco: os trinta túmulos", ClueKind::Temporal, Some(Cemetery), Look3d::None, "No eco, um velho deixa flores em trinta túmulos. No último, sem nome e ainda aberto, ele escreve com o dedo na terra: 'E.V.'", &[1]),
            clue("Círculo vitrificado", ClueKind::Physical, Some(B(BKind::Abandoned, 0)), Look3d::Glow, "No fundo da cisterna, um círculo perfeito de terra vitrificada, roxa e vermelha, que pulsa no ritmo do seu coração. A esfera ainda não existe aqui. Mas já deixou a marca.", &[])
                .hidden(3),
        ],
        echoes: vec![
            echo(B(BKind::Abandoned, 0), Ghost::Struggle, &["O mato alto se curva sem vento.", "Um rapaz e um velho, frente a frente. O mesmo rosto.", "'Desculpe. Só pode haver um de nós.'", "O céu fica vermelho. 3:17."], 13),
            echo(Cemetery, Ghost::Wait, &["Túmulos brancos sob a lua.", "Um velho de sobretudo deixa uma flor em cada um.", "Ele conta baixinho: vinte e oito, vinte e nove, trinta.", "No último, sem nome, ele se ajoelha."], 14),
        ],
        culprit: 1,
        methods: &[
            "Wade o espancou por causa da dívida",
            "O Outro Elias o matou com o machado de 1920, no centro do terreno, às 3:17",
            "Suicídio ritual diante do quadro de cortiça",
            "Raymond Hale repetiu o crime do bisavô",
        ],
        method: 1,
        motives: &[
            "Uma dívida de dois mil dólares",
            "Loucura senil de um velho sem nome",
            "Garantir que só uma versão de Elias encontre a esfera — e alimentá-la com a última morte das 3:17 para fechar o círculo",
            "Vingança pelo pai",
        ],
        motive: 2,
        anomalies: &[
            "Nenhuma — é apenas um crime",
            "Eli era um clone criado pelo Projeto Vermilion",
            "O Outro Elias é Walter Hale reencarnado",
            "A esfera é um arquivo vivo: cada morte às 3:17 a alimentou, e o Outro Elias é você no fim do círculo — você é a causa do próprio mistério",
        ],
        anomaly: 3,
        threads: &[(1, 0), (0, 2), (1, 4), (1, 5), (1, 3), (0, 6), (1, 2)],
        story: "O Outro Elias é você, depois de trinta casos e trinta anos. Ele descobriu que a esfera se alimenta das mortes às 3:17 e, para garantir a própria existência, passou décadas empurrando crimes para essa hora: uma carta, um bilhete, um trompetista pago, um machado no lugar certo. Eli Valcourt era uma versão nova de você, prestes a encontrar a esfera primeiro. O velho tentou comprá-lo com notas de 1969; Eli recusou. Então ele tirou o machado de 1920 do depósito de provas com o nome de Walter Hale, levou o rapaz ao centro do terreno e o matou às 3:17 — a última morte, a que fecha o círculo.",
        sphere: "A esfera é um arquivo vivo de possibilidades, e cada crime às 3:17 foi uma página escrita nele. Você perseguiu o mistério por trinta casos só para descobrir que o homem do sobretudo, as datas riscadas, as plantas na sua letra e o terreno sem dono eram você. Você é a causa do próprio mistério. Agora a esfera flutua a um palmo do seu rosto e espera uma última escolha.",
        on_true: outcome(
            "O Outro Elias não resistiu. Estendeu os pulsos para Raymond Hale e sorriu para você. Na cela, às 3:17, a luz ficou vermelha e ele sumiu; sobrou só o sobretudo dobrado no catre. Yvonne enterrou o filho sob um nome que você vai lembrar para sempre. E no centro do terreno baldio, a um palmo do chão, algo começou a brilhar.",
            "IDOSO SEM IDENTIDADE CONFESSA MORTE NO TERRENO BALDIO — E DESAPARECE DA CELA",
            "...às três e dezessete da manhã, moradores do bairro relatam que o céu ficou vermelho...",
            vec![(1, "left_city"), (2, "grateful"), (3, "police"), (4, "survived"), (5, "grateful")],
        )
        .flags(vec!["other_elias_found"]),
        on_false: outcome(
            "O acusado foi condenado. O velho do sobretudo assistiu ao julgamento da última fila e saiu antes da sentença. Na porta do tribunal, ele se virou para você: 'Ainda não, Elias. Mas você vai voltar. Você sempre volta.' No terreno baldio, a um palmo do chão, algo continuou brilhando.",
            "CASO DO TERRENO BALDIO: CONDENADO O ASSASSINO DO ESTUDANTE",
            "...(estática)... Elias... (estática)... o último caso ainda está aberto...",
            vec![(1, "criminal"), (2, "hates_elias")],
        )
        .flags(vec!["other_elias_free"]),
        digit: "0",
        reward: 120,
    }
}
