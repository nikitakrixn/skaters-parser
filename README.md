
# Описание

Этот репозиторий содержит код на языке программирования Rust для парсинга информации о фигуристах с веб-сайта allskaters.info и записи ее в CSV файл. Проект создан в рамках заказа на сайте фриланса [freelance.habr.com](https://freelance.habr.com/tasks/573080/)

## Как использовать

1. Установите Rust, если он еще не установлен, следуя инструкциям на [официальном сайте](https://www.rust-lang.org/tools/install).
    
2. Клонируйте репозиторий на свой компьютер:
```bash
git clone https://github.com/nikitakrixn/skaters-parser.git
```
3. Перейдите в каталог проекта:

```bash
cd skaters-parser
```
4. Запустите приложение, выполнив следующие команды:

```bash
cargo build --release
cargo run --release
```
Это выполнит сборку проекта и запустит парсинг информации о фигуристах с сайта allskaters.info. Результаты будут сохранены в файле skaters.csv.

## Зависимости
Этот проект использует следующие зависимости:

csv - для работы с CSV файлами.
scraper - для парсинга HTML страниц.
undetected_chromedriver - для управления браузером при парсинге.


## License

[MIT](https://choosealicense.com/licenses/mit/)

