# Weather CLI & Web App

A weather application that provides both CLI and web interfaces, built with Rust using Tera templates for the frontend.

## Features

- **Current Weather**: Get real-time weather conditions
- **Hourly Forecasts**: Detailed hourly predictions
- **Multi-day Forecasts**: Up to 14 days of weather data
- **Rain Alerts**: Visual indicators for high rain probability (≥40%)
- **Responsive Design**: Mobile-friendly web interface
- **CLI Support**: Original command-line interface maintained

## Prerequisites

1. **Weather API Key**: Get a free API key from [WeatherAPI.com](https://www.weatherapi.com/)
2. **Rust**: Install Rust from [rustup.rs](https://rustup.rs/)

## Setup

1. **Clone or create the project structure**:
```bash
mkdir weather_app
cd weather_app
```

2. **Create the .env file**:
```bash
echo "WEATHER_API_KEY=your_api_key_here" > .env
```

3. **Create the project structure**:
```
weather_app/
├── src/
│   ├── main.rs              # Web server
│   ├── lib.rs               # Library exports
│   ├── models.rs            # Data structures
│   ├── weather_service.rs   # Weather API service
│   └── bin/
│       └── cli.rs           # CLI version
├── templates/
│   ├── base.html            # Base template
│   ├── index.html           # Home page
│   ├── weather.html         # Weather results
│   └── error.html           # Error page
├── Cargo.toml
├── .env
└── README.md
```

4. **Install dependencies**:
```bash
cargo build
```

## Usage

### Web Interface

1. **Start the web server**:
```bash
cargo run --bin weather-web
```

2. **Open your browser** and navigate to:
```
http://localhost:3000
```

3. **Use the interface**:
   - Enter a location (city name, coordinates, etc.)
   - Select number of forecast days (1-14)
   - Click "Get Weather" to see results

### CLI Interface

**Run the original CLI version**:
```bash
# Default location (India) for 1 day
cargo run --bin weather-cli

# Specific location
cargo run --bin weather-cli "London"

# Specific location and days
cargo run --bin weather-cli "New York" 3
```

## API Information

- **Free Tier**: Provides up to 3 days of forecast data
- **Rate Limits**: 1 million calls per month on free tier
- **Data Updates**: Weather data is updated frequently throughout the day

## Environment Variables

- `WEATHER_API_KEY`: Your WeatherAPI.com API key (required)

## Features Explained

### Web Interface Features

- **Responsive Design**: Works on desktop and mobile devices
- **Real-time Updates**: Auto-refresh every 30 minutes
- **Visual Indicators**: 
  - High rain probability hours are highlighted in red
  - Past hours are shown dimmed for today's forecast
  - Current day is marked with a "Today" badge

### Data Display

- **Current Conditions**: Temperature and weather description
- **Hourly Forecast**: Time, temperature, rain chance, and conditions
- **Multi-day View**: Organized by date with expandable hourly data
- **Smart Filtering**: Shows only future hours for today, all hours for future days

### Error Handling

- **API Errors**: Graceful handling of API failures
- **Invalid Locations**: Clear error messages for invalid city names
- **Network Issues**: Proper error reporting for connectivity problems

## Development

### Project Structure

- `src/main.rs`: Axum web server with route handlers
- `src/weather_service.rs`: Async service for API calls and data processing
- `src/models.rs`: Serde structs for API response deserialization
- `templates/`: Tera HTML templates with shared base template
- `src/bin/cli.rs`: Original CLI implementation

### Dependencies

- **Web Framework**: Axum (async web framework)
- **Templates**: Tera (Django-like template engine)
- **HTTP Client**: Reqwest (for API calls)
- **Time Handling**: Chrono (timezone-aware date/time)
- **Styling**: Embedded CSS with Font Awesome icons

### Customization

You can customize the appearance by modifying the CSS in `templates/base.html` or add new routes in `src/main.rs`.

## Troubleshooting

### Common Issues

1. **"WEATHER_API_KEY not set"**
   - Ensure your `.env` file exists and contains your API key
   - Check that the key is valid on WeatherAPI.com

2. **Template errors**
   - Ensure the `templates/` directory exists
   - Check that all template files are in place

3. **API errors**
   - Verify your internet connection
   - Check if you've exceeded API rate limits
   - Ensure location names are spelled correctly

4. **Port already in use**
   - The web server runs on port 3000 by default
   - Kill any existing processes or change the port in `main.rs`

### Getting Help

- Check the WeatherAPI.com documentation for API-related issues
- Ensure all dependencies are properly installed with `cargo build`
- Review error messages in the console for specific issues

## License

This project is for educational purposes. Please respect WeatherAPI.com's terms of service when using their API.