# BestLanguage - Análise de Variância de Performance de APIs

## 📋 Sobre o Projeto

Este projeto foi desenvolvido como parte de um trabalho acadêmico focado em **Análise de Variância (ANOVA)** para avaliar o desempenho de diferentes linguagens de programação na implementação de APIs. O objetivo é estruturar um experimento científico e interpretar os resultados obtidos através de análise estatística.

## 🎯 Objetivo do Trabalho

O trabalho visa avaliar se o aluno consegue:
- Estruturar um experimento científico adequadamente
- Interpretar resultados estatísticos
- Aplicar análise de variância no R
- Compreender a interação entre fatores experimentais

## 🔬 Estrutura do Experimento

### Fatores Experimentais

**Fator 1 (Numérico):** Tamanho de Memória RAM
- 250MB
- 500MB  
- 1GB

**Fator 2 (Não-Numérico):** Linguagens de Programação
- C# (.NET)
- Golang (Fiber)
- Python (Flask)
- JavaScript (Node.js/NestJS)
- PHP (Laravel)
- Ruby (Rails)
- Rust (Actix-web)
- Java (Spring Boot)

**Bloco:** Núcleos de CPU
- 0.5 núcleo
- 1 núcleo
- 2 núcleos

**Variável Resposta:** Tempo de resposta (milissegundos/segundos)

## 🏗️ Arquitetura do Projeto

O projeto consiste em 8 APIs idênticas implementadas em diferentes linguagens de programação, todas realizando a mesma operação: **verificação de números primos**.

### Estrutura das APIs

Cada API possui:
- **Endpoint:** `POST /`
- **Entrada:** `{"number": <número_inteiro>}`
- **Saída:** `{"is_prime": <boolean>}`
- **Funcionalidade:** Algoritmo de verificação de números primos

### Tecnologias Utilizadas

| Linguagem | Framework | Diretório |
|-----------|-----------|-----------|
| C# | ASP.NET Core | `api-csharp/` |
| Go | Fiber | `api-fiber/` |
| Python | Flask | `api-flask/` |
| JavaScript | NestJS | `api-javascript/` |
| PHP | Laravel | `api-laravel/` |
| Ruby | Rails | `api-rails/` |
| Rust | Actix-web | `api-rust/` |
| Java | Spring Boot | `api-spring-boot/` |

## 🚀 Como Executar

### Pré-requisitos
- Docker e Docker Compose
- Node.js (para APIs JavaScript)
- Python 3.x (para API Flask)
- PHP 8.x (para API Laravel)
- Ruby 3.x (para API Rails)
- .NET 6+ (para API C#)
- Go 1.19+ (para API Golang)
- Rust 1.70+ (para API Rust)
- Java 17+ (para API Spring Boot)

### Executando as APIs

Cada API pode ser executada individualmente:

```bash
# C# (.NET)
cd api-csharp
dotnet run

# Golang (Fiber)
cd api-fiber
go run main.go

# Python (Flask)
cd api-flask
python main.py

# JavaScript (NestJS)
cd api-javascript
npm install
npm run start:dev

# PHP (Laravel)
cd api-laravel
composer install
php artisan serve

# Ruby (Rails)
cd api-rails
bundle install
rails server

# Rust (Actix-web)
cd api-rust
cargo run

# Java (Spring Boot)
cd api-spring-boot
./mvnw spring-boot:run
```

### Testando as APIs

Todas as APIs respondem na porta 3000 (padrão) e podem ser testadas com:

```bash
curl -X POST http://localhost:3000/ \
  -H "Content-Type: application/json" \
  -d '{"number": 17}'
```

Resposta esperada:
```json
{"is_prime": true}
```

## 📊 Metodologia de Teste

### Coleta de Dados

1. **Configuração do Ambiente:**
   - Configurar diferentes tamanhos de RAM (250MB, 500MB, 1GB)
   - Configurar diferentes números de núcleos de CPU (0.5, 1, 2)
   - Executar cada API em cada configuração

2. **Execução dos Testes:**
   - Enviar requisições para verificação de números primos
   - Medir tempo de resposta para cada requisição
   - Repetir testes múltiplas vezes para garantir confiabilidade

3. **Análise Estatística:**
   - Aplicar ANOVA no R
   - Verificar significância dos fatores
   - Analisar interações entre fatores
   - Gerar gráficos de análise

### Exemplo de Dados Coletados

| RAM | Linguagem | CPU | Tempo_Resposta |
|-----|-----------|-----|----------------|
| 250MB | C# | 0.5 | 15.2ms |
| 250MB | C# | 1 | 12.1ms |
| 250MB | C# | 2 | 8.9ms |
| 250MB | Go | 0.5 | 8.7ms |
| ... | ... | ... | ... |

## 📈 Análise de Variância

### Hipóteses Testadas

1. **H0:** Não há diferença significativa entre os tamanhos de RAM
2. **H0:** Não há diferença significativa entre as linguagens de programação  
3. **H0:** Não há diferença significativa entre os números de núcleos de CPU
4. **H0:** Não há interação significativa entre os fatores

### Interpretação dos Resultados

- **Valor p < 0.05:** Rejeita H0 (diferença significativa)
- **Valor p ≥ 0.05:** Não rejeita H0 (sem diferença significativa)
- **Interação significativa:** O efeito de um fator depende do nível do outro fator

## 🎓 Aspectos Acadêmicos

### Requisitos do Trabalho

- ✅ Estruturação adequada dos dados experimentais
- ✅ Aplicação de análise de variância no R
- ✅ Interpretação completa dos testes estatísticos
- ✅ Demonstração ao vivo dos comandos R
- ✅ Apresentação de gráficos de análise
- ✅ Manipulação de dados para forçar interação significativa (quando necessário)
- ✅ Comparação entre resultados reais e "forçados"

### Conceitos Fundamentais

- **Fator:** Variável independente que pode influenciar a variável resposta
- **Bloco:** Fonte de variação que pode interferir nos resultados
- **Variável Resposta:** Métrica medida para avaliar o efeito dos fatores
- **Interação:** Quando o efeito de um fator depende do nível de outro fator

## 📁 Estrutura do Repositório

```
BestLanguage/
├── api-csharp/          # API em C# (.NET)
├── api-fiber/           # API em Go (Fiber)
├── api-flask/           # API em Python (Flask)
├── api-javascript/      # API em JavaScript (NestJS)
├── api-laravel/         # API em PHP (Laravel)
├── api-rails/           # API em Ruby (Rails)
├── api-rust/            # API em Rust (Actix-web)
├── api-spring-boot/     # API em Java (Spring Boot)
└── README.md            # Este arquivo
```

## 🤝 Contribuição

Este é um projeto acadêmico desenvolvido para fins de estudo e análise estatística. Contribuições são bem-vindas para melhorar a implementação das APIs ou a metodologia de teste.

## 📝 Licença

Este projeto é destinado exclusivamente para fins acadêmicos e educacionais.

---

**Nota:** Este projeto foi desenvolvido como parte de um trabalho acadêmico focado em análise de variância e comparação de performance entre diferentes linguagens de programação.
