import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import DailyScanResults from './DailyScanResults';

describe('DailyScanResults', () => {
  it('renders 3 fixture leads with tier badges', () => {
    const leads = [
      { id: 1, headline: 'H1', tier: 'official_record', source: 'S1', url: 'U1', confidence: 'high', action: 'A1', beat: 'B1', details: 'D1' },
      { id: 2, headline: 'H2', tier: 'news_reporting', source: 'S2', url: 'U2', confidence: 'medium', action: 'A2', beat: 'B2', details: 'D2' },
      { id: 3, headline: 'H3', tier: 'community_signal', source: 'S3', url: 'U3', confidence: 'low', action: 'A3', beat: 'B3', details: 'D3' }
    ];
    render(<DailyScanResults leads={leads} onOpenWorkbench={vi.fn()} />);
    expect(screen.getByText('H1')).toBeInTheDocument();
    expect(screen.getByText('official_record')).toBeInTheDocument();
  });

  it('Open in Workbench fires the right action with the right lead id', () => {
    const onOpenWorkbench = vi.fn();
    const leads = [
      { id: 1, headline: 'H1', tier: 'official_record', source: 'S1', url: 'U1', confidence: 'high', action: 'A1', beat: 'B1', details: 'D1' }
    ];
    render(<DailyScanResults leads={leads} onOpenWorkbench={onOpenWorkbench} />);
    fireEvent.click(screen.getByText('Open in Workbench'));
    expect(onOpenWorkbench).toHaveBeenCalledWith(1);
  });
});
