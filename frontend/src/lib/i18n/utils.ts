import { Locale, isRtlLocale } from './config';

/**
 * Gets the HTML direction attribute based on the locale
 */
export function getDirection(locale: Locale): 'ltr' | 'rtl' {
  return isRtlLocale(locale) ? 'rtl' : 'ltr';
}

/**
 * Formats a number based on the locale
 */
export function formatNumber(
  locale: Locale, 
  number: number, 
  options?: Intl.NumberFormatOptions
): string {
  return new Intl.NumberFormat(locale, options).format(number);
}

/**
 * Formats a date based on the locale
 */
export function formatDate(
  locale: Locale,
  date: Date | number,
  options?: Intl.DateTimeFormatOptions
): string {
  return new Intl.DateTimeFormat(locale, options).format(date);
}

/**
 * Formats a currency based on the locale
 */
export function formatCurrency(
  locale: Locale,
  amount: number,
  currency: string = 'USD'
): string {
  return new Intl.NumberFormat(locale, {
    style: 'currency',
    currency: currency,
  }).format(amount);
}

/**
 * Formats a relative time based on the locale
 */
export function formatRelativeTime(
  locale: Locale,
  value: number,
  unit: Intl.RelativeTimeFormatUnit,
  options?: Intl.RelativeTimeFormatOptions
): string {
  return new Intl.RelativeTimeFormat(locale, options).format(value, unit);
}