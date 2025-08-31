type DynamicRoutes = {
	
};

type Layouts = {
	"/": undefined;
	"/pot-odds-game": undefined;
	"/pure-eq-game": undefined
};

export type RouteId = "/" | "/pot-odds-game" | "/pure-eq-game";

export type RouteParams<T extends RouteId> = T extends keyof DynamicRoutes ? DynamicRoutes[T] : Record<string, never>;

export type LayoutParams<T extends RouteId> = Layouts[T] | Record<string, never>;

export type Pathname = "/" | "/pot-odds-game" | "/pure-eq-game";

export type ResolvedPathname = `${"" | `/${string}`}${Pathname}`;

export type Asset = "/cards/Ace_of_clubs.svg" | "/cards/Ace_of_diamonds.svg" | "/cards/Ace_of_hearts.svg" | "/cards/Ace_of_spades.svg" | "/cards/Eight_of_clubs.svg" | "/cards/Eight_of_diamonds.svg" | "/cards/Eight_of_hearts.svg" | "/cards/Eight_of_spades.svg" | "/cards/Five_of_clubs.svg" | "/cards/Five_of_diamonds.svg" | "/cards/Five_of_hearts.svg" | "/cards/Five_of_spades.svg" | "/cards/Four_of_clubs.svg" | "/cards/Four_of_diamonds.svg" | "/cards/Four_of_hearts.svg" | "/cards/Four_of_spades.svg" | "/cards/Jack_of_clubs.svg" | "/cards/Jack_of_diamonds.svg" | "/cards/Jack_of_hearts.svg" | "/cards/Jack_of_spades.svg" | "/cards/King_of_clubs.svg" | "/cards/King_of_diamonds.svg" | "/cards/King_of_hearts.svg" | "/cards/King_of_spades.svg" | "/cards/Nine_of_clubs.svg" | "/cards/Nine_of_diamonds.svg" | "/cards/Nine_of_hearts.svg" | "/cards/Nine_of_spades.svg" | "/cards/Queen_of_clubs.svg" | "/cards/Queen_of_diamonds.svg" | "/cards/Queen_of_hearts.svg" | "/cards/Queen_of_spades.svg" | "/cards/Seven_of_clubs.svg" | "/cards/Seven_of_diamonds.svg" | "/cards/Seven_of_hearts.svg" | "/cards/Seven_of_spades.svg" | "/cards/Six_of_clubs.svg" | "/cards/Six_of_diamonds.svg" | "/cards/Six_of_hearts.svg" | "/cards/Six_of_spades.svg" | "/cards/Ten_of_clubs.svg" | "/cards/Ten_of_diamonds.svg" | "/cards/Ten_of_hearts.svg" | "/cards/Ten_of_spades.svg" | "/cards/Three_of_clubs.svg" | "/cards/Three_of_diamonds.svg" | "/cards/Three_of_hearts.svg" | "/cards/Three_of_spades.svg" | "/cards/Two_of_clubs.svg" | "/cards/Two_of_diamonds.svg" | "/cards/Two_of_hearts.svg" | "/cards/Two_of_spades.svg" | "/cards_unop/Ace_of_clubs.svg" | "/cards_unop/Ace_of_diamonds.svg" | "/cards_unop/Ace_of_hearts.svg" | "/cards_unop/Ace_of_spades.svg" | "/cards_unop/Eight_of_clubs.svg" | "/cards_unop/Eight_of_diamonds.svg" | "/cards_unop/Eight_of_hearts.svg" | "/cards_unop/Eight_of_spades.svg" | "/cards_unop/Five_of_clubs.svg" | "/cards_unop/Five_of_diamonds.svg" | "/cards_unop/Five_of_hearts.svg" | "/cards_unop/Five_of_spades.svg" | "/cards_unop/Four_of_clubs.svg" | "/cards_unop/Four_of_diamonds.svg" | "/cards_unop/Four_of_hearts.svg" | "/cards_unop/Four_of_spades.svg" | "/cards_unop/Jack_of_clubs.svg" | "/cards_unop/Jack_of_diamonds.svg" | "/cards_unop/Jack_of_hearts.svg" | "/cards_unop/Jack_of_spades.svg" | "/cards_unop/King_of_clubs.svg" | "/cards_unop/King_of_diamonds.svg" | "/cards_unop/King_of_hearts.svg" | "/cards_unop/King_of_spades.svg" | "/cards_unop/Nine_of_clubs.svg" | "/cards_unop/Nine_of_diamonds.svg" | "/cards_unop/Nine_of_hearts.svg" | "/cards_unop/Nine_of_spades.svg" | "/cards_unop/Queen_of_clubs.svg" | "/cards_unop/Queen_of_diamonds.svg" | "/cards_unop/Queen_of_hearts.svg" | "/cards_unop/Queen_of_spades.svg" | "/cards_unop/Seven_of_clubs.svg" | "/cards_unop/Seven_of_diamonds.svg" | "/cards_unop/Seven_of_hearts.svg" | "/cards_unop/Seven_of_spades.svg" | "/cards_unop/Six_of_clubs.svg" | "/cards_unop/Six_of_diamonds.svg" | "/cards_unop/Six_of_hearts.svg" | "/cards_unop/Six_of_spades.svg" | "/cards_unop/Ten_of_clubs.svg" | "/cards_unop/Ten_of_diamonds.svg" | "/cards_unop/Ten_of_hearts.svg" | "/cards_unop/Ten_of_spades.svg" | "/cards_unop/Three_of_clubs.svg" | "/cards_unop/Three_of_diamonds.svg" | "/cards_unop/Three_of_hearts.svg" | "/cards_unop/Three_of_spades.svg" | "/cards_unop/Two_of_clubs.svg" | "/cards_unop/Two_of_diamonds.svg" | "/cards_unop/Two_of_hearts.svg" | "/cards_unop/Two_of_spades.svg" | "/robots.txt";