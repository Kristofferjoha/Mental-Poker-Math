
export const RANKS = [
  'Two', 'Three', 'Four', 'Five', 'Six', 'Seven', 'Eight',
  'Nine', 'Ten', 'Jack', 'Queen', 'King', 'Ace'
] as const;

export const SUITS = ['Clubs', 'Diamonds', 'Hearts', 'Spades'] as const;

export type Rank = (typeof RANKS)[number];
export type Suit = (typeof SUITS)[number];

export interface Card {
  rank: Rank;
  suit: Suit;
}

export type Hands = Card[][];


export interface PureEquityProblem {
  problem_id: string;
  hands: Hands;
  board: Card[];
  num_players: number;
  player_equity: number;
  lower_bound: number;
  upper_bound: number;
}

export interface PureEquityAnswer {
  userGuessIsCorrect: boolean;
  playerEquity: number;
}


export interface PotEquityProblem {
  problem_id: string;
  hands: Hands;
  board: Card[];
  num_players: number;
  pot_size: number;
  bet_to_call: number;
}

export interface PotEquityAnswer {
  userGuessIsCorrect: boolean;
  expectedDecision: boolean;
  playerEquity: number;
  potOdds: number;
}


export type Difficulty = 'easy' | 'medium' | 'hard';

export interface NutsProblem {
  problem_id: string;
  board: Card[];
  candidates: Hands;
  difficulty: Difficulty;
}

export interface NutsAnswer {
  correct: boolean;
  correctIndex: number;
  correctHand: Card[];
}


export interface KingOfHillProblem {
  problem_id: string;
  board: Card[];
  hands: Hands;
  numHands: number;
}

export interface KingOfHillAnswer {
  correctOrder: number[];
  equities: number[];
  correctPairs: number;
  totalPairs: number;
  kendallTau: number;
  perfect: boolean;
}


export interface ApiErrorBody {
  error: 'problem_gone' | 'invalid_parameter';
  message: string;
}
