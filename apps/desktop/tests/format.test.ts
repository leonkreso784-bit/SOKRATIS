// ZAŠTO OVAKO (cigla M2/22 — oblikovanje na jednom mjestu)
// Test tvrdi PONAŠANJE (S-012: sučelje samo oblikuje), ne slučajan izlaz jednog ICU-a — WebView2
// ima drugačiji ICU od Node-a, pa ASCII razmak i crtica moraju biti dio implementacije, ne ishoda.
import { describe, expect, it } from 'vitest';
import hr from '../src/lib/i18n/hr.json';
import { hours, num, percent, phaseState, relative, ymd } from '../src/lib/format';

describe('format', () => {
  it('postoci iz razlomka, decimalni zarez u HR', () => {
    expect(percent(0.589, 'hr')).toBe('58,9 %');
    expect(percent(0.589, 'en')).toBe('58.9%');
    expect(percent(1, 'hr')).toBe('100,0 %');
    expect(percent(null, 'hr')).toBe('—');
    expect(percent(Number.NaN, 'en')).toBe('—');
  });
  it('sati na jednu decimalu, brojevi s tisućicama', () => {
    expect(hours(12.345, 'hr')).toBe('12,3 h');
    expect(hours(0, 'en')).toBe('0.0 h');
    expect(num(70164, 'hr')).toBe('70.164');
    expect(num(70164, 'en')).toBe('70,164');
    expect(num(-5, 'hr')).toBe('-5');
  });
  it('datumi po jeziku', () => {
    expect(ymd('2026-09-18', 'hr')).toBe('18. 9. 2026.');
    expect(ymd('2026-09-18', 'en')).toBe('2026-09-18');
    expect(ymd(null, 'hr')).toBe('—');
  });
  it('relativno vrijeme kroz rječnik', () => {
    const now = 1_789_660_685;
    expect(relative(now, now - 30, hr)).toBe('upravo');
    expect(relative(now, now - 5 * 60, hr)).toBe('prije 5 min');
    expect(relative(now, now - 2 * 3600, hr)).toBe('prije 2 h');
    expect(relative(now, now - 3 * 86400, hr)).toBe('prije 3 dana');
    expect(relative(now, null, hr)).toBe('—');
  });
  it('stanje faze prevedeno', () => {
    expect(phaseState('running', hr)).toBe('u tijeku');
  });
});
