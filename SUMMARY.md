# Mapping Macro - Краткое описание проекта

## Что создано

Мы создали мощный процедурный макрос на Rust для автоматической генерации реализаций трейтов `Into` и `From` с гибкими возможностями маппинга полей.

## Архитектура

```
mapping-macro-test/
├── mapping-macro/           # Процедурный макрос
│   ├── src/lib.rs          # Основная логика макроса
│   └── Cargo.toml          # Зависимости макроса
├── src/
│   ├── lib.rs              # Тесты функциональности
│   └── main.rs             # Демонстрационные примеры
├── examples/
│   ├── basic_usage.rs      # Базовый пример использования
│   └── advanced_usage.rs   # Продвинутый пример с множественными типами
└── README.md               # Полная документация
```

## Основные возможности

### 1. Автоматическая генерация трейтов
- `#[from(SourceType)]` - генерирует `impl From<SourceType>`
- `#[into(TargetType)]` - генерирует `impl Into<TargetType>`
- `#[try_from(SourceType)]` - генерирует `impl TryFrom<SourceType>` с `anyhow::Result`
- `#[try_into(TargetType)]` - генерирует `impl TryInto<TargetType>` с `anyhow::Result`

### 2. Кастомные выражения для полей
```rust
#[from(CreateUserDto | Uuid::new_v4())]
id: Uuid,
```

### 3. Пропуск полей для специфических типов
```rust
#[into_skip(UserDto)]
internal_field: String,
```

### 4. Множественные источники и цели
```rust
#[derive(Mapping)]
#[from(CreateDto)]
#[from(UpdateDto)]
#[into(ResponseDto)]
#[into(SummaryDto)]
#[try_from(FallibleDto)]
#[try_into(ValidatedDto)]
pub struct Model { ... }
```

### 5. Сложная логика маппинга
```rust
#[from(UpdateDto | value.price.unwrap_or(0.0))]
#[from(CreateDto | value.price)]
#[try_from(FallibleDto | value.price_str.parse()?)]
price: f64,
```

## Пример использования

```rust
use mapping_macro::Mapping;

#[derive(Mapping)]
#[from(CreateUserDto)]
#[into(UserDto)]
pub struct UserModel {
    #[from(CreateUserDto | Uuid::new_v4())]
    id: Uuid,
    name: String,
    email: Option<String>,
    #[from(CreateUserDto | Some(chrono::Utc::now()))]
    #[into_skip(UserDto)]
    created_at: Option<DateTime<Utc>>,
    #[from(CreateUserDto | 1 as i32)]
    version: i32,
}

// Автоматически генерируется:
// impl From<CreateUserDto> for UserModel { ... }
// impl Into<UserDto> for UserModel { ... }
```

## Технические детали

### Реализация макроса
- Использует `syn` для парсинга AST
- Использует `quote` для генерации кода
- Поддерживает сложные выражения в атрибутах
- Обрабатывает множественные источники/цели

### Система атрибутов
- **Структурные**: `#[from(Type)]`, `#[into(Type)]`, `#[try_from(Type)]`, `#[try_into(Type)]`
- **Полевые**: `#[from(Type | expr)]`, `#[into_skip(Type)]`, `#[from_skip(Type)]`
- **Фаллибельные**: `#[try_from(Type | expr)]`, `#[try_into_skip(Type)]`, `#[try_from_skip(Type)]`

### Парсинг выражений
- Парсит выражения после символа `|`
- Поддерживает любые валидные Rust выражения
- Доступна переменная `value` для источника
- Для `try_from` поддерживает выражения с `Result<T, E>` и оператор `?`

## Сценарии использования

1. **Доменное моделирование** - конвертация между DTO, сущностями, моделями
2. **API слой** - преобразование запросов/ответов
3. **Интеграция баз данных** - конвертация между БД моделями и бизнес объектами
4. **Межкрейтовая совместимость** - маппинг типов из разных крейтов
5. **Микросервисная архитектура** - конвертация между структурами сервисов
6. **Валидация данных** - фаллибельные конвертации с проверкой входных данных

## Тестирование

### Покрытие тестами
- ✅ 16 интеграционных тестов в `src/lib.rs` (включая TryFrom/TryInto)
- ✅ 2 юнит-теста вспомогательных функций
- ✅ 4 doctest примера в документации
- ✅ 3 полноценных примера использования

### Тестовые сценарии
- Базовые From/Into преобразования
- Кастомные выражения для полей
- Пропуск полей для разных типов
- Множественные источники и цели
- Обработка Optional значений
- Инкремент версий
- Round-trip конвертация
- TryFrom с успешной конвертацией
- TryFrom с различными видами ошибок (UUID, парсинг, валидация)
- TryInto с пропуском полей
- Фаллибельная round-trip конвертация

## Команды для запуска

```bash
# Запуск всех тестов
cargo test --all

# Базовый пример
cargo run --example basic_usage

# Продвинутый пример
cargo run --example advanced_usage

# TryFrom/TryInto пример
cargo run --example simple_try

# Основное демо
cargo run

# Тесты только макроса
cargo test -p mapping-macro
```

## Результат

Создан полнофункциональный процедурный макрос, который:
- ✅ Автоматически генерирует реализации трейтов Into/From
- ✅ Поддерживает сложные выражения для маппинга полей
- ✅ Обрабатывает множественные источники и цели
- ✅ Позволяет пропускать поля для специфических типов
- ✅ Поддерживает фаллибельные конвертации с TryFrom/TryInto
- ✅ Интегрирован с anyhow для обработки ошибок
- ✅ Имеет полную документацию и примеры
- ✅ Покрыт comprehensive тестами
- ✅ Готов к использованию в production

Макрос решает проблему ручного написания boilerplate кода для конвертации между типами и предоставляет гибкую систему для настройки маппинга полей с поддержкой любых Rust выражений, включая фаллибельные операции с обработкой ошибок.