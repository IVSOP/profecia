import type { EventDto, InfoResponse } from '$lib/types';
import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch, params }) => {
	const response = await fetch(`/api/event/${params.id}`);

	if (!response.ok) {
		error(404, 'Evento não encontrado');
	}

	const payload = (await response.json()) as InfoResponse;

	if (!payload.event) {
		error(404, 'Evento não encontrado');
	}

	return { event: payload.event };
};
