# Holy Save

Holy Save é um utilitário para Windows 11 que protege o progresso de jogos de forma invisível, automatizada e limpa. Ele detecta jogos instalados, faz backup por jogo e remove saves residuais quando o jogo é desinstalado.

## Visão geral

O objetivo é manter o sistema e a nuvem organizados: apenas jogos presentes aparecem, o backup é feito quando o save é liberado, e cada jogo é tratado de forma isolada.

## Princípios de negócio

- Visibilidade estrita: exibe apenas jogos com manifestos ativos no PC
- Gatilho da faxina: backup final e limpeza de saves residuais ao desinstalar
- Backup granular e atômico: um arquivo por jogo, sem misturar dados

## Recursos

- Detecta jogos via manifestos Steam e Epic
- Backup automático após o jogo fechar
- Compressão Zstd por jogo
- Checksum para integridade
- Fila offline para uploads pendentes
- Sincronização com Google Drive via OAuth 2.0

## Stack

| Camada | Tecnologia | Motivo |
| --- | --- | --- |
| Backend | Rust | Performance e segurança de memória |
| UI | Svelte | Bundle pequeno e rápido |
| Desktop | Tauri | Binários leves com WebView2 |
| Compressão | Zstandard | Alta velocidade e eficiência |
| Banco | SQLite | Local, simples e robusto |
| Cloud | Google Drive API | Sync direto via OAuth 2.0 |

## Fluxo de operação

1. Detecção e mapeamento: lê manifestos e cruza com o manifesto de saves
2. Monitoramento: observa alterações e espera o save ser liberado
3. Processamento: compacta e gera checksum
4. Sincronização: compara com a nuvem e envia somente se mudou

## Requisitos

- Windows 11
- Steam ou Epic instalados
- Conta Google para backup na nuvem

## Desenvolvimento

- Instalar dependências: `npm install`
- Rodar UI: `npm run dev`
- Rodar app desktop: `npm run tauri dev`
- Verificar tipos: `npm run check`

## Licença

MIT
