import type { Handle } from '@sveltejs/kit';
import type { MeResponse } from '$lib/types';

export const handle: Handle = async ({ event, resolve }) => {
	event.locals.user = null;

	if (event.url.pathname.startsWith('/api')) {
		return resolve(event);
	}

	const response = await event.fetch('/api/user/me');
	if (response.ok) {
		const payload = (await response.json()) as MeResponse;
		event.locals.user = payload.user;
	}

	return resolve(event);
};
