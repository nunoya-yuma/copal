import type { ChatRequest, SseEvent } from '../types';

/**
 * SSEストリームを開始し、イベントを処理する
 *
 * TODO(human): この関数を実装してください
 *
 * 実装のヒント:
 * 1. fetch()で /api/chat にPOSTリクエスト
 * 2. response.body.getReader()でストリーム取得
 * 3. TextDecoderでバイト列を文字列に変換
 * 4. バッファリング: buffer.split('\n')で行分割、lines.pop()で未完成行を保持
 * 5. 'data: 'で始まる行をJSON.parse()してonEvent()に渡す
 * 6. クリーンアップ関数を返す（reader.cancel()）
 *
 * 完全な実装例は ~/.claude/plans/concurrent-churning-finch.md の
 * 「完全な実装例（参照用）」セクションにあります。
 * どうしても詰まったときのみ参照してください。
 */
export async function startChatStream(
  request: ChatRequest,
  onEvent: (event: SseEvent) => void
): Promise<() => void> {
  // TODO(human): fetchでPOSTリクエストを送信
  // const response = await fetch('/api/chat', { ... });

  // TODO(human): response.body.getReader()でストリーム取得

  // TODO(human): TextDecoderとバッファを初期化

  // TODO(human): readLoopを実装（while文でreader.read()を繰り返す）

  // TODO(human): バッファリング処理を実装
  // - buffer += decoder.decode(value, { stream: true })
  // - const lines = buffer.split('\n')
  // - buffer = lines.pop() || ''

  // TODO(human): 'data: 'で始まる行を処理

  // TODO(human): クリーンアップ関数を返す
  throw new Error('Not implemented');
}
