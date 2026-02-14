<script lang="ts">
	import RegisterForm from './register-form.svelte';
	import type { PageProps } from './$types';

	let { form }: PageProps = $props();
</script>

<svelte:head>
	<title>Criar conta - Profecia</title>
</svelte:head>

<div class="flex min-h-[calc(100vh-57px)] w-full items-center justify-center px-4">
	<RegisterForm {form} />
</div>
