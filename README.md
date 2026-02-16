# Парсер RSS фидов

Это учебный проект для парсинга RSS фидов и отправки в очередь RabbitMQ.

![Архитектура](assets/microservices.webp)

## Docker Compose Development Setup

1. Перейти в директорию с Docker Compose файлом:

```bash
cd deploy/docker
```

2. Поднять инфраструктуру:

```bash
docker compose up -d
```

## Зеркалирование

Разработка ведется на локальном Git-сервисе автора (Forgejo).

Настроено автоматическое зеркалирование на внешние Git-сервисы:

- [GitVerse](https://gitverse.ru/digit4lsh4d0w/rss-parser-micro-service).
- [GitHub](https://github.com/digit4lsh4d0w/rss-parser-micro-service).

### Баги

- LFS-ресурсы (медиа) не зеркалируются.

## AI

Код в этом репозитории **не генерируется** с помощью AI.
Это учебный репозиторий, где AI помогает с концепциями, архитектурой, подходами и
решением ошибок **только в режиме чата** (_ask_ / _plan_).
