// TypeScript port of backend `domain::money::Kobo::Display`. Single
// source of truth for kobo -> ₦ formatting in the mobile app — no UI
// component should hand-roll this.
//
//   formatNaira(0)        -> "₦0.00"
//   formatNaira(100)      -> "₦1.00"
//   formatNaira(1_234_567)-> "₦12,345.67"
//   formatNaira(-1_000)   -> "-₦10.00"
export function formatNaira(kobo: number): string {
  if (!Number.isFinite(kobo)) return '₦?';
  const truncated = Math.trunc(kobo);
  const sign = truncated < 0 ? '-' : '';
  const abs = Math.abs(truncated);
  const naira = Math.floor(abs / 100);
  const koboPart = abs % 100;
  return `${sign}₦${groupThousands(naira)}.${String(koboPart).padStart(2, '0')}`;
}

function groupThousands(n: number): string {
  return String(n).replace(/\B(?=(\d{3})+(?!\d))/g, ',');
}
