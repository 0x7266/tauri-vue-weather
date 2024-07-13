interface Coord {
  lat: number;
  lon: number;
}

interface Weather {
  feels_like: number;
  grnd_level: number;
  humidity: number;
  pressure: number;
  sea_level: number;
  temp: number;
  temp_max: number;
  temp_min: number;
}

interface Wind {
  deg: number;
  speed: number;
}

export interface Response {
  coord: Coord;
  main: Weather;
  name: string;
  timezone: number;
  visibility: number;
  wind: Wind;
}
