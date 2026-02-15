import { invalidateAll } from '$app/navigation';

/**
 * Starts polling via `invalidateAll()` at the given interval.
 * Automatically cleans up when the component is destroyed.
 *
 * Usage: call `usePolling(10_000)` at the top level of a component's `<script>`.
 */
export function usePolling(intervalMs: number = 10_000) {
	$effect(() => {
		const id = setInterval(() => {
			invalidateAll();
		}, intervalMs);

		return () => clearInterval(id);
	});
}
