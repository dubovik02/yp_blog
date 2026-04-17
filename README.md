<div align='center'>

# bis_rust

</div>

Константы и ошибки
DOT ENV не загружается для сервера
---

## Технологии

Проект создан на базе:

-   Rust, Cargo

## Реализованный функционал


## Сборка проекта

cargo build

## Запуск проекта

## Примеры команд

# Http

--server http://localhost:8080 register --username Abc --email abc@mail.cd --password qwerty
--server http://localhost:8080 login --email abc2@mail.cd --password qwerty
--server http://localhost:8080 create --title "abc2@mail.cd" --content "qwerty" 
--server http://localhost:8080 get --post-id 31
--server http://localhost:8080 update --post-id 31 --title "new abc2@mail.cd" --content "new qwerty"
--server http://localhost:8080 delete --post-id 31
--server http://localhost:8080 list

# Grpc

--grpc --server http://localhost:50051 register --username Abc2 --email abc2@mail.cd --password qwerty
--grpc --server http://localhost:50051 login --email abc2@mail.cd --password qwerty
--grpc --server http://localhost:50051 create --title "abc2@mail.cd" --content "qwerty" 
--grpc --server http://localhost:50051 get --post-id 31
--grpc --server http://localhost:50051 update --post-id 31 --title "new abc2@mail.cd" --content "new qwerty"
--grpc --server http://localhost:50051 delete --post-id 31
--grpc --server http://localhost:50051 list