// React removed
import { render, screen, waitFor, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import { DailyScanResults } from './DailyScanResults';
import * as ipc from '../ipc';

vi.mock('../ipc', () => ({
  listDailyScanLeads: vi.fn(),
  openExternalUrl: vi.fn(),
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
        why_flagged: 'Multiple sources mention the same civic action.',
        source_name: 'Watched sources',
        source_type: 'agenda',
        priority: 'high',
        suggested_next_step: 'Confirm the vote date.',
      }
    ]);

    render(<DailyScanResults scanId={1} />);
    
    await waitFor(() => {
      expect(screen.getByTestId('aggregated-badge')).toBeInTheDocument();
      expect(screen.getByTestId('aggregated-badge')).toHaveTextContent('Aggregated across sources');
      expect(screen.getByText('Why this was flagged')).toBeInTheDocument();
      expect(screen.getByText('Multiple sources mention the same civic action.')).toBeInTheDocument();
      expect(screen.getByText('Suggested next step')).toBeInTheDocument();
      expect(screen.getByText('Confirm the vote date.')).toBeInTheDocument();
      expect(screen.getByText('High priority')).toBeInTheDocument();
    });
  });
  
  it('renders source context instead of leading with raw source ID', async () => {
    vi.mocked(ipc.listDailyScanLeads).mockResolvedValue([
      {
        scan_id: 1,
        title: 'Test Topic',
        summary: 'A summary',
        original_url: 'http://example.com',
        source_id: 42,
        source_name: 'Council Agenda Center',
        source_type: 'primary_record',
        priority: 'medium',
      }
    ]);

    render(<DailyScanResults scanId={1} />);

    await waitFor(() => {
      expect(screen.getByText('Council Agenda Center / primary record')).toBeInTheDocument();
      expect(screen.getByText('Source #42')).toBeInTheDocument();
      expect(screen.queryByText('Source ID: 42')).not.toBeInTheDocument();
    });
  });

  it('surfaces story-quality metadata for background and verification decisions', async () => {
    vi.mocked(ipc.listDailyScanLeads).mockResolvedValue([
      {
        scan_id: 1,
        title: 'Council video archive',
        summary: 'The city maintains an archive of past meetings.',
        original_url: 'https://example.gov/videos',
        source_name: 'Council Archive',
        source_type: 'official update',
        priority: 'low',
        story_type: 'background',
        disposition: 'background',
        what_changed: 'no current change found',
        novelty: 1,
        publishability_note: 'A newly posted vote, transcript, or deadline would make this publishable.',
      }
    ]);

    render(<DailyScanResults scanId={1} />);

    await waitFor(() => {
      expect(screen.getByText('Background')).toBeInTheDocument();
      expect(screen.getByText('background')).toBeInTheDocument();
      expect(screen.getByText(/Why now:/)).toBeInTheDocument();
      expect(screen.getByText(/no current change found/)).toBeInTheDocument();
      expect(screen.getByText(/Novelty:/)).toBeInTheDocument();
      expect(screen.getByText(/1\/5/)).toBeInTheDocument();
      expect(screen.getByText(/Before publishing:/)).toBeInTheDocument();
      expect(screen.getByText(/newly posted vote/)).toBeInTheDocument();
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

  it('opens original source links through the desktop opener', async () => {
    vi.mocked(ipc.listDailyScanLeads).mockResolvedValue([
      {
        scan_id: 3,
        title: 'Public hearing',
        summary: 'A hearing was posted.',
        original_url: 'https://example.gov/hearing',
        source_id: 1,
      },
    ]);
    vi.mocked(ipc.openExternalUrl).mockResolvedValue();

    render(<DailyScanResults scanId={3} />);

    const link = await screen.findByRole('link', { name: /Open source and review/i });
    fireEvent.click(link);

    await waitFor(() => {
      expect(ipc.openExternalUrl).toHaveBeenCalledWith('https://example.gov/hearing');
    });
  });
});
