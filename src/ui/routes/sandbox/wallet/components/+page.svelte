<script lang="ts">
  import CashOutAmount from "$components/wallet/CashOutAmount.svelte";
  import AddressToInput from "$components/wallet/AddressToInput.svelte";
  import CashBalanceContainer from "$components/wallet/CashBalanceContainer.svelte";
  import FundWalletWarning from "$components/wallet/FundWalletWarning.svelte";
  import ToggleSwitch from "$components/wallet/ToggleSwitch.svelte";
  import Transaction from "$components/wallet/Transaction.svelte";
  import Banner from "$components/wallet/Banner.svelte";
  import TransactionHistoryButton from "$components/wallet/TransactionHistoryButton.svelte";
  import CoinAddressContainer from "$components/wallet/CoinAddressContainer.svelte";

  // Demo handlers
  const handleOkClick = () => console.log("OK clicked");
  const handleFundWalletClick = () => console.log("Fund Wallet clicked");
  const handleCopy = (coin: string) => console.log(`Copied ${coin} address`);
</script>

<main class="bg-deep-charcoal min-h-screen py-12 px-6">
  <div class="max-w-6xl mx-auto">
    <!-- Header -->
    <div class="text-center mb-12">
      <h1 class="text-5xl font-bold text-white mb-3">Wallet Components</h1>
      <p class="text-silver-mist text-lg">
        Visual guide & demo for all wallet UI components
      </p>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
      <!-- CashOutAmount -->
      <section class="wallet-card">
        <h2 class="text-2xl font-semibold mb-2">CashOutAmount</h2>
        <p class="text-silver-mist mb-4">Amount input with sanitization</p>
        <CashOutAmount coinType="ckUSDC" amount={0} />
      </section>

      <!-- AddressToInput -->
      <section class="wallet-card">
        <h2 class="text-2xl font-semibold mb-2">AddressToInput</h2>
        <p class="text-silver-mist mb-4">Address input with icon</p>
        <AddressToInput
          placeholder="Enter address"
          icon="bank"
          width="w-full"
        />
      </section>

      <!-- CashBalanceContainer -->
      <section class="wallet-card">
        <h2 class="text-2xl font-semibold mb-2">CashBalanceContainer</h2>
        <p class="text-silver-mist mb-4">Balance display</p>
        <CashBalanceContainer
          amount="100"
          currency="ckUSDC"
          promptLabel="Add Cash To"
        />
      </section>

      <!-- FundWalletWarning -->
      <section class="wallet-card">
        <h2 class="text-2xl font-semibold mb-2">FundWalletWarning</h2>
        <p class="text-silver-mist mb-4">Warning modal</p>
        <FundWalletWarning
          onOkClick={handleOkClick}
          onFundWalletClick={handleFundWalletClick}
        />
      </section>

      <!-- ToggleSwitch -->
      <section class="wallet-card">
        <h2 class="text-2xl font-semibold mb-2">ToggleSwitch</h2>
        <p class="text-silver-mist mb-4">Deposit / Cash-out toggle</p>
        <ToggleSwitch mode="deposit" />
      </section>

      <!-- Transaction -->
      <section class="wallet-card">
        <h2 class="text-2xl font-semibold mb-2">Transaction</h2>
        <p class="text-silver-mist mb-4">Single transaction row</p>
        <div class="space-y-4">
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
        </div>
      </section>

      <!-- Banner -->
      <section class="wallet-card">
        <h2 class="text-2xl font-semibold mb-2">Banner</h2>
        <p class="text-silver-mist mb-4">Informational banner</p>
        <Banner message="ICP will be automatically exchanged for ckUSDC." />
      </section>

      <!-- TransactionHistoryButton -->
      <section class="wallet-card">
        <h2 class="text-2xl font-semibold mb-2">TransactionHistoryButton</h2>
        <p class="text-silver-mist mb-4">History button</p>
        <TransactionHistoryButton title="Transaction History" />
      </section>

      <!-- CoinAddressContainer -->
      <section class="wallet-card lg:col-span-2">
        <h2 class="text-2xl font-semibold mb-2">CoinAddressContainer</h2>
        <p class="text-silver-mist mb-4">Address with copy button</p>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <CoinAddressContainer
            coinType="ICP"
            address="87a4427b0ae47c3a92f0f2132a98f9a5d69d5ecefa97ad9ac3919766bbae85ac"
            onCopy={() => handleCopy("ICP")}
          />
          <CoinAddressContainer
            coinType="BTC"
            address="bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlhfe2wf"
            onCopy={() => handleCopy("BTC")}
          />
        </div>
      </section>
    </div>

    <!-- Footer note -->
    <p class="text-center text-silver-mist/70 mt-16 text-sm">
      This is a visual demo page. All components are fully functional.
    </p>
  </div>
</main>

<style lang="postcss">
  .wallet-card {
    @apply bg-dark-slate/50 p-6 rounded-2xl border border-silver-mist/30 text-left transition-all hover:border-silver-mist/50;
  }
</style>
