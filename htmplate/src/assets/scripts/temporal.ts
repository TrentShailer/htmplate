import { Intl, Temporal } from "npm:temporal-polyfill@0.3.0";

export function parseUtcToLocalDateTime(utcTimestamp: string): Temporal.ZonedDateTime {
  const timezone = Intl.DateTimeFormat().resolvedOptions().timeZone;
  return Temporal.Instant.from(utcTimestamp).toZonedDateTimeISO(timezone);
}

export { Intl, Temporal };
