# Сервис онлайн-экскурсий "Вузиум"

### Требования

<details>
<summary>Установка необходимых зависимостей для компиляции</summary>

<details>
<summary>Для Windows</summary>

##### 1. Загрузить установщик [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/ru/visual-cpp-build-tools/) и открыть его, чтобы начать установку.

##### 2. Установить [Rust](https://www.rust-lang.org/tools/install)

##### 3. Установить Tauri CLI

```
cargo install tauri-cli --version 2.0.0-beta.12 --locked
```

##### 4. Установить WebAssembly target

```
rustup target add wasm32-unknown-unknown
```

##### 5. Установить Trunk

```
cargo install trunk
```

##### 6. Установить [Node.js](https://nodejs.org/en/download/)

##### 7. Установить json-server

```
npm install -g json-server@0.17.4
```

</details>
</details>

### Компиляция

<details>
<summary>Для Windows</summary>

##### 1. в режиме "dev":

##### - Запуск аварийного сервера данных и Docker

```
json-server --watch test/db.json --port 8000


```

###### - Чистый Web

```
trunk serve
```

###### - С обёрткой Tauri

```
cargo tauri dev
```

</details>
