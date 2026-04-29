<div align='center'>

# bis_blog

</div>


## Технологии

Проект создан на базе:

-   Rust, Cargo, webAssembly, PostgreSql

## Реализованный функционал

веб-сервер с HTTP и gRPC API,
клиентская библиотека,
CLI-клиент и WASM-фронтенд.

## Настройка серверной части

Развернуть и нстроить серев БД PostgreSQL

Добавить .env в корень проекта

Параметры переменных среды: 
DATABASE_URL - строка подключения к серверу базы данных
JWT_SECRET - ключ
SERVER_HOST - адрес сервера http
SERVER_PORT - порт сервера http
GRPC_SERVER_HOST - адрес сервера grpc
GRPC_SERVER_PORT - порт сервера grpc

## Сборка проекта

Запуск из корня проекта:

cargo build --workspace

## Запуск сервера

Из корня проекта

cargo run -p blog-server

## HTTP API

Аутентификация пользователя:
POST /api/v1/auth/register
POST /api/v1/auth/login
GERT /api/v1/protected/me

Работа с блогом:
Получение списка постов (пагинация) - GET /api/v1/posts?limit={limit}&offset={offset}
Получение поста по ID - GET /api/v1/posts/{id}
Создание поста - POST /api/v1/protected/posts
Редактирование поста - PUT /api/v1/protected/posts/{id}
Удаление поста - DELETE /api/v1/protected/posts/{id}


## GRPC API
Схема blog-server/proto/blog.proto/

gRPC интерфейс реализует функционал HTTP API:

регистрация
вход
список постов
получение поста
создание поста
обновление поста
удаление поста

## CLI-клиент

Запуск из корня проекта:
cargo run -p blog-cli < HTTP/CRPC COMMAND >

## Примеры команд CLI

# Http

--server http://localhost:3000 register --username Abc --email abc@mail.cd --password qwerty
--server http://localhost:3000 login --email abc2@mail.cd --password qwerty
--server http://localhost:3000 create --title "abc2@mail.cd" --content "qwerty" 
--server http://localhost:3000 get --post-id 31
--server http://localhost:3000 update --post-id 31 --title "new abc2@mail.cd" --content "new qwerty"
--server http://localhost:3000 delete --post-id 31
--server http://localhost:3000 list

# Grpc

--grpc --server http://localhost:50051 register --username Abc2 --email abc2@mail.cd --password qwerty
--grpc --server http://localhost:50051 login --email abc2@mail.cd --password qwerty
--grpc --server http://localhost:50051 create --title "abc2@mail.cd" --content "qwerty" 
--grpc --server http://localhost:50051 get --post-id 31
--grpc --server http://localhost:50051 update --post-id 31 --title "new abc2@mail.cd" --content "new qwerty"
--grpc --server http://localhost:50051 delete --post-id 31
--grpc --server http://localhost:50051 list

## WASM-клиент

Установить trunk:
cargo install trunk

Запуск из корня /blog-wasm:
trunk serve
Перейти на http://localhost:8080