import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { ExpertChatPanel } from '../ExpertChatPanel';

describe('ExpertChatPanel', () => {
  it('renders the expert chat interface and primary assistance topics', () => {
    render(<ExpertChatPanel />);

    expect(
      screen.getByRole('heading', { name: /ask a carbon farming expert/i })
    ).toBeInTheDocument();
    expect(screen.getAllByText(/carbon farming/i).length).toBeGreaterThan(0);
    expect(screen.getAllByText(/soil health/i).length).toBeGreaterThan(0);
    expect(screen.getAllByText(/certification/i).length).toBeGreaterThan(0);
    expect(screen.getAllByText(/market prices/i).length).toBeGreaterThan(0);
    expect(screen.getByRole('textbox', { name: /message/i })).toBeInTheDocument();
  });

  it('allows farmers to send questions about soil health and carbon farming', () => {
    render(<ExpertChatPanel />);

    const textarea = screen.getByRole('textbox', { name: /message/i });
    const sendButton = screen.getByRole('button', { name: /send question/i });

    fireEvent.change(textarea, { target: { value: 'How can I measure my soil carbon levels?' } });
    expect(sendButton).not.toBeDisabled();

    fireEvent.click(sendButton);
    expect(screen.getByText('How can I measure my soil carbon levels?')).toBeInTheDocument();
  });

  it('allows clicking quick topic suggestions to ask questions', () => {
    render(<ExpertChatPanel />);

    const topicButton = screen.getByRole('button', { name: /^soil health$/i });
    fireEvent.click(topicButton);

    expect(screen.getAllByText('Soil health').length).toBeGreaterThan(0);
  });
});
