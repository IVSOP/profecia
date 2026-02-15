<script lang="ts">
	import { Toaster as SonnerToaster, type ToasterProps } from 'svelte-sonner';
	import { mode } from 'mode-watcher';

	let props: ToasterProps = $props();

	const theme = $derived(mode.current === 'dark' ? 'dark' : 'light');
</script>

<SonnerToaster
	{theme}
	class="toaster group"
	toastOptions={{
		classes: {
			actionButton: 'group-[.toaster]:!bg-primary group-[.toaster]:!text-primary-foreground'
		}
	}}
	{...props}
/>
