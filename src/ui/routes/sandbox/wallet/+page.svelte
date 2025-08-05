<script>
	import CashOutAmount from '$components/wallet/CashOutAmount.svelte';
	import AddressToInput from '$components/wallet/AddressToInput.svelte';
	import CashBalanceContainer from '$components/wallet/CashBalanceContainer.svelte';
	import FundWalletWarning from '$components/wallet/FundWalletWarning.svelte';
	import ToggleSwitch from '$components/wallet/ToggleSwitch.svelte';
	import Transaction from '$components/wallet/Transaction.svelte';
	import Banner from '$components/wallet/Banner.svelte';
	import TransactionHistoryButton from '$components/wallet/TransactionHistoryButton.svelte';
	import CoinAddressContainer from '$components/wallet/CoinAddressContainer.svelte';
</script>

<main
	class="bg-deep-charcoal flex items-center justify-center bg-gradient-to-r from-deep-charcoal to-dark-slate overflow-y-auto"
>
	<div class="flex flex-col items-center gap-12 text-center text-white max-w-4xl p-8">
		<h1 class="text-4xl font-bold mb-6">Poro Wallet Component Visual Guide</h1>
		<p class="text-lg text-silver-mist mb-8">
			Explore UI components for wallet features, including deposits, cash-outs, transactions,
			and warnings in the Poro platform.
		</p>

		<section class="w-full flex flex-col gap-6 border-t border-silver-mist/30 pt-6">
			<h2 class="text-3xl font-semibold">CashOutAmount</h2>
			<p class="text-sm text-silver-mist mb-4">
				Component for inputting cash-out amounts with sanitization.
			</p>
			<CashOutAmount coinType="ckUSDC" amount={0} />
		</section>

		<section class="w-full flex flex-col gap-6 border-t border-silver-mist/30 pt-6">
			<h2 class="text-3xl font-semibold">AddressToInput</h2>
			<p class="text-sm text-silver-mist mb-4">
				Input field for entering transfer addresses with icon.
			</p>
			<AddressToInput placeholder="Enter address" icon="bank" width="w-full" />
		</section>

		<section class="w-full flex flex-col gap-6 border-t border-silver-mist/30 pt-6">
			<h2 class="text-3xl font-semibold">CashBalanceContainer</h2>
			<p class="text-sm text-silver-mist mb-4">
				Displays cash balance with currency details.
			</p>
			<CashBalanceContainer amount="100" currency="ckUSDC" promptLabel="Add Cash To" />
		</section>

		<section class="w-full flex flex-col gap-6 border-t border-silver-mist/30 pt-6">
			<h2 class="text-3xl font-semibold">FundWalletWarning</h2>
			<p class="text-sm text-silver-mist mb-4">
				Warning modal for insufficient wallet funds.
			</p>
			<FundWalletWarning
				onOkClick={() => console.log('OK clicked')}
				onFundWalletClick={() => console.log('Fund Wallet clicked')}
			/>
		</section>

		<section class="w-full flex flex-col gap-6 border-t border-silver-mist/30 pt-6">
			<h2 class="text-3xl font-semibold">ToggleSwitch</h2>
			<p class="text-sm text-silver-mist mb-4">Toggle between deposit and cash-out modes.</p>
			<ToggleSwitch mode="deposit" />
		</section>

		<section class="w-full flex flex-col gap-6 border-t border-silver-mist/30 pt-6">
			<h2 class="text-3xl font-semibold">Transaction</h2>
			<p class="text-sm text-silver-mist mb-4">Single transaction row for history lists.</p>
			<Transaction
				status="Deposited"
				description="Converted to Dollar (ckUSDC)"
				date="Aug 5, 2025"
				fiatAmount="$100"
				cryptoAmount="50 ICP"
			/>
			<Transaction
				status="Withdrawn"
				description="To bank account"
				date="Aug 4, 2025"
				fiatAmount="$50"
				cryptoAmount="25 ICP"
				cashOutAmount="$50"
			/>
		</section>

		<section class="w-full flex flex-col gap-6 border-t border-silver-mist/30 pt-6">
			<h2 class="text-3xl font-semibold">Banner</h2>
			<p class="text-sm text-silver-mist mb-4">
				Informational banner for messages like exchange notes.
			</p>
			<Banner message="ICP will be automatically exchanged for ckUSDC." />
		</section>

		<section class="w-full flex flex-col gap-6 border-t border-silver-mist/30 pt-6">
			<h2 class="text-3xl font-semibold">TransactionHistoryButton</h2>
			<p class="text-sm text-silver-mist mb-4">Button to view transaction history.</p>
			<TransactionHistoryButton title="Transaction History" />
		</section>

		<section class="w-full flex flex-col gap-6 border-t border-silver-mist/30 pt-6">
			<h2 class="text-3xl font-semibold">CoinAddressContainer</h2>
			<p class="text-sm text-silver-mist mb-4">Displays coin address with copy button.</p>
			<CoinAddressContainer
				coinType="ICP"
				address="87a4427b0ae47c3a92f0f2132a98f9a5d69d5ecefa97ad9ac3919766bbae85ac"
				onCopy={() => console.log('Copied!')}
			/>
			<CoinAddressContainer
				coinType="BTC"
				address="bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlhfe2wf"
				onCopy={() => console.log('Copied!')}
			/>
		</section>
	</div>
</main>
