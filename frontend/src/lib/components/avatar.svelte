<script lang="ts">
	import * as Avatar from '$lib/components/ui/avatar/index.js';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';

	let { username }: { username: string } = $props();
</script>

<DropdownMenu.Root>
	<DropdownMenu.Trigger class="focus:outline-none">
		<Avatar.Root class="cursor-pointer ring-1 ring-border transition hover:ring-2 hover:ring-ring">
			<Avatar.Fallback class="justify-center bg-muted text-muted-foreground">
				{username.slice(0, 2).toUpperCase()}
			</Avatar.Fallback>
		</Avatar.Root>
	</DropdownMenu.Trigger>
	<DropdownMenu.Content>
		<DropdownMenu.Group>
			<DropdownMenu.Label>{username}</DropdownMenu.Label>
			<DropdownMenu.Separator />
			<DropdownMenu.Item>
				<a href="/logout" class="w-full" data-sveltekit-reload>Sair</a>
			</DropdownMenu.Item>
		</DropdownMenu.Group>
	</DropdownMenu.Content>
</DropdownMenu.Root>
