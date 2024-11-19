<script>
	import { Icon, MainToolBar } from 'poro-components-ui';

	import { Carta, MarkdownEditor } from 'carta-md';
	import '../carta_default.css';

	const carta = new Carta({
		theme: 'dracula'
	});

	let value = '';

	export let has_chats = true;
</script>

<div class="grid_layout">
	<div class="read_main_layout">
		{#if has_chats === true}
			<Icon
				name="poro_logo"
				class="cursor_default"
				size="12rem"
				scale="1"
				viewSize={{
					width: 221,
					height: 59
				}}
			/>
		{/if}
	</div>
	<div class="write_main_layout">
		<div class="toolbar">
			<MainToolBar />
		</div>
		<div class="editor_root">
			<div class="markdown">
				<MarkdownEditor bind:value mode="split" {carta} disableToolbar={true} placeholder="#" />
			</div>
		</div>
	</div>
</div>

<!-- Mobile Not Supported -->
<div class="grid lg:hidden h-screen place-items-center text-white text-4xl">
	<h1>Sorry, Mobile Not Supported</h1>
</div>

<style lang="postcss">
	:global(.carta-font-code) {
		font-family: '...', monospace;
		font-size: 1.1rem;
	}

	.grid_layout {
		@apply hidden lg:grid grid-cols-12 gap-y-2 place-items-center h-screen fixed inset-0;
	}
	.read_main_layout {
		@apply col-start-1 col-end-7 row-start-1 row-end-auto flex justify-center items-center bg-black-a w-full h-full;
	}
	.write_main_layout {
		@apply col-start-7 col-end-13 row-start-1 row-end-auto justify-between flex flex-col bg-black-b w-full h-full;
	}
	.toolbar {
		@apply w-full h-16 bg-black-b z-10 flex items-center justify-end p-10;
	}
	.editor_root {
		@apply flex-grow p-4;
	}
	.markdown {
		@apply overflow-auto w-full h-full fixed mb-10 z-10;
	}
</style>
