<script>
  import CashOutAmount from "$components/wallet/CashOutAmount.svelte";
  import AddressToInput from "$components/wallet/AddressToInput.svelte";
  import CashBalanceContainer from "$components/wallet/CashBalanceContainer.svelte";
  import FundWalletWarning from "$components/wallet/FundWalletWarning.svelte";
  import ToggleSwitch from "$components/wallet/ToggleSwitch.svelte";
  import Transaction from "$components/wallet/Transaction.svelte";
  import Banner from "$components/wallet/Banner.svelte";
  import TransactionHistoryButton from "$components/wallet/TransactionHistoryButton.svelte";
  import CoinAddressContainer from "$components/wallet/CoinAddressContainer.svelte";
</script>

<main
  class="bg-deep-charcoal flex items-center justify-center bg-gradient-to-r from-deep-charcoal to-dark-slate overflow-y-auto"
>
  <div class="flex flex-col items-center gap-8 text-white max-w-6xl p-8 w-full">
    <h1 class="text-4xl font-bold mb-6 text-center">
      Wallet Components Visual Guide
    </h1>
    <div class="grid grid-cols-1 gap-8 w-full">
      <section
        class="flex flex-col gap-4 bg-dark-slate/50 p-6 rounded-lg border border-silver-mist/30 text-left"
      >
        <h2 class="text-2xl font-semibold">CashOutAmount</h2>
        <p class="text-sm text-silver-mist mb-2">
          Component for inputting cash-out amounts with sanitization.
        </p>
        <CashOutAmount coinType="ckUSDC" amount={0} />
      </section>
      <section
        class="flex flex-col gap-4 bg-dark-slate/50 p-6 rounded-lg border border-silver-mist/30 text-left"
      >
        <h2 class="text-2xl font-semibold">AddressToInput</h2>
        <p class="text-sm text-silver-mist mb-2">
          Input field for entering transfer addresses with icon.
        </p>
        <AddressToInput
          placeholder="Enter address"
          icon="bank"
          width="w-full"
        />
      </section>
      <section
        class="flex flex-col gap-4 bg-dark-slate/50 p-6 rounded-lg border border-silver-mist/30 text-left"
      >
        <h2 class="text-2xl font-semibold">CashBalanceContainer</h2>
        <p class="text-sm text-silver-mist mb-2">
          Displays cash balance with currency details.
        </p>
        <CashBalanceContainer
          amount="100"
          currency="ckUSDC"
          promptLabel="Add Cash To"
        />
      </section>
      <section
        class="flex flex-col gap-4 bg-dark-slate/50 p-6 rounded-lg border border-silver-mist/30 text-left"
      >
        <h2 class="text-2xl font-semibold">FundWalletWarning</h2>
        <p class="text-sm text-silver-mist mb-2">
          Warning modal for insufficient wallet funds.
        </p>
        <FundWalletWarning
          onOkClick={() => console.log("OK clicked")}
          onFundWalletClick={() => console.log("Fund Wallet clicked")}
        />
      </section>
      <section
        class="flex flex-col gap-4 bg-dark-slate/50 p-6 rounded-lg border border-silver-mist/30 text-left"
      >
        <h2 class="text-2xl font-semibold">ToggleSwitch</h2>
        <p class="text-sm text-silver-mist mb-2">
          Toggle between deposit and cash-out modes.
        </p>
        <ToggleSwitch mode="deposit" />
      </section>
      <section
        class="flex flex-col gap-4 bg-dark-slate/50 p-6 rounded-lg border border-silver-mist/30 text-left"
      >
        <h2 class="text-2xl font-semibold">Transaction</h2>
        <p class="text-sm text-silver-mist mb-2">
          Single transaction row for history lists.
        </p>
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
      <section
        class="flex flex-col gap-4 bg-dark-slate/50 p-6 rounded-lg border border-silver-mist/30 text-left"
      >
        <h2 class="text-2xl font-semibold">Banner</h2>
        <p class="text-sm text-silver-mist mb-2">
          Informational banner for messages like exchange notes.
        </p>
        <Banner message="ICP will be automatically exchanged for ckUSDC." />
      </section>
      <section
        class="flex flex-col gap-4 bg-dark-slate/50 p-6 rounded-lg border border-silver-mist/30 text-left"
      >
        <h2 class="text-2xl font-semibold">TransactionHistoryButton</h2>
        <p class="text-sm text-silver-mist mb-2">
          Button to view transaction history.
        </p>
        <TransactionHistoryButton title="Transaction History" />
      </section>
      <section
        class="flex flex-col gap-4 bg-dark-slate/50 p-6 rounded-lg border border-silver-mist/30 text-left"
      >
        <h2 class="text-2xl font-semibold">CoinAddressContainer</h2>
        <p class="text-sm text-silver-mist mb-2">
          Displays coin address with copy button.
        </p>
        <div class="grid grid-cols-1 gap-4">
          <CoinAddressContainer
            coinType="ICP"
            address="87a4427b0ae47c3a92f0f2132a98f9a5d69d5ecefa97ad9ac3919766bbae85ac"
            onCopy={() => console.log("Copied!")}
          />
          <CoinAddressContainer
            coinType="BTC"
            address="bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlhfe2wf"
            onCopy={() => console.log("Copied!")}
          />
        </div>
      </section>
    </div>
  </div>
</main>
