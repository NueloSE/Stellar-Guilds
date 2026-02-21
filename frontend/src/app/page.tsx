import { redirect } from 'next/navigation';
import { defaultLocale } from '@/lib/i18n/config';

// Redirect to the default locale
export default function RootPage() {
  redirect(`/${defaultLocale}`);
}