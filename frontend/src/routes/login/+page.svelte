<script lang="ts">
	import LoginForm from './login-form.svelte';
	import type { PageProps } from './$types';

	let { form }: PageProps = $props();
</script>

<svelte:head>
	<title>Entrar - Profecia</title>
</svelte:head>

<div class="flex min-h-[calc(100vh-57px)] w-full items-center justify-center px-4">
	<LoginForm {form} />
</div>
