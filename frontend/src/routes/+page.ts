import type { AllPercentagesResponse, EventDto, ListResponse, MarketPercentagesDto } from '$lib/types';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch }) => {
	try {
		const [eventsRes, pctRes] = await Promise.all([
			fetch('/api/event'),
			fetch('/api/event/percentages')
		]);

		if (!eventsRes.ok) {
			return { events: [] as EventDto[], allPercentages: {} as Record<string, Record<string, MarketPercentagesDto>> };
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
	} catch {
		return { events: [] as EventDto[], allPercentages: {} as Record<string, Record<string, MarketPercentagesDto>> };
	}
};
