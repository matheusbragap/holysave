# Arquitetura do Holy Save

## Visao geral

O Holy Save e um utilitario para Windows 11 que detecta jogos instalados, faz backup por jogo e remove saves residuais quando o jogo e desinstalado.

## Regras de negocio

- Visibilidade estrita: so exibe jogos com manifestos ativos no PC.
- Gatilho da faxina: backup final e limpeza de saves residuais ao desinstalar.
- Backup granular e atomico: um arquivo por jogo, sem misturar dados.

## Stack

| Camada | Tecnologia | Motivo |
| --- | --- | --- |
| Backend | Rust | Performance e seguranca de memoria |
| UI | Svelte | Bundle pequeno e rapido |
| Desktop | Tauri | Binarios leves com WebView2 |
| Compressao | Zstandard | Alta velocidade e eficiencia |
| Banco | SQLite | Local, simples e robusto |
| Cloud | Google Drive API | Sync direto via OAuth 2.0 |

## Pipeline

1. Deteccao e mapeamento: le manifestos Steam/Epic e cruza com o manifesto do Ludusavi.
2. Monitoramento: observa alteracoes e espera o save ser liberado.
3. Processamento: compacta em Zstd e gera checksum.
4. Sincronizacao: compara com a nuvem e envia somente se mudou.

## Fontes de dados

- Steam: arquivos .acf e pastas em steamapps.
- Epic: arquivos .item.
- Ludusavi: manifesto YAML com mapeamento de saves.

## Documentos detalhados

- Steam service: ./steam.md

## Documentação do código (arquivos e fluxos)

Referência ficheiro-a-ficheiro, diagramas Mermaid e tabela `invoke`: [../codigo-fonte/README.md](../codigo-fonte/README.md).
