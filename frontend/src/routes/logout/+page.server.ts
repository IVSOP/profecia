import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ cookies, fetch }) => {
	await fetch('/api/user/logout', { method: 'POST' }).catch(() => {});

	cookies.delete('sessionId', { path: '/' });

	redirect(303, '/');
};
