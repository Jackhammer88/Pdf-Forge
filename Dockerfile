# Используем официальный образ Rust как базовый
FROM rust:latest

# Установим все необходимые системные зависимости
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libpoppler-glib-dev \
    libcairo2-dev \
    libglib2.0-dev \
    && rm -rf /var/lib/apt/lists/*

# Устанавливаем рабочую директорию для нашего проекта
WORKDIR /usr/src/myapp

# Копируем файл Cargo.toml и Cargo.lock в контейнер
COPY Cargo.toml Cargo.lock ./

# Загружаем зависимости проекта (это поможет кешировать слои)
RUN cargo fetch

# Копируем все исходные файлы проекта в контейнер
COPY . .

ENV RUSTFLAGS="-L /usr/lib/x86_64-linux-gnu -lpoppler-glib -lcairo -lglib-2.0 -lgobject-2.0"

# Компилируем проект
RUN cargo build --release

# Указываем команду, которая будет запускаться в контейнере
CMD ["bash"]
