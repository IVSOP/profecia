// Request types

export interface AuthRequest {
	username: string;
	password: string;
}

export interface CreateMarketRequest {
	displayName: string;
	optionAName: string;
	optionBName: string;
	rules: string;
}

export interface CreateEventRequest {
	displayName: string;
	markets: CreateMarketRequest[];
}

// Response types

export interface UserDto {
	id: string;
	username: string;
}

export interface AuthResponse {
	sessionId: string;
	user: UserDto;
}

export interface MeResponse {
	user: UserDto;
}

export type MarketOption = 'OptionA' | 'OptionB';

export interface MarketDto {
	id: string;
	displayName: string;
	optionAName: string;
	optionBName: string;
	rules: string;
	resolvedOption: MarketOption | null;
}

export interface EventDto {
	id: string;
	displayName: string;
	markets: MarketDto[];
}

export interface InfoResponse {
	event: EventDto | null;
}

export interface ListResponse {
	events: EventDto[];
}
