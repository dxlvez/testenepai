# RED THREAD — Fio Vermelho

Jogo de investigação noir, 3D visto de cima (estilo Path of Exile), feito para jogar em família.
Elias Vale, um pesquisador obcecado por crimes sem solução, toca uma esfera vermelha e passa a
saltar entre linhas do tempo: 30 casos, 12 cidades, de Nova Orleans em 1920 a Nova Orleans em 2001.

## Como jogar (Windows)

1. Abra `dist/RedThread.exe` (um único arquivo — não precisa instalar nada).
2. Os saves ficam na pasta `arquivos_de_caso`, criada ao lado do `.exe`.
3. As teclas podem ser trocadas em **Esc → Ajustes → Controles**.

## Controles padrão

| Tecla | Ação |
|---|---|
| W A S D | Andar |
| Shift (segurar) | Correr |
| Alt (segurar) | Andar devagar / furtivo |
| Mouse | Mirar (com a arma sacada) — cada arma tem alcance máximo |
| Botão esquerdo | Atirar / atacar |
| E | Interagir, conversar (amizade, romance, roubar, render, amarrar, sequestrar...) |
| F | Soco / ataque corpo a corpo |
| G | Sacar / guardar arma |
| T | Recarregar |
| C | Arrastar / carregar corpo |
| H | Esconder corpo / esconder-se |
| B | Render-se à polícia |
| Q | Eco temporal (ver o passado no local) |
| R | Visão Vermelha (os fios entre pessoas e pistas) |
| Tab | Quadro do caso (ligar pistas, acusar) |
| J | Diário (jornais, memória, linhas do tempo, casos, fotos) |
| M | Mapa |
| I | Bolsa (inventário) |
| N | Rede de investigadores e relações |
| P / V | Erguer a câmera / tirar foto |
| Z / X | Girar a câmera |
| Roda do mouse | Zoom |
| Esc | Salvar, carregar, ajustes |

## Dicas

- Cada caso tem **3 camadas**: o culpado, o como/porquê e a esfera. Acusar errado também encerra o caso —
  com consequências na linha do tempo.
- Durma no esconderijo (a cama) para salvar e para voltar a ouvir a esfera quando decidir ficar numa época.
- Pessoas lembram do que viram. Testemunhas denunciam, a polícia segue rastros, e dá para limpar a cena do crime.
- Os números escritos no verso de cada arquivo de caso significam alguma coisa.

## Compilar

```
cargo build --release                                   # Linux
cargo build --release --target x86_64-pc-windows-gnu    # .exe para Windows (mingw-w64)
```

Ferramentas de desenvolvimento: `--check-cases` (valida os 30 casos), `--dump-kinds`, `--dump-map --city nome`,
`--render-audio pasta`.
