import { fail, redirect } from '@sveltejs/kit';
import type { Actions } from './$types';
import type { AuthResponse } from '$lib/types';

export const actions = {
	default: async ({ request, cookies, fetch }) => {
		const formData = await request.formData();
		const username = formData.get('username')?.toString() ?? '';
		const password = formData.get('password')?.toString() ?? '';
		const confirmPassword = formData.get('confirmPassword')?.toString() ?? '';

		if (!username || !password || !confirmPassword) {
			return fail(400, { username, error: 'Preencha todos os campos.' });
		}

		if (password !== confirmPassword) {
			return fail(400, { username, error: 'As passwords não coincidem.' });
		}

		const response = await fetch(`/api/user/register`, {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({ username, password })
		});

		if (!response.ok) {
			const data = await response.json().catch(() => null);
			return fail(400, {
				username,
				error: data?.error || 'Não foi possível criar a conta. Tente novamente.'
			});
		}

		const data = (await response.json()) as AuthResponse;

		cookies.set('sessionId', data.sessionId, {
			path: '/',
			maxAge: 60 * 60 * 24 * 7,
			sameSite: 'lax',
			httpOnly: false
		});

		redirect(303, '/');
	}
} satisfies Actions;
