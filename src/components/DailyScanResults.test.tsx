// React removed
import { render, screen, waitFor } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import { DailyScanResults } from './DailyScanResults';
import * as ipc from '../ipc';

vi.mock('../ipc', () => ({
  listDailyScanLeads: vi.fn(),
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
});
