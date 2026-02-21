import createMiddleware from 'next-intl/middleware';
import { locales, defaultLocale } from './lib/i18n/config';

export default createMiddleware({
  // A list of all locales that are supported
  locales,
  
  // Used when no locale matches
  defaultLocale,
  
  // If enabled, the locale detection will be skipped when the cookie is present
  localeDetection: true,
});

export const config = {
  // Match only internationalized pathnames
  matcher: [
    // Skip all internal paths (_next)
    '/((?!api|_next|.*\\..*).*)',
  ],
};