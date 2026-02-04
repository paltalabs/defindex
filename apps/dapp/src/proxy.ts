import { NextResponse } from "next/server";
import type { NextRequest } from "next/server";
import {
  BLOCKED_COUNTRIES,
  GEOBLOCKED_ERROR,
} from "@/lib/geo/blocked-countries";

/**
 * Geoblocking proxy for OFAC/EU/UN sanctions compliance.
 *
 * Blocks users from sanctioned countries before the app renders.
 * - Page routes: Redirects to /blocked
 * - API routes: Returns 403 JSON response
 */
export function proxy(request: NextRequest) {
  const pathname = request.nextUrl.pathname;

  // Get country from Vercel geo headers (x-vercel-ip-country)
  // In development, also check TEST_GEO_COUNTRY env var for testing
  const vercelCountry = request.headers.get("x-vercel-ip-country");
  const testCountry = process.env.TEST_GEO_COUNTRY;
  const country = testCountry || vercelCountry;

  // Clone request headers and add pathname for layout
  const requestHeaders = new Headers(request.headers);
  requestHeaders.set("x-pathname", pathname);

  // Allow the blocked page to render (prevent redirect loop)
  if (pathname === "/blocked") {
    return NextResponse.next({
      request: { headers: requestHeaders },
    });
  }

  // Fail open: if geo data is unavailable, allow the request
  if (!country) {
    return NextResponse.next({
      request: { headers: requestHeaders },
    });
  }

  // Check if country is blocked
  const isBlocked = BLOCKED_COUNTRIES.includes(
    country as (typeof BLOCKED_COUNTRIES)[number]
  );

  if (!isBlocked) {
    return NextResponse.next({
      request: { headers: requestHeaders },
    });
  }

  // Handle blocked API routes with JSON response
  if (pathname.startsWith("/api/")) {
    return NextResponse.json(GEOBLOCKED_ERROR, { status: 403 });
  }

  // Redirect page requests to blocked page
  return NextResponse.redirect(new URL("/blocked", request.url));
}

export const config = {
  matcher: [
    /*
     * Match all request paths except:
     * - _next/static (static files)
     * - _next/image (image optimization)
     * - favicon.ico (favicon file)
     * - Image/asset files
     */
    "/((?!_next/static|_next/image|favicon.ico|.*\\.(?:svg|png|jpg|jpeg|gif|webp|ico)$).*)",
  ],
};
