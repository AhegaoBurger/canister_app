<script>
  import LinkedInIcon from "../icons/linkedin.svelte";
  import XIcon from "../icons/x-logo.svelte";
  import Badge from "../icons/on-chain.svelte";
</script>

<div class="bg-background min-h-screen flex flex-col items-center px-6">
  <!-- Main Content -->
  <div class="flex flex-col items-center text-center mt-8 max-w-3xl">
    <h1 class="text-4xl font-bold">Cloud Storage Fully On-Chain</h1>
    <h2 class="text-lg mt-2 text-gray-400">
      Sovereign data management designed for privacy, secured on the blockchain.
    </h2>
  </div>

  <!-- Feature Boxes -->
  <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mt-16 max-w-5xl">
    <div class="glow-box">
      <h3 class="text-xl font-semibold">Sovereign</h3>
      <p class="text-gray-300 mt-2">
        Your Canister - fully owned and controlled by you.
      </p>
    </div>

    <div class="glow-box">
      <h3 class="text-xl font-semibold">Tamper-proof</h3>
      <p class="text-gray-300 mt-2">
        Encryption + blockchain = tamper-proof privacy.
      </p>
    </div>

    <div class="glow-box">
      <h3 class="text-xl font-semibold">Unstoppable</h3>
      <p class="text-gray-300 mt-2">
        Permanent, decentralized storage - always available.
      </p>
    </div>
  </div>

  <!-- New CTA Section -->
  <div class="flex flex-col md:flex-row gap-6 mt-12">
    <a href="/personal" class="btn btn-cta"> Personal Canisters </a>
    <a href="/enterprise" class="btn btn-cta"> Enterprise Canisters </a>
  </div>

  <!-- Demo Notice Section -->
  <div class="mt-12 text-center text-gray-400">
    <h2 class="text-lg">
      This is a Demo version of our MVP, do not upload confidential data.<br />
      Follow us for updates and news on our upcoming launch!
    </h2>
  </div>

  <div class="mt-6 text-center">
    <div class="flex gap-4">
      <a
        href="https://www.linkedin.com/company/canister-cloud/"
        target="_blank"
      >
        <span class="h-6 w-6">
          <LinkedInIcon />
        </span>
      </a>
      <a href="https://x.com/CanisterCloud" target="_blank">
        <span class="h-6 w-6">
          <XIcon />
        </span>
      </a>
    </div>
  </div>
  <div class="mt-6 text-center">
    <a href="https://internetcomputer.org/" target="_blank">
      <Badge />
    </a>
  </div>
</div>

<style>
  .glow-box {
    background: rgba(255, 255, 255, 0.05);
    padding: 20px;
    border-radius: 10px;
    text-align: center;
    max-width: 250px;
    box-shadow: 0 0 15px rgba(0, 162, 255, 0.4);
    transition: box-shadow 0.3s ease-in-out;
  }

  .glow-box:hover {
    box-shadow: 0 0 15px rgba(0, 162, 255, 0.8);
  }

  /* .btn-primary {
    background-color: #007bff;
    color: white;
    font-weight: bold;
    text-align: center;
    transition: background-color 0.3s ease;
  }

  .btn-primary:hover {
    background-color: #0056b3;
  }

  .btn-secondary {
    background-color: #28a745;
    color: white;
    font-weight: bold;
    text-align: center;
    transition: background-color 0.3s ease;
  }

  .btn-secondary:hover {
    background-color: #1e7e34;
  } */

  .fixed-social-icons {
    position: fixed;
    bottom: 15px;
    left: 50%;
    transform: translateX(-50%);
    display: flex;
    gap: 10px;
  }
</style>

