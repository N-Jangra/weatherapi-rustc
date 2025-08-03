# 🌤️ Weather CLI Tool (Rust)

A simple, colorful command-line weather forecast application written in Rust. It uses the [WeatherAPI](https://www.weatherapi.com/) to fetch current weather and hourly forecasts for a specified location.

---

##  Features

-  Current weather conditions
-  Hourly forecast up to 3 days
-  Color-coded output for rain chances (red if ≥ 40%)
-  Location-based search
-  Environment-based API key handling
-  Smart date filtering (ignores past hours from today)

---

##  Usage

###  Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- A free API key from [weatherapi.com](https://www.weatherapi.com/)

###  Setup

1. Clone the repository:

```bash
git clone https://github.com/N-Jangra/weatherapi-rustc
cd weatherapi-rustc
````

2. Create a `.env` file in the root directory and add your API key:

```env
WEATHER_API_KEY=your_weatherapi_key_here
```

3. Build the project:

```bash
cargo build
```

4. Run the CLI:

```bash
cargo run -- "Location" [days]
```

* **Location**: Name of the city/town (e.g., `"Mahendragarh"`)
* **Days** (optional): Number of forecast days (1–14, default: 1). The CLI limits output to a maximum of 3 days.

---

###  Example

```bash
cargo run -- "Mahendragarh" 5
```

**Sample Output:**

```text
Mahendragarh, India: 34°C, Partly Cloudy

📅 Forecast for 2025-08-03:

08:30 - 36°C, 0%, Sunny
09:30 - 36°C, 79%, Patchy rain nearby  <-- Red if rain ≥ 40%
...

📅 Forecast for 2025-08-04:
...
```

---

##  Project Structure

```text
├── src
│   ├── main.rs       # CLI logic
│   └── models.rs     # Structs for deserializing Weather API response
├── .env              # Contains WEATHER_API_KEY
├── Cargo.toml        # Rust dependencies
└── README.md         # You're here!
```

---

##  Dependencies

* [`reqwest`](https://docs.rs/reqwest) – for making HTTP requests
* [`serde`](https://serde.rs/) – for JSON parsing
* [`chrono`](https://docs.rs/chrono/) – for handling time and date
* [`dotenvy`](https://crates.io/crates/dotenvy) – for loading `.env` variables
* [`colored`](https://crates.io/crates/colored) – for colorful CLI output

---

##  TODO

* Add unit tests
* Support for weekly summary
* Option to export output to CSV

---

##  License

MIT © 2025 N-Jangra

