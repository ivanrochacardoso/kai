#  Kai - Tecla AI

> **Expansor de texto com IA** - Digite inteligente, trabalhe mais rápido!

Kai é uma ferramenta leve de expansão de texto que integra IA para ajudá-lo a escrever de forma mais rápida e inteligente. Simplesmente digite `:pergunta sua pergunta/` em qualquer lugar do sistema e obtenha respostas instantâneas com IA.

##  Funcionalidades

-  **Sistema Global**: Funciona em qualquer aplicação (navegadores, editores, chat)
-  **Rápido e Leve**: Uso mínimo de recursos do sistema
-  **Powered by IA**: Usa a API do OpenRouter para respostas inteligentes
-  **Trigger Simples**: Digite `:pergunta sua pergunta/` e receba resultados instantâneos
-  **Privacy-First**: Executa localmente, envia apenas suas consultas para o serviço de IA
-  **Multiplataforma**: Funciona no Linux, Windows e macOS

##  Instalação

### Pré-requisitos

- Rust 1.70+ ([rustup.rs](https://rustup.rs/))
- Chave da API OpenRouter ([openrouter.ai](https://openrouter.ai/))

### Configuração

1. **Configure sua chave da API:**
   
   Abra `src/main.rs` e substitua `"sk-or-v1-xxxxxx"` pela sua chave real do OpenRouter:
   ```rust
   let api_key = "sk-or-v1-sua-chave-aqui";
   ```

2. **Compile e execute:**
   ```bash
   cargo build --release
   cargo run --release
   ```

3. **Instalação global (opcional):**
   ```bash
   cargo install --path .
   ```

## Como Usar

1. **Inicie o Kai** - Execute a aplicação
2. **Digite em qualquer lugar** - Em qualquer aplicação, digite: `:pergunta o que é rust?/`
3. **Receba resultados instantâneos** - O Kai substituirá seu texto pela resposta da IA
4. **Sair** - Pressione `Ctrl+C` no terminal

### Exemplos

```
:pergunta explique computação quântica/
:pergunta escreva uma função python para ordenar uma lista/
:pergunta traduza hello para português/
:pergunta resuma este artigo em 3 pontos/
```

## Configuração

Atualmente, a configuração é feita modificando `src/main.rs`:

- **Chave da API**: Substitua a variável `api_key`
- **Modelo**: Altere o campo `model` no payload da API (padrão: `openai/gpt-3.5-turbo`)
- **Trigger**: Modifique o padrão do trigger (padrão: `:pergunta ... /`)
- **Max Tokens**: Ajuste `max_tokens` para respostas mais longas/curtas

### Modelos Disponíveis

O OpenRouter oferece diversos modelos. Alguns exemplos:

```rust
"model": "openai/gpt-3.5-turbo",        // Rápido e econômico
"model": "openai/gpt-4",                // Mais inteligente
```

## Arquitetura

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Listener      │    │   Buffer &       │    │   API IA        │
│   Teclado       ├────┤   Pattern        ├────┤   (OpenRouter)  │
│   (rdev)        │    │   Matching       │    │                 │
└─────────────────┘    └──────────────────┘    └─────────────────┘
         │                        │                       │
         │                        │                       │
         ▼                        ▼                       ▼
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Simulação     │    │   Processamento  │    │   Runtime       │
│   de Texto      │    │   de Resposta    │    │   Assíncrono    │
│   (enigo)       │    │                  │    │   (Tokio)       │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

## Casos de Uso

### Para Desenvolvedores
```
:pergunta como corrigir este erro em rust/
:pergunta otimize este código sql/
:pergunta explique este padrão de design/
```

### Para Escritores
```
:pergunta melhore este parágrafo/
:pergunta sinônimos para "importante"/
:pergunta resuma este texto/
```

### Para Estudantes
```
:pergunta explique derivadas de forma simples/
:pergunta principais causas da segunda guerra/
:pergunta fórmula da área do círculo/
```

### Para Produtividade
```
:pergunta escreva um email profissional sobre/
:pergunta crie uma lista de tarefas para/
:pergunta horário ideal para reunião/
```

## Considerações

- **Linux**: Pode requerer bibliotecas GTK para funcionamento completo
- **macOS**: Pode solicitar permissões de acessibilidade para captura de teclado
- **Windows**: Alguns antivírus podem alertar devido ao monitoramento de teclado
- **Custo**: Cada consulta consome créditos da sua conta OpenRouter
- **Internet**: Requer conexão ativa para funcionar

## Contribuições

Contribuições são bem-vindas! Formas de ajudar:

-  Reporte bugs e problemas
-  Sugira novas funcionalidades
-  Melhore a documentação
-  Envie pull requests

##  Licença

Este projeto está licenciado sob a Licença MIT - veja o arquivo [LICENSE](LICENSE) para detalhes.

##  Agradecimentos

- Inspirado pelo [Espanso](https://espanso.org/) - o incrível expansor de texto
- Construído com ❤️ e ☕ usando Rust 🦀
- Obrigado ao time do OpenRouter pela excelente API

---

<div align="center">
  <strong>Feito com 🦀 Rust</strong><br>
  ⭐ Dê uma estrela se este projeto foi útil!
</div>