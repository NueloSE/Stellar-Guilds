import { useLocale as useNextLocale } from 'next-intl';
import { isRtlLocale } from '@/lib/i18n/config';

export function useLocale() {
  const locale = useNextLocale();
  const isRTL = isRtlLocale(locale as any);
  
  return {
    locale,
    isRTL,
    direction: isRTL ? 'rtl' : 'ltr' as 'ltr' | 'rtl',
  };
}