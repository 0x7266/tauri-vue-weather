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

export function convertCountryCode(country) {
  let regionNames = new Intl.DisplayNames(["en"], { type: "region" });
  return regionNames.of(country);
}
