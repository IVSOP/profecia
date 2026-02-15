import type { AllPercentagesResponse, EventDto, ListResponse, MarketPercentagesDto } from '$lib/types';

export type HomePageData = {
	events: EventDto[];
	allPercentages: Record<string, Record<string, MarketPercentagesDto>>;
};

/**
 * Fetches events and their percentages.
 * Reusable from the load function and for manual refetches.
 */
export async function fetchHomeData(
	fetcher: typeof fetch = fetch
): Promise<HomePageData> {
	const [eventsRes, pctRes] = await Promise.all([
		fetcher('/api/event'),
		fetcher('/api/event/percentages')
	]);

	if (!eventsRes.ok) {
		return { events: [], allPercentages: {} };
	}

	const payload = (await eventsRes.json()) as ListResponse;

	let allPercentages: Record<string, Record<string, MarketPercentagesDto>> = {};
	if (pctRes.ok) {
		const response = (await pctRes.json()) as AllPercentagesResponse;
		for (const [eventId, eventPct] of Object.entries(response.percentages)) {
			allPercentages[eventId] = eventPct.percentages;
		}
	}

	return { events: payload.events, allPercentages };
}
