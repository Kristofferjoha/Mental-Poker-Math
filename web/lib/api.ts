import type {
  KingOfHillAnswer,
  KingOfHillProblem,
  NutsAnswer,
  NutsProblem,
  PotEquityAnswer,
  PotEquityProblem,
  PureEquityAnswer,
  PureEquityProblem,
  ApiErrorBody
} from './types';

const API_BASE =
  process.env.NEXT_PUBLIC_API_URL ?? 'http://127.0.0.1:8001';

export class ApiError extends Error {
  constructor(
    readonly status: number,
    readonly code: ApiErrorBody['error'] | 'unknown',
    message: string
  ) {
    super(message);
    this.name = 'ApiError';
  }

  get isGone() {
    return this.code === 'problem_gone';
  }
}

async function request<T>(path: string, init?: RequestInit): Promise<T> {
  let response: Response;
  try {
    response = await fetch(`${API_BASE}${path}`, init);
  } catch {
    throw new ApiError(0, 'unknown', 'Could not reach the server. Is the backend running?');
  }

  if (!response.ok) {
    const body = (await response.json().catch(() => null)) as ApiErrorBody | null;
    throw new ApiError(
      response.status,
      body?.error ?? 'unknown',
      body?.message ?? `Request failed with status ${response.status}`
    );
  }
  return response.json() as Promise<T>;
}

function get<T>(path: string, params: Record<string, string | number | undefined> = {}) {
  const query = new URLSearchParams();
  for (const [key, value] of Object.entries(params)) {
    if (value !== undefined) query.set(key, String(value));
  }
  const suffix = query.toString();
  return request<T>(suffix ? `${path}?${suffix}` : path);
}

function post<T>(path: string, body: unknown) {
  return request<T>(path, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body)
  });
}

export const api = {
  pureEquity: {
    generate: async (opts: { streets?: string; numPlayers?: number; tolerance?: number }) => {
      const problem = await get<PureEquityProblem>('/api/pure-equity-get-problem', {
        streets: opts.streets,
        numPlayers: opts.numPlayers,
        tolerance: opts.tolerance
      });
      const band = [problem.player_equity, problem.lower_bound, problem.upper_bound];
      if (!band.every((n) => typeof n === 'number' && Number.isFinite(n))) {
        throw new ApiError(
          500,
          'unknown',
          'The API returned a problem with no answer band. It is running an older build than this app expects.'
        );
      }
      return problem;
    },
    check: (problemId: string, guessValue: number) =>
      post<PureEquityAnswer>('/api/pure-equity-check-answer', {
        problemId,
        guess_value: guessValue
      })
  },

  potEquity: {
    generate: (opts: { streets?: string; numPlayers?: number; allowOverbets?: boolean }) =>
      get<PotEquityProblem>('/api/pot-equity-get-problem', {
        streets: opts.streets,
        numPlayers: opts.numPlayers,
        allowOverbets: opts.allowOverbets === undefined ? undefined : String(opts.allowOverbets)
      }),
    check: (problemId: string, decision: boolean) =>
      post<PotEquityAnswer>('/api/pot-equity-check-answer', { problemId, decision })
  },

  whatsTheNuts: {
    generate: (opts: { difficulty?: string }) =>
      get<NutsProblem>('/api/whats-the-nuts-get-problem', opts),
    check: (problemId: string, selectedIndex: number | null) =>
      post<NutsAnswer>('/api/whats-the-nuts-check-answer', { problemId, selectedIndex })
  },

  kingOfTheHill: {
    generate: (opts: { numHands?: number; streets?: string }) =>
      get<KingOfHillProblem>('/api/king-of-the-hill-get-problem', opts),
    check: (problemId: string, orderedIndices: number[]) =>
      post<KingOfHillAnswer>('/api/king-of-the-hill-check-answer', {
        problemId,
        orderedIndices
      })
  }
};
