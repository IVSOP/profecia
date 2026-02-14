import type { EventDto, ListResponse } from '$lib/types';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch }) => {
	try {
		const response = await fetch('/api/event');

		if (!response.ok) {
			return { events: [] as EventDto[] };
		}

		const payload = (await response.json()) as ListResponse;
		return { events: payload.events };
	} catch {
		return { events: [] as EventDto[] };
	}
};
