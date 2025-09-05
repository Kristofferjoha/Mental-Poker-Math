// ===================================================================
// 1. CORE / SHARED TYPES
// ===================================================================

export interface Card {
	rank: string;
	suit: string;
}

// ===================================================================
// 2. POT ODDS + EV DECISION GAME (/pot-odds-ev)
// ===================================================================

export interface PotEquityProblem {
	problem_id: string;
	player_hand: Card[];
	opponent_hand: Card[];
	board: Card[];
	pot_size: number;
	bet_to_call: number;
}

export interface PotEquityCheckResponse {
	userGuessIsCorrect: boolean;
    expectedDecision: boolean;
	playerEquity: number;
	potOdds: number; 
}

// ===================================================================
// 3. PURE EQUITY GAME (/pure-equity)
// ===================================================================

export interface PureEquityProblem {
	problem_id: string;
	player_hand: Card[];
	opponent_hand: Card[];
	board: Card[];
}

export interface PureEquityCheckResponse {
	userGuessIsCorrect: boolean;
	playerEquity: number;
	directionalHint: 'Higher' | 'Lower' | 'Not-Active';
}

// ===================================================================
// 4. PURE POT ODDS GAME (/pure-pot-odds)
// ===================================================================


export interface PurePotOddsProblem {
	problem_id: string;
	pot_size: number;
	bet_to_call: number;
	equity: number;
}

export interface PurePotOddsCheckResponse {
	userGuessIsCorrect: boolean;
	expectedDecision: boolean;
	potOdds: number; 
}

// ===================================================================
// 5. SHARED TYPES
// ===================================================================

export type AnyProblem = PotEquityProblem | PureEquityProblem | PurePotOddsProblem;
export type AnyFeedback = PotEquityCheckResponse | PureEquityCheckResponse | PurePotOddsCheckResponse;
export type CardBasedProblem = PotEquityProblem | PureEquityProblem;

interface PotEquityHistoryItem {
	game: 'pot-odds-equity';
	problem: PotEquityProblem;
	response: PotEquityCheckResponse;
	userDecision: boolean;
}

interface PureEquityHistoryItem {
	game: 'pure-equity';
	problem: PureEquityProblem;
	response: PureEquityCheckResponse;
	userGuess: number;
}

export type HistoryItem = PotEquityHistoryItem | PureEquityHistoryItem;

export type OptionChoice = { label: string; value: string | number | boolean };

export type OptionConfig = {
	id: string;
	label: string;
	type: 'select' | 'checkbox' | 'checkbox-group';
	defaultValue: number | boolean | string[];
	choices?: OptionChoice[];
};