// Request types

export interface AuthRequest {
	username: string;
	password: string;
}

export interface CreateMarketRequest {
	displayName: string;
	imageUrl?: string;
	optionAName: string;
	optionBName: string;
	rules: string;
}

export interface CreateEventRequest {
	displayName: string;
	imageUrl?: string;
	markets: CreateMarketRequest[];
}

// Response types

export interface UserDto {
	id: string;
	username: string;
	isAdmin: boolean;
}

export interface AuthResponse {
	sessionId: string;
	user: UserDto;
}

export interface MeResponse {
	user: UserDto;
}

export type MarketOption = 'optionA' | 'optionB';

export interface MarketDto {
	id: string;
	displayName: string;
	imageUrl: string | null;
	optionAName: string;
	optionBName: string;
	rules: string;
	resolvedOption: MarketOption | null;
}

export interface EventDto {
	id: string;
	displayName: string;
	imageUrl: string | null;
	url: string;
	pubkey: string;
	markets: MarketDto[];
	pendingBuyOrders: number;
}

export interface InfoResponse {
	event: EventDto | null;
}

export interface ListResponse {
	events: EventDto[];
}

// Position types

export interface PositionDto {
	id: string;
	marketId: string;
	userId: string;
	option: MarketOption;
	shares: number;
	pricePerShare: number;
}

// Buy order types

export interface BuyOrderRequest {
	marketId: string;
	userId: string;
	shares: number;
	pricePerShare: number;
	option: MarketOption;
}

export interface BuyOrderDto {
	id: string;
	marketId: string;
	userId: string;
	shares: number;
	pricePerShare: number;
	option: MarketOption;
}

// Market percentages

export interface MarketPercentagesDto {
	optionAPercentage: number | null;
	optionBPercentage: number | null;
}

export interface EventPercentagesResponse {
	percentages: Record<string, MarketPercentagesDto>;
}

// Chart data

export interface MarketSnapshotPointDto {
	recordedAt: string;
	percentages: Record<string, number | null>;
}

export interface EventChartDto {
	points: MarketSnapshotPointDto[];
}

export interface AllPercentagesResponse {
	percentages: Record<string, EventPercentagesResponse>;
}

export interface BalanceResponse {
	balanceCents: number;
}

export interface AirdropStatusResponse {
	lastAirdrop: string | null;
	secondsUntilAvailable: number;
	available: boolean;
}

// Leaderboard

export interface LeaderboardEntry {
	userId: string;
	username: string;
	realizedProfitCents: number;
}

export interface LeaderboardResponse {
	entries: LeaderboardEntry[];
}
