![TypeScript](https://img.shields.io/badge/TypeScript-007ACC?style=for-the-badge&logo=typescript&logoColor=white)
![RabbitMQ](https://img.shields.io/badge/rabbitmq-%23FF6600.svg?&style=for-the-badge&logo=rabbitmq&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![AWS Timestream](https://img.shields.io/badge/AWS-%23FF9900.svg?style=for-the-badge&logo=amazon-aws&logoColor=white)

# Projeto de Estágio na Hedro Sistemas Inteligentes

Este projeto faz parte do programa de estágio da Hedro Sistemas Inteligentes de 2024.1. O objetivo deste projeto é fornecer uma base para os estagiários entenderem e trabalharem com as tecnologias utilizadas pela Hedro ao longo do estágio. Algumas das tecnologias incluídas neste projeto são TypeScript, Protocolo MQTT da EMQX, RabbitMQ, Rust e AWS Timestream.

## Tutorial para usar a aplicação

### Pré-requisitos
- Docker instalado em seu sistema: [Docker Installation Guide](https://docs.docker.com/get-docker/)

### Passo 1: Clonar o repositório
```bash
git clone https://github.com/PedroLouback/Hedro-Training-2024.1
cd Hedro-Training-2024.1
```

### Passo 2: Construir os contêineres Docker
Primeiro, vamos construir os contêineres para cada parte da aplicação.

#### Construir contêiner para o simulador de dispositivo
```bash
cd device-simulator/
docker build . -t device-simulator
cd ..
```

#### Construir contêiner para a aplicação ponte do RabbitMQ e EMQX
```bash
cd rmq-bridge/
docker build . -t rmq-bridge
cd ..
```

#### Construir contêiner para o consumer do RabbitMQ
```bash
cd rmq-consumer
docker build . -t rmq-consumer
cd ..
```

## Tutorial para usar a aplicação

### Passo 3: Executar a aplicação
Depois de construir os contêineres, podemos iniciar a aplicação usando o Docker Compose.

```bash
docker-compose up -d
```

### Passo 4: Verificar o status da aplicação (opcional)
Se desejar verificar se os contêineres estão em execução e ver os logs, você pode usar o comando abaixo.

```bash
docker-compose ps
```
Este comando listará todos os serviços em execução e seus status.

### Passo 5: Parar a aplicação (opcional)
Para parar todos os serviços da aplicação, você pode usar o comando:

```bash
docker-compose down
```
Isso encerrará todos os contêineres e os removerá.

Agora você configurou e executou sua aplicação com sucesso!




