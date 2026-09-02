export interface Card {
	rank: string;
	suit: string;
}

export interface PotEquityProblem {
	problem_id: string;
	hands: Card[][];
	board: Card[];
	num_players: number;
	pot_size: number;
	bet_to_call: number;
}

export interface PotEquityCheckResponse {
	userGuessIsCorrect: boolean;
    expectedDecision: boolean;
	playerEquity: number;
	potOdds: number; 
}

export interface PureEquityProblem {
	problem_id: string;
	hands: Card[][];
	board: Card[];
	num_players: number;
}

export interface PureEquityCheckResponse {
	userGuessIsCorrect: boolean;
	playerEquity: number;
	directionalHint: 'Higher' | 'Lower' | 'Not-Active';
}

export type AnyProblem = PotEquityProblem | PureEquityProblem;
export type AnyFeedback = PotEquityCheckResponse | PureEquityCheckResponse;
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


export interface ApiErrorBody {
	error: 'problem_gone' | 'invalid_parameter';
	message: string;
}
