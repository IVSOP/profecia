<script lang="ts">
	import { enhance } from '$app/forms';
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import { Input } from '$lib/components/ui/input';
	import { FieldGroup, Field, FieldLabel, FieldDescription } from '$lib/components/ui/field';
	import type { ActionData } from './$types';

	interface Props {
		form: ActionData;
	}

	let { form }: Props = $props();
	let loading = $state(false);
</script>

<Card.Root class="mx-auto w-full max-w-sm">
	<Card.Header>
		<Card.Title class="text-2xl">Criar conta</Card.Title>
		<Card.Description>Preencha os campos abaixo para criar a sua conta</Card.Description>
	</Card.Header>
	<Card.Content>
		<form
			method="POST"
			use:enhance={() => {
				loading = true;
				return async ({ update }) => {
					loading = false;
					await update();
				};
			}}
		>
			<FieldGroup>
				{#if form?.error}
					<div
						class="rounded-md border border-destructive/50 bg-destructive/10 px-3 py-2 text-sm text-destructive"
					>
						{form.error}
					</div>
				{/if}
				<Field>
					<FieldLabel for="username">Nome de utilizador</FieldLabel>
					<Input
						id="username"
						name="username"
						type="text"
						placeholder="utilizador"
						required
						minlength={3}
						maxlength={32}
						value={form?.username ?? ''}
						disabled={loading}
					/>
				</Field>
				<Field>
					<FieldLabel for="password">Password</FieldLabel>
					<Input
						id="password"
						name="password"
						type="password"
						required
						minlength={5}
						maxlength={128}
						disabled={loading}
					/>
				</Field>
				<Field>
					<FieldLabel for="confirm-password">Confirmar password</FieldLabel>
					<Input
						id="confirm-password"
						name="confirmPassword"
						type="password"
						required
						minlength={5}
						maxlength={128}
						disabled={loading}
					/>
				</Field>
				<Field>
					<Button type="submit" class="w-full" disabled={loading}>
						{#if loading}
							A criar conta...
						{:else}
							Criar conta
						{/if}
					</Button>
					<FieldDescription class="text-center">
						Já tem conta? <a href="/login" class="underline underline-offset-4">Entrar</a>
					</FieldDescription>
				</Field>
			</FieldGroup>
		</form>
	</Card.Content>
</Card.Root>
