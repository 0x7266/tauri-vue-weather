export function convertTimeStamp(timestamp: number, timezone: number): string {
  const convertTimezone = timezone / 3600; // convert seconds to hours

  const date = new Date(timestamp * 1000);

  return date.toLocaleString("en-US", {
    weekday: "long",
    day: "numeric",
    month: "long",
    year: "numeric",
    hour: "numeric",
    minute: "numeric",
    timeZone: `Etc/GMT${convertTimezone >= 0 ? "-" : "+"}${Math.abs(convertTimezone)}`,
    hour12: true,
  });
}

export function convertCountryCode(country: string) {
  let regionNames = new Intl.DisplayNames(["en"], { type: "region" });
  return regionNames.of(country);
}

export const initialDataPlaceholder = {
  location: {
    name: "",
    region: "",
    country: "",
    lat: 0,
    lon: 0,
    tz_id: "",
    localtime_epoch: 0,
    localtime: "",
  },
  current: {
    condition: {
      code: 0,
      icon: "",
      text: "",
    },
    feelslike_c: 0,
    feelslike_f: 0,
    humidity: 0,
    is_day: false,
    last_updated: "",
    last_updated_epoch: 0,
    precip_in: 0,
    precip_mm: 0,
    pressure_in: 0,
    pressure_mb: 0,
    temp_c: 0,
    temp_f: 0,
    wind_degree: 0,
    wind_dir: "",
    wind_kph: 0,
    wind_mph: 0,
  },
  forecast: {
    forecastday: [],
  },
};

export const initialAutocompletePlaceholder = {
  country: "",
  id: 0,
  lat: 0,
  lon: 0,
  name: "",
  region: "",
};
