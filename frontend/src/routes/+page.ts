import { fetchHomeData } from '$lib/data/home';
import type { EventDto, MarketPercentagesDto } from '$lib/types';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch }) => {
	try {
		return await fetchHomeData(fetch);
	} catch {
		return { events: [] as EventDto[], allPercentages: {} as Record<string, Record<string, MarketPercentagesDto>> };
	}
};
