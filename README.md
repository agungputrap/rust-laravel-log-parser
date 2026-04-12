# Laravel Log Parser

A fast, lightweight command-line tool written in Rust to parse and filter standard `laravel.log` files. Instead of using `grep` or scrolling through massive files, this tool intelligently groups multi-line error traces and extracts categorized error summaries directly to your terminal.

> **Note:** This is an educational project built specifically to learn and practice implementing Rust concepts, such as file I/O, error handling (`Result`/`Option`), and pattern matching. 

## Features
- **Efficient Parsing**: Uses buffered reading to parse massive log files line-by-line without overloading memory.
- **Multi-line Grouping**: Understands Laravel's log structure and groups multi-line stack traces belonging to a single event.
- **Categorization & Summarization**: Transforms massive JSON stacks / stack traces into readable categories (e.g., `DB_ERROR`, `CONNECTION_REFUSED`, `VIEW_ERROR`).
- **Environment Filtering**: Includes an optional log environment filter (e.g., `local`, `production`) to isolate errors from a specific stage.

## Prerequisites
To run this project, make sure you have Rust and Cargo installed:
* [Install Rust](https://rustup.rs/)

## How to Run

1. Place your `laravel.log` file in the root directory of this project (it is ignored by git).
2. Run the tool through Cargo.

**Parse the whole file:**
```bash
cargo run
```

**Filter by Environment:**
You can pass an argument to filter the logs by environment (like `local` or `prod` or `ERROR` depending on how your logs are structured):
```bash
cargo run -- local
cargo run -- production
```

## Example `laravel.log`

Here's an example of standard, generalized Laravel log text that this tool can successfully parse. Drop this into `laravel.log` to test it out:

```text
[2021-12-06 09:32:59] local.ERROR: SQLSTATE[HY000] [2002] Connection refused (SQL: select * from `users` where `role` is null order by `id` asc) {"exception":"[object] (Illuminate\\Database\\QueryException(code: 2002): SQLSTATE[HY000] [2002] Connection refused"}
[2022-01-04 04:46:52] production.ERROR: Server error: `GET http://10.0.0.100:8000/api/product-search/?order=asc` resulted in a `500 Internal Server Error` response:
<!DOCTYPE HTML PUBLIC "-//IETF//DTD HTML 2.0//EN">
<html><head>
<title>500 Internal Server Error</title>
</head><body>
[2022-05-13 01:37:26] local.ERROR: cURL error 7: Failed to connect to 10.0.0.100 port 8000: Connection refused (see https://curl.haxx.se/libcurl/c/libcurl-errors.html)
[2022-05-24 08:45:46] development.ERROR: Unable to locate Mix file: /css/app.css. (View: /var/www/html/resources/views/app.blade.php) {"exception":"[object] (Facade\\Ignition\\Exceptions\\ViewException(code: 0): Unable to locate Mix file: /css/app.css."}
```
