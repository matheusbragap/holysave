# Modelo Game

Este documento descreve o modelo basico de jogo usado no backend e exposto ao frontend.

## Estrutura

- id: identificador do launcher (app id).
- name: nome exibido do jogo.
- platform: origem do jogo (Steam, Epic, etc.).
- install_dir: diretorio de instalacao do jogo.
- save_path: caminho local do save, se encontrado.
- status: estado de sincronizacao do jogo.
- size_bytes: tamanho total do save em bytes.
- checksum: hash do pacote de backup, se ja calculado.
- is_ignored: marca se o jogo deve ser ignorado pelo sistema.

## Option no Rust

Campos do tipo Option podem ser Some(valor) ou None. Ex.: save_path pode ser None quando o jogo nao tem save detectado.

## Platform

- Steam
- Gog
- Epic
- Manual

## GameStatus

- Synced: tudo em dia.
- Pending: mudanca local detectada, precisa de backup.
- Uploading: upload em progresso.
- Error: erro de sincronizacao com descricao.
