import { ThinkingDots } from './ThinkingDots';

interface ResearchPhaseIndicatorProps {
  currentPhase: string | null;
}

function getPhaseLabel(toolName: string): string {
  let label;
  switch (toolName) {
    case 'web_search':
      label = '🔍 ウェブを検索中...';
      break;

    case 'web_fetch':
      label = '📄 ページを取得中...';
      break;

    default:
      label = '⚙️ 処理中...';
      break;
  }
  return label;
}

export function ResearchPhaseIndicator({ currentPhase }: ResearchPhaseIndicatorProps) {
  if (!currentPhase) return null;

  return (
    <div className="research-phase-indicator">
      <ThinkingDots />
      {getPhaseLabel(currentPhase)}
    </div>
  );
}
