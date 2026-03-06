import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import { ResearchPhaseIndicator } from './ResearchPhaseIndicator';

describe('ResearchPhaseIndicator', () => {
  it('should render nothing when currentPhase is null', () => {
    const { container } = render(<ResearchPhaseIndicator currentPhase={null} />);
    expect(container).toBeEmptyDOMElement();
  });

  it('should render search label for web_search', () => {
    render(<ResearchPhaseIndicator currentPhase="web_search" />);
    expect(screen.getByText('🔍 ウェブを検索中...')).toBeInTheDocument();
  });

  it('should render fetch label for web_fetch', () => {
    render(<ResearchPhaseIndicator currentPhase="web_fetch" />);
    expect(screen.getByText('📄 ページを取得中...')).toBeInTheDocument();
  });

  it('should render generic label for unknown tool names', () => {
    render(<ResearchPhaseIndicator currentPhase="some_unknown_tool" />);
    expect(screen.getByText('⚙️ 処理中...')).toBeInTheDocument();
  });

  it('should render three thinking dots inside ResearchPhaseIndicator', () => {
    const { container } = render(<ResearchPhaseIndicator currentPhase="web_search" />);
    const dots = container.querySelectorAll('.thinking-dot');
    expect(dots).toHaveLength(3)
  });

});
