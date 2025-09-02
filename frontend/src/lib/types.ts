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

export interface PotEvProblem {
	problem_id: string;
	player_hand: Card[];
	opponent_hand: Card[];
	board: Card[];
	pot_size: number;
	bet_to_call: number;
}

export interface PotEvCheckResponse {
	isCorrect: boolean;
	correctDecision: boolean;
	playerEquity: number;
	potOdds: number; 
}

// ===================================================================
// 3. PURE EQUITY GAME (/pure-equity)
// ===================================================================

export interface PureEqProblem {
	problem_id: string;
	player_hand: Card[];
	opponent_hand: Card[];
	board: Card[];
}

export interface PureEqCheckResponse {
	isCorrect: boolean;
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
	isCorrect: boolean;
	correctDecision: boolean;
	potOdds: number; 
}

// ===================================================================
// 5. SHARED TYPES
// ===================================================================

export type AnyProblem = PotEvProblem | PureEqProblem | PurePotOddsProblem;
export type AnyFeedback = PotEvCheckResponse | PureEqCheckResponse | PurePotOddsCheckResponse;
export type CardBasedProblem = PotEvProblem | PureEqProblem;

interface PotEvHistoryItem {
	game: 'pot-odds-ev';
	problem: PotEvProblem;
	response: PotEvCheckResponse;
	userDecision: boolean;
}

interface PureEqHistoryItem {
	game: 'pure-equity';
	problem: PureEqProblem;
	response: PureEqCheckResponse;
	userGuess: number;
}

export type HistoryItem = PotEvHistoryItem | PureEqHistoryItem;

export type OptionChoice = { label: string; value: string | number | boolean };

export type OptionConfig = {
	id: string;
	label: string;
	type: 'select' | 'checkbox' | 'checkbox-group';
	defaultValue: number | boolean | string[];
	choices?: OptionChoice[];
};