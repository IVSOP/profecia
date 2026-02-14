import { redirect } from '@sveltejs/kit';
import type { EventDto, ListResponse } from '$lib/types';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch, parent }) => {
	const { user } = await parent();

	if (!user?.isAdmin) {
		throw redirect(303, '/');
	}

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
