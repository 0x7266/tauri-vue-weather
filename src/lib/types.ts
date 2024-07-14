interface Coord {
  lat: number; // float becomes number in TypeScript
  lon: number;
}

interface City {
  coord: Coord;
  country: string;
  id: number; // u64 becomes number for simplicity
  name: string;
  population: number;
  sunrise: number;
  sunset: number;
  timezone: number;
}

interface Main {
  feels_like: number;
  humidity: number;
  temp: number;
  temp_max: number;
  temp_min: number;
}

interface Weather {
  description: string;
  icon: string;
  id: number;
  main: string;
}

interface Day {
  dt_txt: string;
  main: Main;
  weather: Weather[]; // Array of Weather objects
}

export interface Data {
  city: City;
  list: Day[];
}

//interface Geo {
//  coord: Coord;
//}
