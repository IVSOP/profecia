<script lang="ts">
	interface Props {
		message: string;
	}

	let { message }: Props = $props();
</script>

{#if message}
	<div class="rounded-md border border-destructive/50 bg-destructive/10 px-3 py-2 text-sm text-destructive">
		{message}
	</div>
{/if}
