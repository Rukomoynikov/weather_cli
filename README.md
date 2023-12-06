# Weather CLI

This package allows you to fetch weather date from [OpenWeatherMap](http://openweathermap.org/) to your terminal. To use it you need to get an API key on their site.

## How to use it 

With cargo installed, run the following command:

```bash
1. cargo install wthr
2. wthr config set api_key <your_api_key>
3. wthr current <city_name>

# Example result
> Temperature: 5.68
> Feels like:  1.66
> Wind speed:  6.26
```