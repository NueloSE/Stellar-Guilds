import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "../../globals.css";
import { getMessages } from 'next-intl/server';
import { NextIntlClientProvider } from 'next-intl';
import { notFound } from 'next/navigation';
import { locales, defaultLocale } from '@/lib/i18n/config';

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "Stellar Guilds",
  description: "User Profile & Reputation Dashboard",
};

export default async function LocaleLayout({
  children,
  params,
}: Readonly<{
  children: React.ReactNode;
  params: { locale: string };
}>) {
  // Validate that the incoming locale is supported
  if (!locales.includes(params.locale as any)) {
    notFound();
  }

  // Get the messages for the current locale
  const messages = await getMessages();

  return (
    <html lang={params.locale} dir={params.locale === 'ar' || params.locale === 'he' ? 'rtl' : 'ltr'} className="dark">
      <body className={inter.className}>
        <NextIntlClientProvider locale={params.locale} messages={messages}>
          <div className="min-h-screen bg-stellar-navy text-stellar-white font-sans">
            {children}
          </div>
        </NextIntlClientProvider>
      </body>
    </html>
  );
}

// Generate static params for all supported locales
export function generateStaticParams() {
  return locales.map((locale) => ({ locale }));
}