// React removed
import { render, screen, waitFor, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import { DailyScanResults } from './DailyScanResults';
import * as ipc from '../ipc';

vi.mock('../ipc', () => ({
  listDailyScanLeads: vi.fn(),
  toUserMessage: (e: unknown) => (e instanceof Error ? e.message : String(e)),
}));

describe('DailyScanResults', () => {
  it('renders aggregated badge when source_id is missing', async () => {
    vi.mocked(ipc.listDailyScanLeads).mockResolvedValue([
      {
        scan_id: 1,
        title: 'Test Topic',
        summary: 'A summary',
        original_url: 'http://example.com',
        source_id: undefined,
      }
    ]);

    render(<DailyScanResults scanId={1} />);
    
    await waitFor(() => {
      expect(screen.getByTestId('aggregated-badge')).toBeInTheDocument();
      expect(screen.getByTestId('aggregated-badge')).toHaveTextContent('Aggregated across sources');
    });
  });
  
  it('renders source ID badge when source_id is present', async () => {
    vi.mocked(ipc.listDailyScanLeads).mockResolvedValue([
      {
        scan_id: 1,
        title: 'Test Topic',
        summary: 'A summary',
        original_url: 'http://example.com',
        source_id: 42,
      }
    ]);

    render(<DailyScanResults scanId={1} />);

    await waitFor(() => {
      expect(screen.getByText('Source ID: 42')).toBeInTheDocument();
    });
  });

  it('shows empty state with a "Run scan again" button when there are no leads', async () => {
    vi.mocked(ipc.listDailyScanLeads).mockResolvedValue([]);
    const onRunScan = vi.fn();

    render(<DailyScanResults scanId={5} onRunScan={onRunScan} />);

    await waitFor(() => {
      expect(screen.getByTestId('daily-scan-empty')).toBeInTheDocument();
    });
    expect(screen.getByText(/No new leads found/i)).toBeInTheDocument();
    fireEvent.click(screen.getByTestId('daily-scan-run-again'));
    expect(onRunScan).toHaveBeenCalledTimes(1);
  });

  it('shows an error state with a Retry button that refetches', async () => {
    vi.mocked(ipc.listDailyScanLeads)
      .mockRejectedValueOnce(new Error('boom'))
      .mockResolvedValueOnce([
        { scan_id: 7, title: 'Recovered', summary: 'ok', original_url: 'http://example.com', source_id: 1 },
      ]);

    render(<DailyScanResults scanId={7} />);

    await waitFor(() => {
      expect(screen.getByTestId('daily-scan-results-error')).toBeInTheDocument();
    });
    expect(screen.getByText(/boom/)).toBeInTheDocument();

    fireEvent.click(screen.getByTestId('daily-scan-retry'));

    await waitFor(() => {
      expect(screen.getByText('Recovered')).toBeInTheDocument();
    });
  });
});
