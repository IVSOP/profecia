import { error } from '@sveltejs/kit';
import type { BuyOrderDto, EventChartDto, EventDto, EventPercentagesResponse, InfoResponse, MarketPercentagesDto, PositionDto } from '$lib/types';

export type EventPageData = {
	event: EventDto;
	positions: PositionDto[];
	buyOrders: BuyOrderDto[];
	allMarketOrders: Record<string, BuyOrderDto[]>;
	marketPercentages: Record<string, MarketPercentagesDto>;
	chartData: EventChartDto;
};

/**
 * Fetches all data for a single event page.
 * Reusable from the load function and for manual refetches.
 */
export async function fetchEventData(
	fetcher: typeof fetch,
	eventId: string,
	userId?: string
): Promise<EventPageData> {
	const response = await fetcher(`/api/event/${eventId}`);

	if (!response.ok) {
		error(404, 'Evento não encontrado');
	}

	const payload = (await response.json()) as InfoResponse;

	if (!payload.event) {
		error(404, 'Evento não encontrado');
	}

	let positions: PositionDto[] = [];
	let buyOrders: BuyOrderDto[] = [];

	// Fetch percentages for all markets in this event
	let marketPercentages: Record<string, MarketPercentagesDto> = {};
	const pctResponse = await fetcher(`/api/event/percentages/${eventId}`);
	if (pctResponse.ok) {
		const pctPayload = (await pctResponse.json()) as EventPercentagesResponse;
		marketPercentages = pctPayload.percentages;
	}

	// Fetch chart data (percentage history)
	let chartData: EventChartDto = { points: [] };
	const chartResponse = await fetcher(`/api/event/chart/${eventId}`);
	if (chartResponse.ok) {
		chartData = (await chartResponse.json()) as EventChartDto;
	}

	// Fetch ALL buy orders for each market (for the order book)
	const allOrderPromises = payload.event.markets.map(async (market) => {
		const res = await fetcher(`/api/event/buyorder/${market.id}`);
		if (!res.ok) return { marketId: market.id, orders: [] as BuyOrderDto[] };
		const orders = (await res.json()) as BuyOrderDto[];
		return { marketId: market.id, orders };
	});
	const allOrderResults = await Promise.all(allOrderPromises);
	const allMarketOrders: Record<string, BuyOrderDto[]> = {};
	for (const { marketId, orders } of allOrderResults) {
		allMarketOrders[marketId] = orders;
	}

	if (userId) {
		const posResponse = await fetcher(`/api/event/position/${eventId}`);
		if (posResponse.ok) {
			positions = (await posResponse.json()) as PositionDto[];
		}

		// Filter user's buy orders from the already-fetched data
		buyOrders = Object.values(allMarketOrders)
			.flat()
			.filter((o) => o.userId === userId);
	}

	return { event: payload.event, positions, buyOrders, allMarketOrders, marketPercentages, chartData };
}
