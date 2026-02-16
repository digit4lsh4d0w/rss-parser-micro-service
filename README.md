# Парсер RSS фидов

Это учебный проект для парсинга RSS фидов и отправки в очередь RabbitMQ.

[![License: AGPL v3](https://img.shields.io/badge/License-AGPL_v3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)

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

## Лицензия

Этот проект распространяется под лицензией [GNU Affero General Public License v3.0 или более поздней версией](LICENSE).

Это означает, что вы можете свободно использовать, изменять и распространять этот код,
при условии, что производные работы также будут открыты под той же лицензией.
