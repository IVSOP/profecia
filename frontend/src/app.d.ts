// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
import type { UserDto } from '$lib/types';

declare global {
	namespace App {
		// interface Error {}
		interface Locals {
			user: UserDto | null;
		}
		interface PageData {
			user: UserDto | null;
		}
		// interface PageState {}
		// interface Platform {}
	}
}

export {};
