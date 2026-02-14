// Request types

export interface AuthRequest {
	username: string;
	password: string;
}

export interface CreateMarketRequest {
	display_name: string;
	option_a_name: string;
	option_b_name: string;
	rules: string;
}

export interface CreateEventRequest {
	id: string;
	display_name: string;
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
	display_name: string;
	option_a_name: string;
	option_b_name: string;
	rules: string;
	resolved_option: MarketOption | null;
}

export interface EventDto {
	id: string;
	display_name: string;
	markets: MarketDto[];
}

export interface InfoResponse {
	event: EventDto | null;
}

export interface ListResponse {
	events: EventDto[];
}
