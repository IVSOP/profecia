<script lang="ts">
	import * as Avatar from '$lib/components/ui/avatar/index.js';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
	import { UserIcon, LogOutIcon, TrophyIcon } from '@lucide/svelte';

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
				<a href="/profile" class="flex w-full items-center gap-2">
					<UserIcon class="h-4 w-4" />
					Perfil
				</a>
			</DropdownMenu.Item>
			<DropdownMenu.Item>
				<a href="/leaderboard" class="flex w-full items-center gap-2">
					<TrophyIcon class="h-4 w-4" />
					Leaderboard
				</a>
			</DropdownMenu.Item>
			<DropdownMenu.Separator />
			<DropdownMenu.Item>
				<a href="/logout" class="flex w-full items-center gap-2" data-sveltekit-reload>
					<LogOutIcon class="h-4 w-4" />
					Sair
				</a>
			</DropdownMenu.Item>
		</DropdownMenu.Group>
	</DropdownMenu.Content>
</DropdownMenu.Root>
