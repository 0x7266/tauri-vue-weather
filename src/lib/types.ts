export interface Data {
  location: Location;
  current: Current;
  forecast: Forecast;
}

export interface Autocomplete {
  country: string;
  id: number; // Change u64 to number for simplicity
  lat: number; // Change f32 to number for simplicity
  lon: number; // Change f32 to number for simplicity
  name: string;
  region: string;
}

export interface Location {
  name: string;
  region: string;
  country: string;
  lat: number; // Change f32 to number for simplicity
  lon: number; // Change f32 to number for simplicity
  tz_id: string;
  localtime_epoch: number; // Change u64 to number for simplicity
  localtime: string;
}

export interface Current {
  condition: Condition;
  feelslike_c: number; // Change f32 to number for simplicity
  feelslike_f: number; // Change f32 to number for simplicity
  humidity: number; // Change u64 to number for simplicity
  is_day: boolean;
  last_updated: string;
  last_updated_epoch: number; // Change u64 to number for simplicity
  precip_in: number; // Change u64 to number for simplicity
  precip_mm: number; // Change u64 to number for simplicity
  pressure_in: number; // Change f32 to number for simplicity
  pressure_mb: number; // Change u32 to number for simplicity
  temp_c: number; // Change f32 to number for simplicity
  temp_f: number; // Change f32 to number for simplicity
  wind_degree: number; // Change f32 to number for simplicity
  wind_dir: string;
  wind_kph: number; // Change f64 to number for simplicity
  wind_mph: number; // Change f64 to number for simplicity
}

export interface Condition {
  code: number; // Change u32 to number for simplicity
  icon: string;
  text: string;
}

export interface Forecast {
  forecastday: Forecastday[];
}

export interface Forecastday {
  date: string;
  date_epoch: string;
  day: Day;
}

export interface Day {
  avghumidity: number; // Change u32 to number for simplicity
  avgtemp_c: number; // Change f32 to number for simplicity
  avgtemp_f: number; // Change f32 to number for simplicity
  avgvis_km: number; // Change u32 to number for simplicity
  avgvis_miles: number; // Change u32 to number for simplicity
  condition: Condition;
  daily_chance_of_rain: number; // Change u8 to number for simplicity
  daily_chance_of_snow: number; // Change u8 to number for simplicity
  daily_will_it_rain: number; // Change u8 to number for simplicity
  daily_will_it_snow: number; // Change u8 to number for simplicity
  maxtemp_c: number; // Change f32 to number for simplicity
  maxtemp_f: number; // Change f32 to number for simplicity
  maxwind_kph: number; // Change f32 to number for simplicity
  maxwind_mph: number; // Change f32 to number for simplicity
  mintemp_c: number; // Change f32 to number for simplicity
  mintemp_f: number; // Change f32 to number for simplicity
  totalprecip_in: number; // Change f32 to number for simplicity
  totalprecip_mm: number; // Change f32 to number for simplicity
  totalsnow_cm: number; // Change f32 to number for simplicity
}
