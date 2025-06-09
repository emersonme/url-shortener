# 🔗 Encurtador de URLs em Rust

Um encurtador de URLs simples e eficiente desenvolvido em Rust com Axum, PostgreSQL e Redis.

## 🚀 Funcionalidades

- **POST /shorten**: Encurta uma URL longa
- **GET /:short_code**: Redireciona para a URL original (307 Temporary Redirect)
- **GET /:short_code/stats**: Retorna estatísticas da URL encurtada

## 🛠️ Tecnologias

- **Rust** - Linguagem de programação
- **Axum** - Framework web assíncrono
- **PostgreSQL** - Banco de dados principal
- **Redis** - Cache para melhor performance
- **SQLx** - Driver de banco de dados
- **Tokio** - Runtime assíncrono

## 📋 Pré-requisitos

- Rust 1.70+
- PostgreSQL
- Redis

## 🔧 Configuração

1. **Clone o repositório e instale as dependências:**
```bash
cargo build
```

2. **Execute o docker compose:**
```bash
docker compose up -d
```

## 🏃‍♂️ Executando

```bash
cargo run
```

O servidor estará rodando em `http://localhost:3000`

## 📡 API Endpoints

### Encurtar URL
```bash
POST /shorten
Content-Type: application/json

{
  "url": "https://www.example.com/very/long/url"
}
```

**Resposta:**
```json
{
  "short_code": "abc123",
  "short_url": "http://localhost:3000/abc123",
  "original_url": "https://www.example.com/very/long/url"
}
```

### Redirecionar
```bash
GET /:short_code
```

Retorna um redirecionamento 307 para a URL original.

### Estatísticas
```bash
GET /:short_code/stats
```

**Resposta:**
```json
{
  "short_code": "abc123",
  "original_url": "https://www.example.com/very/long/url",
  "clicks": 42,
  "created_at": "2024-01-01T12:00:00Z"
}
```

## 🔍 Exemplos de Uso

### Encurtar uma URL:
```bash
curl -X POST http://localhost:3000/shorten \
  -H "Content-Type: application/json" \
  -d '{"url": "https://www.rust-lang.org"}'
```

### Acessar estatísticas:
```bash
curl http://localhost:3000/abc123/stats
```

## 🏗️ Arquitetura

- **Cache Redis**: URLs são armazenadas no Redis para acesso rápido
- **PostgreSQL**: Armazenamento persistente com contagem de clicks
- **Hash Base62**: Códigos curtos de 6 caracteres
- **Async**: Operações assíncronas para melhor performance

## 🧪 Testes

```bash
cargo test
```

## 📝 Variáveis de Ambiente

| Variável | Descrição | Padrão |
|----------|-----------|---------|
| `DATABASE_URL` | URL de conexão do PostgreSQL | `postgres://localhost/url_shortener` |
| `REDIS_URL` | URL de conexão do Redis | `redis://localhost` |
| `SERVER_ADDRESS` | Endereço do servidor | `0.0.0.0:3000` |

## 📄 Licença

Este projeto está sob a licença MIT. 