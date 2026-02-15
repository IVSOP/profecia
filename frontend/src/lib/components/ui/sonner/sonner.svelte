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
			toast: 'group-[.toaster]:!bg-background group-[.toaster]:!text-foreground group-[.toaster]:!border-border',
			closeButton: 'group-[.toaster]:!bg-background group-[.toaster]:!text-foreground group-[.toaster]:!border-border',
			actionButton: 'group-[.toaster]:!bg-primary group-[.toaster]:!text-primary-foreground'
		}
	}}
	{...props}
/>
