# Projeto Actix Web com Server-Sent Events (SSE)

Este projeto implementa um servidor web simples usando Actix, que envia eventos de servidor (Server-Sent Events - SSE) para o cliente. O servidor escuta na porta 8081 e envia uma mensagem a cada 2 segundos.

## Estrutura do Projeto

O projeto consiste em um único arquivo principal que define um endpoint para SSE. Quando um cliente acessa o endpoint, ele recebe um fluxo contínuo de mensagens.

### Dependências

- `actix-web`: Para a construção do servidor web.
- `futures-util`: Para trabalhar com streams assíncronas.
- `tokio`: Para gerenciar o tempo e a execução assíncrona.
- `bytes`: Para manipulação de dados em bytes.
- `serde`: Para deserialização de dados.

### Funcionalidades

- **Endpoint `/event/{message}`**: Recebe um parâmetro `message` na URL e começa a enviar eventos que incluem essa mensagem a cada 2 segundos.

  Exemplo de uso: `GET /event/hello`

### Como Usar

1. **Instale as dependências**: Certifique-se de que você tenha o Rust e o Cargo instalados. Depois, adicione as dependências necessárias no seu `Cargo.toml`.

2. **Compile e Execute**: Execute o servidor com o comando:
   ```bash
   cargo run
   ```

3. **Acesse o Endpoint**: Abra um navegador ou uma ferramenta de teste de API e acesse o endpoint:
   ```
   http://127.0.0.1:8081/event/mensagem
   ```

4. **Visualize os Eventos**: O navegador exibirá as mensagens a cada 2 segundos.

### Exemplo de Resposta

O servidor enviará dados no seguinte formato:
```
data: {"id":-1,"timestamp":1730830535,"message":"mensagem"}
```

### Observações

- O fluxo de eventos continua até que a conexão seja encerrada pelo cliente.
- Este exemplo é uma implementação básica e pode ser expandido com tratamento de erros e funcionalidades adicionais conforme necessário.

## Docs

_[Async article](https://aarambhdevhub.medium.com/mastering-async-programming-in-rust-with-tokio-full-tutorial-real-world-examples-635c6e5dcdcc)_