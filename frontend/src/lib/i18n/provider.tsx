'use client';

import { ReactNode } from 'react';
import { NextIntlClientProvider } from 'next-intl';
import { useLocale } from 'next-intl';
import { getDirection } from './utils';
import { Locale } from './config';

interface I18nProviderProps {
  children: ReactNode;
  locale: Locale;
  messages: Record<string, unknown>;
}

export function I18nProvider({ children, locale, messages }: I18nProviderProps) {
  const direction = getDirection(locale);

  return (
    <html lang={locale} dir={direction}>
      <body>
        <NextIntlClientProvider locale={locale} messages={messages}>
          {children}
        </NextIntlClientProvider>
      </body>
    </html>
  );
}

// Custom hook to access the current locale
export function useCurrentLocale(): Locale {
  return useLocale() as Locale;
}

// Custom hook to check if the current locale is RTL
export function useIsRtl(): boolean {
  const locale = useCurrentLocale();
  return getDirection(locale) === 'rtl';
}