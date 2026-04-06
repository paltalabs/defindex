/**
 * OFAC/EU/UN Sanctioned Countries
 *
 * Country codes that are blocked from accessing the application
 * due to regulatory compliance requirements.
 *
 * Note: Russia (RU) is included to cover Crimea, Donetsk, and Luhansk
 * regions since Vercel geo-IP only provides country-level data.
 */
export const BLOCKED_COUNTRIES = [
  "KP", // North Korea — OFAC / UN / EU sanctions
  "IR", // Iran — OFAC / UN / EU sanctions
  "SY", // Syria — OFAC / UN / EU sanctions
  "CU", // Cuba — OFAC sanctions
  "RU", // Russia — EU sanctions + OFAC restrictions (includes sanctioned regions)
  "BY", // Belarus — EU sanctions / OFAC restrictions
  "VE", // Venezuela — OFAC / EU sanctions
  "SD", // Sudan — OFAC / UN sanctions
  "MM", // Myanmar — EU sanctions / OFAC restrictions
  "LY", // Libya — UN / EU / OFAC sanctions
  "AF", // Afghanistan — UN sanctions (Taliban) / OFAC restrictions
] as const;

export type BlockedCountryCode = (typeof BLOCKED_COUNTRIES)[number];

export const GEOBLOCKED_ERROR = {
  code: "GEOBLOCKED",
  message:
    "Access denied. This service is not available in your region due to regulatory restrictions.",
} as const;
