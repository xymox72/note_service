set dotenv-load

# 🚀 Запуск приложения в деве
dev:
    cargo watch -x run

# 🏗 Сборка docker-образа
build:
    docker build -t note-service .

# 🐘 Запуск базы данных
db:
    docker compose up -d

# ⏳ Ждём, пока база станет доступна
wait-db:
    @echo "⏳ Ожидание базы данных..."
    docker exec notes_postgres bash -c "until pg_isready -U postgres -d notes_db; do sleep 1; done"

# 🔁 Миграция базы
migrate:
    just wait-db
    docker cp migrations/init.sql notes_postgres:/init.sql
    docker exec -i notes_postgres psql -U postgres -d notes_db -f /init.sql

# 🛑 Остановка и удаление базы
db-stop:
    docker compose down
# 🛑 Остановка application
app-stop:
    @echo "🛑 Остановка контейнера приложения..."
    docker stop note_service || true
    docker rm -f note_service || true

fmt:
    cargo fmt -- --check

lint:
    cargo clippy -- -D warnings

# 🧼 Очистка сборки Rust
clean:
    cargo clean

# 🔼 Полный запуск приложения
up:
    just build
    just db
    just wait-db
    just migrate
    @echo "✅ Сервис доступен на http://localhost:${PORT:=8080}"

# 🔽 Остановка
down:
    just app-stop
    just db-stop

# 🧪 Тесты
test:
    cargo test

# ❤️ Healthcheck эндпоинт
health:
    curl -s -o /dev/null -w "%{http_code}\n" http://localhost:${PORT:=8080}/health

# 🔥 Полная очистка Docker и проекта
clean-all:
    just clean

    @echo "🧹 Останавливаем все контейнеры..."
    docker stop $(docker ps -aq) || true

    @echo "🗑 Удаляем все контейнеры..."
    docker rm -f $(docker ps -aq) || true

    @echo "🖼 Удаляем все образы..."
    docker rmi -f $(docker images -q) || true

    @echo "📦 Удаляем все тома..."
    docker volume rm $(docker volume ls -q) || true

    @echo "🌐 Удаляем все кастомные сети..."
    docker network rm $(docker network ls -q | grep -v "bridge\|host\|none") || true

    @echo "🧼 Чистим builder-кэш..."
    docker builder prune -a -f

    @echo "✅ Docker окружение очищено."
